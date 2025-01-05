use ffmpeg_sys_next::*;
use std::ffi::CString;
use std::path::PathBuf;
use std::ptr;

use ffmpeg_sys_next::AVCodecID::AV_CODEC_ID_H264;

use xcap::image::{open, RgbaImage};
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
            gop_size: 24,
            max_b_frames: 1,
            pix_fmt: AVPixelFormat::AV_PIX_FMT_YUV420P,
        }
    }
}

pub struct VideoEncoder {
    resolution: (u32, u32),
    fmt_ctx: *mut AVFormatContext,
    codec_ctx: *mut AVCodecContext,
    stream: *mut AVStream,
    frame_index: i64,
    time_base: AVRational,
}

impl VideoEncoder {
    pub fn initialize(
        output_path: PathBuf,
        frame_rate: u32,
        resolution: (u32, u32),
        config: EncoderConfig,
    ) -> Result<Self, String> {
        unsafe {
            // Allocate the output media context
            let mut fmt_ctx: *mut AVFormatContext = ptr::null_mut();
            let output_path_cstr = pathbuf_to_cstring(&output_path);
            let mut av_io_context: *mut AVIOContext = ptr::null_mut();

            if avformat_alloc_output_context2(
                &mut fmt_ctx,
                ptr::null_mut(),
                ptr::null(),
                output_path_cstr.as_ptr(),
            ) < 0
            {
                return Err("Could not deduce output format from file extension".to_string());
            }

            if fmt_ctx.is_null() {
                return Err("Could not allocate output format context".to_string());
            }

            // Open the output file
            if avio_open(&mut av_io_context, output_path_cstr.as_ptr(), AVIO_FLAG_WRITE) < 0 {
                avformat_free_context(fmt_ctx);
                return Err("Could not open output file".to_string());
            }
            (*fmt_ctx).pb = av_io_context;

            // Find the encoder
            let codec = avcodec_find_encoder(AV_CODEC_ID_H264);
            if codec.is_null() {
                avformat_free_context(fmt_ctx);
                return Err("Codec not found".to_string());
            }

            // Add a new stream to the output file
            let stream = avformat_new_stream(fmt_ctx, ptr::null());
            if stream.is_null() {
                avformat_free_context(fmt_ctx);
                return Err("Could not allocate stream".to_string());
            }
            (*stream).id = ((*fmt_ctx).nb_streams - 1) as i32;

            // Allocate the codec context for the encoder
            let mut codec_ctx = avcodec_alloc_context3(codec);
            if codec_ctx.is_null() {
                avformat_free_context(fmt_ctx);
                return Err("Could not allocate video codec context".to_string());
            }

            // Set codec parameters
            (*codec_ctx).codec_id = AV_CODEC_ID_H264;
            (*codec_ctx).bit_rate = config.bitrate;
            (*codec_ctx).width = resolution.0 as i32;
            (*codec_ctx).height = resolution.1 as i32;
            (*codec_ctx).time_base = AVRational {
                num: 1,
                den: frame_rate as i32,
            };
            (*codec_ctx).gop_size = config.gop_size;
            (*codec_ctx).max_b_frames = config.max_b_frames;
            (*codec_ctx).pix_fmt = config.pix_fmt;

            // Set global headers flag if necessary
            if (*(*fmt_ctx).oformat).flags & AVFMT_GLOBALHEADER != 0 {
                (*codec_ctx).flags |= AV_CODEC_FLAG_GLOBAL_HEADER as i32;
            }

            // Open the codec
            if avcodec_open2(codec_ctx, codec, ptr::null_mut()) < 0 {
                avcodec_free_context(&mut codec_ctx);
                avformat_free_context(fmt_ctx);
                return Err("Could not open codec".to_string());
            }

            // Copy the stream parameters to the muxer
            if avcodec_parameters_from_context((*stream).codecpar, codec_ctx) < 0 {
                avcodec_free_context(&mut codec_ctx);
                avformat_free_context(fmt_ctx);
                return Err("Could not copy codec parameters".to_string());
            }

            // Write the stream header
            if avformat_write_header(fmt_ctx, ptr::null_mut()) < 0 {
                avcodec_free_context(&mut codec_ctx);
                avformat_free_context(fmt_ctx);
                return Err("Error occurred when opening output file".to_string());
            }

            Ok(Self {
                resolution,
                fmt_ctx,
                codec_ctx,
                stream,
                frame_index: 0,
                time_base: (*codec_ctx).time_base,
            })
        }
    }

