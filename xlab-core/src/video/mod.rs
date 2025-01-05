use ffmpeg_sys_next::*;
use std::ptr;
use std::{ffi::CString, path::PathBuf};

use ffmpeg_sys_next::AVCodecID::AV_CODEC_ID_H264;
use xcap::image::RgbaImage;
use yuvutils_rs::{
    rgba_to_yuv420, YuvChromaSubsampling, YuvPlanarImageMut, YuvRange, YuvStandardMatrix,
};

pub struct EncoderConfig {
    pub bitrate: i64,
    pub gop_size: i32,
    pub max_b_frames: i32,
    pub pix_fmt: AVPixelFormat,
}

impl Default for EncoderConfig {
    fn default() -> Self {
        Self {
            bitrate: 400_000,
            gop_size: 30,
            max_b_frames: 1,
            pix_fmt: AVPixelFormat::AV_PIX_FMT_YUV420P,
        }
    }
}

pub struct VideoEncoder {
    resolution: (u32, u32),
    fmt_ctx: *mut AVFormatContext,
    codec_ctx: *mut AVCodecContext,
    config: EncoderConfig,
    frame_index: i64,
}

impl VideoEncoder {
    pub fn initialize(
        output_path: PathBuf,
        frame_rate: u32,
        resolution: (u32, u32),
        config: EncoderConfig,
    ) -> Result<Self, String> {
        println!("output_path: {:?}", output_path);
        let output_path = pathbuf_to_cstring(&output_path);
        unsafe {
            avformat_network_init();

            // Allocate format context
            let mut fmt_ctx = avformat_alloc_context();
            let format_name = CString::new("mp4").unwrap();

            let output_format =
                av_guess_format(format_name.as_ptr(), output_path.as_ptr(), ptr::null());
            if output_format.is_null() {
                return Err("Failed to guess output format".to_string());
            }

            if avformat_alloc_output_context2(
                &mut fmt_ctx,
                output_format,
                format_name.as_ptr(),
                output_path.as_ptr(),
            ) < 0
                || fmt_ctx.is_null()
            {
                return Err("Failed to allocate format context".to_string());
            }

            // Find the H.264 codec
            let codec = avcodec_find_encoder(AV_CODEC_ID_H264);
            if codec.is_null() {
                return Err("H264 encoder not found".to_string());
            }

            // Create a new stream
            let stream = avformat_new_stream(fmt_ctx, codec);
            if stream.is_null() {
                return Err("Failed to create stream".to_string());
            }

            // Allocate and configure codec context
            let codec_ctx = avcodec_alloc_context3(codec);
            if codec_ctx.is_null() {
                return Err("Failed to allocate codec context".to_string());
            }

            (*codec_ctx).width = resolution.0 as i32;
            (*codec_ctx).height = resolution.1 as i32;
            (*codec_ctx).time_base = AVRational {
                num: 1,
                den: frame_rate as i32,
            };
            (*codec_ctx).bit_rate = config.bitrate;
            (*codec_ctx).gop_size = config.gop_size;
            (*codec_ctx).max_b_frames = config.max_b_frames;
            (*codec_ctx).pix_fmt = config.pix_fmt;

            // Enable global headers if required by the format
            if (*(*fmt_ctx).oformat).flags & AVFMT_GLOBALHEADER != 0 {
                (*codec_ctx).flags |= AV_CODEC_FLAG_GLOBAL_HEADER as i32;
            }

            // Open the codec
            if avcodec_open2(codec_ctx, codec, ptr::null_mut()) < 0 {
                return Err("Failed to open codec".to_string());
            }

            // Copy codec parameters to the stream
            (*stream).codecpar = avcodec_parameters_alloc();
            if (*stream).codecpar.is_null() {
                return Err("Failed to allocate codec parameters".to_string());
            }

            if avcodec_parameters_from_context((*stream).codecpar, codec_ctx) < 0 {
                return Err("Failed to copy codec parameters to stream".to_string());
            }

            // Set stream time base
            (*stream).time_base = (*codec_ctx).time_base;

            // Open the output file if required
            if (*(*fmt_ctx).oformat).flags & AVFMT_NOFILE == 0 {
                if avio_open(&mut (*fmt_ctx).pb, output_path.as_ptr(), AVIO_FLAG_WRITE) < 0 {
                    return Err("Failed to open output file".to_string());
                }
            }

            // Write the format header
            if avformat_write_header(fmt_ctx, ptr::null_mut()) < 0 {
                return Err("Failed to write format header".to_string());
            }

            Ok(Self {
                resolution,
                fmt_ctx,
                codec_ctx,
                config,
                frame_index: 0,
            })
        }
    }

