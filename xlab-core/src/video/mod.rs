mod ffmpeg_bindings;
use std::{path::PathBuf, ptr};
use ffmpeg_bindings::*;

// Error codes from FFmpeg
const AVERROR_EAGAIN: i32 = -11;
const AVERROR_EOF: i32 = -541478725;

pub struct VideoEncoder {
    format_context: *mut AVFormatContext,
    codec_context: *mut AVCodecContext,
    stream: *mut AVStream,
    frame: *mut AVFrame,
    pkt: *mut AVPacket,
    next_pts: i64,
}

impl VideoEncoder {
    pub fn initialize(
        output_path: PathBuf,
        frame_rate: u32,
        resolution: (u32, u32),
    ) -> Result<Self, String> {
        unsafe {
            let mut format_context: *mut AVFormatContext = ptr::null_mut();
            let output_path_str = output_path.to_str().unwrap();

            // Deduce output format based on file extension (mp4)
            let fmt = av_guess_format(ptr::null(), output_path_str.as_ptr() as *const i8, ptr::null());
            if fmt.is_null() {
                return Err("Could not deduce output format".to_string());
            }
            avformat_alloc_output_context2(&mut format_context, fmt, ptr::null(), output_path_str.as_ptr() as *const i8);
            if format_context.is_null() {
                return Err("Could not allocate output context".to_string());
            }

            // Find codec for video encoding (H.264 for MP4)
            let codec = avcodec_find_encoder(AVCodecID_AV_CODEC_ID_H264);
            if codec.is_null() {
                return Err("Could not find encoder".to_string());
            }

            // Create a new stream for the video
            let stream = avformat_new_stream(format_context, codec);
            if stream.is_null() {
                return Err("Could not create new stream".to_string());
            }

            // Allocate codec context
            let codec_context = avcodec_alloc_context3(codec);
            if codec_context.is_null() {
                return Err("Could not allocate codec context".to_string());
            }

            // Set video parameters (resolution, framerate, pixel format)
            (*codec_context).width = resolution.0 as i32;
            (*codec_context).height = resolution.1 as i32;
            (*codec_context).time_base = AVRational { num: 1, den: frame_rate as i32 };
            (*codec_context).framerate = AVRational { num: frame_rate as i32, den: 1 };
            (*codec_context).pix_fmt = AVPixelFormat_AV_PIX_FMT_YUV420P;

            // Set codec parameters and open the codec
            if avcodec_open2(codec_context, codec, ptr::null_mut()) < 0 {
                return Err("Could not open codec".to_string());
            }

            // Allocate frame for encoding
            let frame = av_frame_alloc();
            if frame.is_null() {
                return Err("Could not allocate frame".to_string());
            }
            (*frame).format = (*codec_context).pix_fmt;
            (*frame).width = (*codec_context).width;
            (*frame).height = (*codec_context).height;

            // Allocate frame buffer
            av_frame_get_buffer(frame, 32);

            // Allocate packet for encoding
            let pkt = av_packet_alloc();
            if pkt.is_null() {
                return Err("Could not allocate packet".to_string());
            }

            // Open output file
            if avio_open(&mut (*format_context).pb, output_path_str.as_ptr() as *const i8, AVIO_FLAG_WRITE as i32) < 0 {
                return Err("Could not open output file".to_string());
            }

            // Write header to the output file
            avformat_write_header(format_context, ptr::null_mut());

            Ok(Self {
                format_context,
                codec_context,
                stream,
                frame,
                pkt,
                next_pts: 0,
            })
        }
    }

    pub fn push_frame(&mut self, image_path: &PathBuf) -> Result<(), String> {
        unsafe {
            // Load the image (PNG)
            let img = xcap::image::open(image_path).map_err(|e| e.to_string())?.to_rgb8();
            let width = img.width() as i32;
            let height = img.height() as i32;

            // Check if the image matches the encoder settings
            if width != (*self.frame).width || height != (*self.frame).height {
                return Err("Image dimensions do not match video dimensions".to_string());
            }

            // Convert image to YUV420P format (you need to implement this)
            let yuv_data = rgb_to_yuv420p(&img);
            ptr::copy_nonoverlapping(yuv_data.as_ptr(), (*self.frame).data[0], yuv_data.len());

            // Set PTS (presentation timestamp) for the frame
            (*self.frame).pts = self.next_pts;
            self.next_pts += 1;

            // Make the frame writable before sending it to the encoder
            av_frame_make_writable(self.frame);

            // Send frame to the encoder
            av_packet_unref(self.pkt);
            let ret = avcodec_send_frame(self.codec_context, self.frame);
            if ret < 0 {
                return Err(format!("Error sending frame: {}", ret));
            }

            // Receive encoded packet and write it
            while ret >= 0 {
                let ret = avcodec_receive_packet(self.codec_context, self.pkt);
                if ret == AVERROR_EAGAIN as i32 || ret == AVERROR_EOF as i32 {
                    break;
                } else if ret < 0 {
                    return Err(format!("Error receiving packet: {}", ret));
                }

                // Rescale packet timestamps
                av_packet_rescale_ts(self.pkt, (*self.codec_context).time_base, (*self.stream).time_base);

                // Write the packet to the output file
                av_interleaved_write_frame(self.format_context, self.pkt);
            }
            Ok(())
        }
    }

    pub fn finalize(self) -> Result<(), String> {
        unsafe {
            // Write the trailer to finalize the video file
            av_write_trailer(self.format_context);
            avio_closep(&mut (*self.format_context).pb);
            Ok(())
        }
    }
}

impl Drop for VideoEncoder {
    fn drop(&mut self) {
        unsafe {
            av_frame_free(&mut self.frame);
            av_packet_free(&mut self.pkt);
            avcodec_free_context(&mut self.codec_context);
            avformat_free_context(self.format_context);
        }
    }
}

// RGB to YUV420P conversion (you should implement this conversion)
fn rgb_to_yuv420p(rgb: &xcap::image::RgbImage) -> Vec<u8> {
    let width = rgb.width() as usize;
    let height = rgb.height() as usize;
    let mut yuv = Vec::with_capacity(width * height * 3 / 2);

    // Convert RGB to Y
    for y in 0..height {
        for x in 0..width {
            let pixel = rgb.get_pixel(x as u32, y as u32);
            let r = pixel[0] as f32;
            let g = pixel[1] as f32;
            let b = pixel[2] as f32;

            let y_val = (0.299 * r + 0.587 * g + 0.114 * b).round() as u8;
            yuv.push(y_val);
        }
    }

    // Downsample U and V components (4:2:0 chroma subsampling)
    let mut u_vals = Vec::with_capacity(width * height / 4);
    let mut v_vals = Vec::with_capacity(width * height / 4);

    for y in (0..height).step_by(2) {
        for x in (0..width).step_by(2) {
            let mut u = 0.0;
            let mut v = 0.0;

            for iy in 0..2 {
                for ix in 0..2 {
                    let px = rgb.get_pixel((x + ix) as u32, (y + iy) as u32);
                    let r = px[0] as f32;
                    let g = px[1] as f32;
                    let b = px[2] as f32;

                    u += (-0.1687 * r - 0.3313 * g + 0.5 * b).round();
                    v += (0.5 * r - 0.4187 * g - 0.0813 * b).round();
                }
            }

            u /= 4.0;
            v /= 4.0;

            u_vals.push(u as u8);
            v_vals.push(v as u8);
        }
    }

    // Append U and V to the YUV vector
    yuv.extend_from_slice(&u_vals);
    yuv.extend_from_slice(&v_vals);

    yuv
}