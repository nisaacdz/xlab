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
    pub max_b_frames: i32,
    pub pix_fmt: AVPixelFormat,
}

impl Default for EncoderConfig {
    fn default() -> Self {
        Self {
            max_b_frames: 1,
            pix_fmt: AVPixelFormat::AV_PIX_FMT_YUV420P,
        }
    }
}

pub struct VideoEncodingError {}

pub struct VideoEncoder {
    resolution: (u32, u32),
    fmt_ctx: *mut AVFormatContext,
    codec_ctx: *mut AVCodecContext,
    stream: *mut AVStream,
    image_index: i64,
    time_base: AVRational,
}

impl VideoEncoder {
    pub fn initialize(
        output_path: PathBuf,
        image_rate: u32,
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
            if avio_open(
                &mut av_io_context,
                output_path_cstr.as_ptr(),
                AVIO_FLAG_WRITE,
            ) < 0
            {
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

            // Allocate the codec context for the encoder
            let mut codec_ctx = avcodec_alloc_context3(codec);
            if codec_ctx.is_null() {
                avformat_free_context(fmt_ctx);
                return Err("Could not allocate video codec context".to_string());
            }

            // Set codec parameters
            (*codec_ctx).codec_id = AV_CODEC_ID_H264;
            (*codec_ctx).bit_rate = compute_bit_rate(resolution, image_rate);
            (*codec_ctx).width = resolution.0 as i32;
            (*codec_ctx).height = resolution.1 as i32;
            (*codec_ctx).time_base = AVRational {
                num: 1,
                den: image_rate as i32,
            };
            (*codec_ctx).gop_size = image_rate as i32;
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
                // debug print out a few variables to see what caused this
                println!("codec_id: {:?}", (*codec_ctx).codec_id);
                println!("bit_rate: {:?}", (*codec_ctx).bit_rate);
                println!("width: {:?}", (*codec_ctx).width);
                println!("height: {:?}", (*codec_ctx).height);
                println!("time_base: {:?}", (*codec_ctx).time_base);
                println!("gop_size: {:?}", (*codec_ctx).gop_size);
                println!("max_b_frames: {:?}", (*codec_ctx).max_b_frames);
                println!("pix_fmt: {:?}", (*codec_ctx).pix_fmt);
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
                image_index: 1,
                time_base: (*codec_ctx).time_base,
            })
        }
    }

    pub fn append_image(&mut self, image_path: &PathBuf) -> Result<(), String> {
        // Load the image
        let mut img = open(image_path)
            .map_err(|_| "Failed to open image".to_string())?
            .to_rgba8();

        if img.dimensions() != self.resolution {
            resize_image(&mut img, self.resolution);
        }

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
            (*frame).pts = self.image_index;

            // Allocate the buffers for the frame data
            if av_frame_get_buffer(frame, 0) < 0 {
                av_frame_free(&mut frame);
                return Err("Could not allocate the video frame data".to_string());
            }

            // Make sure the frame data is writable
            if av_frame_make_writable(frame) < 0 {
                av_frame_free(&mut frame);
                return Err("Frame data is not writable".to_string());
            }

            // Copy Y-plane data row by row
            let y_plane = yuv_image.y_plane.borrow();
            let y_stride = (*frame).linesize[0] as usize;
            let y_width = self.resolution.0 as usize;
            for i in 0..self.resolution.1 as usize {
                let src_offset = i * y_width;
                let dst_offset = i * y_stride;
                ptr::copy_nonoverlapping(
                    y_plane.as_ptr().add(src_offset),
                    (*frame).data[0].add(dst_offset),
                    y_width,
                );
            }

            // Copy U-plane data row by row
            let u_plane = yuv_image.u_plane.borrow();
            let u_stride = (*frame).linesize[1] as usize;
            let u_width = self.resolution.0 as usize / 2;
            let u_height = self.resolution.1 as usize / 2;
            for i in 0..u_height {
                let src_offset = i * u_width;
                let dst_offset = i * u_stride;
                ptr::copy_nonoverlapping(
                    u_plane.as_ptr().add(src_offset),
                    (*frame).data[1].add(dst_offset),
                    u_width,
                );
            }

            // Copy V-plane data row by row
            let v_plane = yuv_image.v_plane.borrow();
            let v_stride = (*frame).linesize[2] as usize;
            for i in 0..u_height {
                let src_offset = i * u_width;
                let dst_offset = i * v_stride;
                ptr::copy_nonoverlapping(
                    v_plane.as_ptr().add(src_offset),
                    (*frame).data[2].add(dst_offset),
                    u_width,
                );
            }

            (3..(*frame).data.len()).for_each(|i| (*frame).data[i] = std::ptr::null_mut());

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

                if av_interleaved_write_frame(self.fmt_ctx, packet) < 0 {
                    av_packet_free(&mut packet);
                    av_frame_free(&mut frame);
                    return Err("Error writing packet to output file".to_string());
                }
            }

            av_packet_free(&mut packet);
            av_frame_free(&mut frame);
        }

        self.image_index += 1;
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
        &mut yuv_image,    // Target YUV image
        image.as_raw(),    // RGBA input slice
        image.width() * 4, // RGBA stride (4 bytes per pixel)
        YuvRange::Limited, // Commonly used range
        YuvStandardMatrix::Bt709,
    )
    .expect("RGBA to YUV420 conversion failed");

    yuv_image
}

fn pathbuf_to_cstring(path: &PathBuf) -> CString {
    CString::new(path.to_string_lossy().as_bytes()).expect("Failed to convert PathBuf to CString")
}

fn compute_bit_rate((width, height): (u32, u32), frame_rate: u32) -> i64 {
    let resolution_factor = f64::powf(width as f64 * height as f64, 1.161);
    let frame_rate_factor = f64::powf(frame_rate as f64, 0.585);
    (resolution_factor * frame_rate_factor * 0.265) as i64
}

use fast_image_resize::images::Image;
use fast_image_resize::{PixelType, Resizer};

fn resize_image(img: &mut RgbaImage, (new_width, new_height): (u32, u32)) {
    // Convert RgbaImage to fast_image_resize::images::Image
    let (old_width, old_height) = img.dimensions();
    let buffer_mut = unsafe {
        std::slice::from_raw_parts_mut(
            img.as_mut_ptr(),
            old_width as usize * old_height as usize * 4,
        )
    };
    let old_img = Image::from_slice_u8(old_width, old_height, buffer_mut, PixelType::U8x4).unwrap();

    // Create a new image with the desired dimensions
    let mut new_img = Image::new(new_width, new_height, PixelType::U8x4);

    // Resize the image
    Resizer::new()
        .resize(&old_img, &mut new_img, None)
        .expect("Failed to resize image");

    // Replace the original image with the resized image
    *img = RgbaImage::from_vec(new_width, new_height, new_img.into_vec()).unwrap();
}
