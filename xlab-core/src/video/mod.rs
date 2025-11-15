use ffmpeg_sys_next::*;
use std::ffi::{CStr, CString};
use std::path::PathBuf;
use std::ptr;
use xcap::image::RgbaImage;

use ffmpeg_sys_next::AVCodecID::AV_CODEC_ID_H264;
use ffmpeg_sys_next::AVPixelFormat::AV_PIX_FMT_RGBA;

pub struct EncoderConfig {
    pub pix_fmt: AVPixelFormat,
    pub preset: String,
    pub crf: i32,
    pub thread_count: i32,
}

impl Default for EncoderConfig {
    fn default() -> Self {
        Self {
            pix_fmt: AVPixelFormat::AV_PIX_FMT_YUV420P,
            preset: "ultrafast".to_string(),
            crf: 23,
            thread_count: 0, // 0 = auto-detect
        }
    }
}

pub struct VideoEncoder {
    fmt_ctx: *mut AVFormatContext,
    codec_ctx: *mut AVCodecContext,
    stream: *mut AVStream,
    sws_ctx: *mut SwsContext,
    frame: *mut AVFrame,
    packet: *mut AVPacket,
    time_base: AVRational,
    target_resolution: (u32, u32),
    source_resolution: (u32, u32),
    needs_resize: bool,
}

impl VideoEncoder {
    pub fn new(
        output_path: PathBuf,
        fps: u32,
        resolution: (u32, u32),
        image_dimensions: (u32, u32),
        config: EncoderConfig,
    ) -> Result<Self, String> {
        unsafe {
            let mut fmt_ctx = ptr::null_mut();
            let output_path_c = path_to_cstring(&output_path);

            // 1. Allocate format context
            if avformat_alloc_output_context2(
                &mut fmt_ctx,
                ptr::null(),
                ptr::null(),
                output_path_c.as_ptr(),
            ) < 0
            {
                return Err("Failed to create output context".into());
            }

            // 2. Open output file
            let mut avio_ctx = ptr::null_mut();
            if avio_open(&mut avio_ctx, output_path_c.as_ptr(), AVIO_FLAG_WRITE) < 0 {
                return Err("Failed to open output file".into());
            }
            (*fmt_ctx).pb = avio_ctx;

            // 3. Create stream
            let stream = avformat_new_stream(fmt_ctx, ptr::null());
            if stream.is_null() {
                return Err("Failed to create stream".into());
            }

            // 4. Initialize codec context
            let codec = avcodec_find_encoder(AV_CODEC_ID_H264);
            if codec.is_null() {
                return Err("H.264 encoder not found".into());
            }

            let codec_ctx = avcodec_alloc_context3(codec);
            if codec_ctx.is_null() {
                return Err("Failed to allocate codec context".into());
            }

            // 5. Configure codec parameters
            (*codec_ctx).codec_id = AV_CODEC_ID_H264;
            (*codec_ctx).width = resolution.0 as i32;
            (*codec_ctx).height = resolution.1 as i32;
            (*codec_ctx).time_base = AVRational {
                num: 1,
                den: fps as i32,
            };
            (*codec_ctx).pix_fmt = config.pix_fmt;
            (*codec_ctx).thread_count = config.thread_count;
            (*codec_ctx).thread_type = FF_THREAD_FRAME;

            // 6. Copy parameters to stream
            if avcodec_parameters_from_context((*stream).codecpar, codec_ctx) < 0 {
                return Err("Failed to copy codec parameters".into());
            }

            // 7. Set encoder options
            let crf = CString::new("crf").unwrap();
            let preset = CString::new(config.preset).unwrap();
            av_opt_set_int((*codec_ctx).priv_data, crf.as_ptr(), config.crf as i64, 0);
            av_opt_set(
                (*codec_ctx).priv_data,
                CStr::from_bytes_with_nul(b"preset\0").unwrap().as_ptr(),
                preset.as_ptr(),
                0,
            );

            // 8. Open codec
            if avcodec_open2(codec_ctx, codec, ptr::null_mut()) < 0 {
                return Err("Failed to open codec".into());
            }

            // 9. Create scaling context
            let sws_ctx = sws_getContext(
                image_dimensions.0 as i32,
                image_dimensions.1 as i32,
                AV_PIX_FMT_RGBA,
                resolution.0 as i32,
                resolution.1 as i32,
                config.pix_fmt,
                SWS_BILINEAR,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            );

            // 10. Allocate frame
            let frame = av_frame_alloc();
            (*frame).width = resolution.0 as i32;
            (*frame).height = resolution.1 as i32;
            (*frame).format = config.pix_fmt as i32;
            if av_frame_get_buffer(frame, 0) < 0 {
                return Err("Failed to allocate frame buffers".into());
            }

            // 11. Allocate packet
            let packet = av_packet_alloc();

            // 12. Write header
            if avformat_write_header(fmt_ctx, ptr::null_mut()) < 0 {
                return Err("Failed to write header".into());
            }

            let needs_resize = image_dimensions != resolution;
            
            Ok(Self {
                fmt_ctx,
                codec_ctx,
                stream,
                sws_ctx,
                frame,
                packet,
                time_base: (*codec_ctx).time_base,
                target_resolution: resolution,
                source_resolution: image_dimensions,
                needs_resize,
            })
        }
    }