    pub fn push_frame(&mut self, image_path: &PathBuf) -> Result<(), String> {
        print!("Pushing frame new frame; ");
        let img = xcap::image::open(image_path)
            .map_err(|_| "Failed to open image".to_string())?
            .to_rgba8();
        let yuv_pixels = yuv_conversion(&img);
        let (width, height) = (self.resolution.0 as usize, self.resolution.1 as usize);

        unsafe {
            let mut frame = av_frame_alloc();
            if frame.is_null() {
                return Err("Failed to allocate AVFrame".to_string());
            }

            (*frame).format = self.config.pix_fmt as i32;
            (*frame).width = width as i32;
            (*frame).height = height as i32;
            (*frame).pts = self.frame_index;

            // Allocate buffer for the frame
            if av_frame_get_buffer(frame, 32) < 0 {
                av_frame_free(&mut frame);
                return Err("Failed to allocate frame buffer".to_string());
            }

            // Copy YUV data to frame's buffers
            ptr::copy_nonoverlapping(
                yuv_pixels.y_plane.borrow().as_ptr(),
                (*frame).data[0],
                yuv_pixels.y_stride as usize * height,
            );
            ptr::copy_nonoverlapping(
                yuv_pixels.u_plane.borrow().as_ptr(),
                (*frame).data[1],
                yuv_pixels.u_stride as usize * (height / 2),
            );
            ptr::copy_nonoverlapping(
                yuv_pixels.v_plane.borrow().as_ptr(),
                (*frame).data[2],
                yuv_pixels.v_stride as usize * (height / 2),
            );

            // Send frame to encoder
            if avcodec_send_frame(self.codec_ctx, frame) < 0 {
                av_frame_free(&mut frame);
                return Err("Failed to send frame to encoder".to_string());
            }

            // Retrieve encoded packets and write them
            let mut packet = av_packet_alloc();
            if packet.is_null() {
                return Err("Failed to allocate AVPacket".to_string());
            }

            while avcodec_receive_packet(self.codec_ctx, packet) == 0 {
                // Write the encoded packet to the output file
                if av_interleaved_write_frame(self.fmt_ctx, packet) < 0 {
                    av_packet_free(&mut packet);
                    av_frame_free(&mut frame);
                    return Err("Failed to write packet".to_string());
                }
            }

            av_packet_free(&mut packet);
            av_frame_free(&mut frame);
        }

        println!("Frame {} pushed", self.frame_index);
        self.frame_index += 1;
        Ok(())
    }

    pub fn finalize(self) -> Result<(), String> {
        println!("Finalizing video");
        unsafe {
            // Check if format context is valid
            if self.fmt_ctx.is_null() {
                return Err("Format context is null".to_string());
            }

            // Write trailer
            let ret = av_write_trailer(self.fmt_ctx);
            if ret < 0 {
                return Err(format!("Failed to write trailer: {}", ret));
            }
        }
        Ok(())
    }
}

impl Drop for VideoEncoder {
    fn drop(&mut self) {
        unsafe {
            avcodec_free_context(&mut self.codec_ctx);
            avformat_free_context(self.fmt_ctx);
        }
    }
}

fn yuv_conversion(image: &RgbaImage) -> YuvPlanarImageMut<u8> {
    let mut yuv_image =
        YuvPlanarImageMut::alloc(image.width(), image.height(), YuvChromaSubsampling::Yuv420);

    rgba_to_yuv420(
        &mut yuv_image,           // Target YUV image
        image.as_raw(),           // RGBA input slice
        image.width() * 4,        // RGBA stride (4 bytes per pixel)
        YuvRange::Limited,        // Commonly used range
        YuvStandardMatrix::Bt709, // Common standard for HD videos
    )
    .expect("RGBA to YUV420 conversion failed");

    yuv_image
}

fn pathbuf_to_cstring(path: &PathBuf) -> CString {
    CString::new(path.to_string_lossy().as_bytes()).expect("Failed to convert PathBuf to CString")
}