    pub fn append_image(&mut self, image_path: &PathBuf) -> Result<(), String> {
        // Load the image
        let img = open(image_path)
            .map_err(|_| "Failed to open image".to_string())?
            .to_rgba8();

        // Convert the image to YUV420 format
        let yuv_image = yuv_conversion(&img);

        unsafe {
            // Allocate video frame
            let mut frame = av_frame_alloc();
            if frame.is_null() {
                return Err("Could not allocate video frame".to_string());
            }
            (*frame).format = (*self.codec_ctx).pix_fmt as i32;
            (*frame).width = self.resolution.0 as i32;
            (*frame).height = self.resolution.1 as i32;
            (*frame).pts = self.frame_index;

            // Allocate the buffers for the frame data
            if av_frame_get_buffer(frame, 32) < 0 {
                av_frame_free(&mut frame);
                return Err("Could not allocate the video frame data".to_string());
            }

            // Make sure the frame data is writable
            if av_frame_make_writable(frame) < 0 {
                av_frame_free(&mut frame);
                return Err("Frame data is not writable".to_string());
            }

            // Copy the YUV data to the frame
            let y_size = (*frame).width as usize * (*frame).height as usize;
            let uv_size = y_size / 4;

            ptr::copy_nonoverlapping(
                yuv_image.y_plane.borrow().as_ptr(),
                (*frame).data[0],
                y_size,
            );
            ptr::copy_nonoverlapping(
                yuv_image.u_plane.borrow().as_ptr(),
                (*frame).data[1],
                uv_size,
            );
            ptr::copy_nonoverlapping(
                yuv_image.v_plane.borrow().as_ptr(),
                (*frame).data[2],
                uv_size,
            );

            // Send the frame to the encoder
            if avcodec_send_frame(self.codec_ctx, frame) < 0 {
                av_frame_free(&mut frame);
                return Err("Error sending frame to encoder".to_string());
            }

            // Allocate packet
            let mut packet = av_packet_alloc();
            if packet.is_null() {
                av_frame_free(&mut frame);
                return Err("Failed to allocate packet".to_string());
            }

            // Receive encoded packets and write them
            while avcodec_receive_packet(self.codec_ctx, packet) == 0 {
                av_packet_rescale_ts(packet, self.time_base, (*self.stream).time_base);
                (*packet).stream_index = (*self.stream).index;

                if av_interleaved_write_frame(self.fmt_ctx, packet) < 0 {
                    av_packet_free(&mut packet);
                    av_frame_free(&mut frame);
                    return Err("Error writing packet to output file".to_string());
                }
            }

            av_packet_free(&mut packet);
            av_frame_free(&mut frame);
        }

        self.frame_index += 1;
        Ok(())
    }

    pub fn finalize(&mut self) -> Result<(), String> {
        unsafe {
            // Flush the encoder
            if avcodec_send_frame(self.codec_ctx, ptr::null_mut()) == 0 {
                let mut packet = av_packet_alloc();
                while avcodec_receive_packet(self.codec_ctx, packet) == 0 {
                    av_packet_rescale_ts(packet, self.time_base, (*self.stream).time_base);
                    (*packet).stream_index = (*self.stream).index;
                    av_interleaved_write_frame(self.fmt_ctx, packet);
                }
                av_packet_free(&mut packet);
            }

            // Write trailer
            if av_write_trailer(self.fmt_ctx) < 0 {
                return Err("Error writing video trailer".to_string());
            }

            // Close the output file
            if (*(*self.fmt_ctx).oformat).flags & AVFMT_NOFILE == 0 {
                avio_close((*self.fmt_ctx).pb);
            }

            Ok(())
        }
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