    /// Appends an RGBA image to the video stream. If the image dimensions match the target
    /// resolution, only format conversion is performed. Otherwise, resizing is also applied.
    pub fn append_image(&mut self, image: RgbaImage, index: u64) -> Result<(), String> {
        let (width, height) = image.dimensions();
        let rgba_data = image.into_raw();
        self.add_frame(&rgba_data, width, height, index)?;
        Ok(())
    }

    fn add_frame(
        &mut self,
        rgba_data: &[u8],
        width: u32,
        height: u32,
        frame_index: u64,
    ) -> Result<(), String> {
        unsafe {
            if self.needs_resize {
                // Images need resizing - use sws_scale
                let src_slice = [rgba_data.as_ptr()];
                let src_stride = [(width * 4) as i32];

                let result = sws_scale(
                    self.sws_ctx,
                    src_slice.as_ptr(),
                    src_stride.as_ptr(),
                    0,
                    height as i32,
                    (*self.frame).data.as_ptr() as *mut *mut u8,
                    (*self.frame).linesize.as_ptr(),
                );

                if result < 0 {
                    return Err("Failed to scale image".into());
                }
            } else {
                // Images are already at target resolution - copy directly
                let src_slice = [rgba_data.as_ptr()];
                let src_stride = [(width * 4) as i32];

                let result = sws_scale(
                    self.sws_ctx,
                    src_slice.as_ptr(),
                    src_stride.as_ptr(),
                    0,
                    height as i32,
                    (*self.frame).data.as_ptr() as *mut *mut u8,
                    (*self.frame).linesize.as_ptr(),
                );

                if result < 0 {
                    return Err("Failed to convert image format".into());
                }
            }

            // Set frame properties
            (*self.frame).pts = frame_index as i64;

            // Send frame to encoder
            let send_result = avcodec_send_frame(self.codec_ctx, self.frame);
            if send_result < 0 {
                return Err("Failed to send frame to encoder".into());
            }

            // Process packets
            while avcodec_receive_packet(self.codec_ctx, self.packet) >= 0 {
                av_packet_rescale_ts(self.packet, self.time_base, (*self.stream).time_base);
                (*self.packet).stream_index = (*self.stream).index;

                let write_result = av_interleaved_write_frame(self.fmt_ctx, self.packet);
                if write_result < 0 {
                    return Err("Failed to write frame".into());
                }

                av_packet_unref(self.packet);
            }

            Ok(())
        }
    }

    pub fn finalize(self) -> Result<(), String> {
        unsafe {
            // Flush encoder
            avcodec_send_frame(self.codec_ctx, ptr::null_mut());
            while avcodec_receive_packet(self.codec_ctx, self.packet) >= 0 {
                av_packet_rescale_ts(self.packet, self.time_base, (*self.stream).time_base);
                (*self.packet).stream_index = (*self.stream).index;
                av_interleaved_write_frame(self.fmt_ctx, self.packet);
                av_packet_unref(self.packet);
            }

            av_write_trailer(self.fmt_ctx);
        }
        Ok(())
    }
}

impl Drop for VideoEncoder {
    fn drop(&mut self) {
        unsafe {
            // Free all allocated resources in reverse allocation order
            av_packet_free(&mut self.packet);
            av_frame_free(&mut self.frame);
            sws_freeContext(self.sws_ctx);
            avcodec_free_context(&mut self.codec_ctx);
            if !(*self.fmt_ctx).pb.is_null() {
                avio_closep(&mut (*self.fmt_ctx).pb);
            }
            avformat_free_context(self.fmt_ctx);
        }
    }
}

fn path_to_cstring(path: &PathBuf) -> CString {
    CString::new(path.to_str().unwrap()).expect("Invalid path")
}
