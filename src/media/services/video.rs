use std::{fs, path::Path};

use ffmpeg_next as ffmpeg;
use ffprobe::ffprobe;
use image::{ImageBuffer, RgbImage};

use crate::{media::models::media_metadata::MediaMetadata, user::models::user::User};

pub struct VideoService {}

impl VideoService {
    pub fn extract_video_metadata(path: &str, metadata: &mut MediaMetadata) {
        match ffprobe(path) {
            Ok(info) => {
                if let Some(duration_str) = &info.format.duration {
                    if let Ok(duration) = duration_str.parse::<f64>() {
                        metadata.duration = Some(duration);
                    }
                }

                if let Some(tags) = &info.format.tags {
                    if let Some(creation_time) = &tags.creation_time {
                        if let Ok(datetime) = chrono::DateTime::parse_from_rfc3339(creation_time) {
                            metadata.taken_at = Some(datetime.naive_utc());
                        }
                    }
                }

                for stream in info.streams {
                    match stream.codec_type.as_deref() {
                        Some("video") => {
                            metadata.video_codec = stream.codec_name.clone();
                            metadata.width = stream.width.map(|w| w as i32);
                            metadata.height = stream.height.map(|h| h as i32);
                            metadata.video_bitrate = stream.bit_rate.clone();
                            metadata.frame_rate =
                                Self::parse_ffmpeg_rational(&stream.avg_frame_rate);
                        }
                        Some("audio") => {
                            metadata.audio_codec = stream.codec_name.clone();
                            metadata.audio_bitrate = stream.bit_rate.clone();
                            metadata.sample_rate = stream.sample_rate.clone();
                        }
                        _ => {}
                    }
                }
            }
            Err(e) => {
                eprintln!("Could not analyze file with ffprobe: {:?}", e);
            }
        }
    }

    pub async fn generate_video_thumbnail(
        filepath: &str,
        filename: &str,
        max_width: u32,
        user: &User,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let output_path = format!("./uploads/{}/thumbnails", user.uuid);
        fs::create_dir_all(&output_path)?;

        let stem = Path::new(filename)
            .file_stem() // gets the filename without extension
            .and_then(|s| s.to_str())
            .ok_or("Invalid filename")?;

        let thumbnail_path = format!("{}/{}.jpg", output_path, stem);

        ffmpeg::init()?;

        let mut ictx = ffmpeg::format::input(&filepath)?;
        let input = ictx
            .streams()
            .best(ffmpeg::media::Type::Video)
            .ok_or("No video stream found")?;
        let stream_index = input.index();

        let mut decoder = ffmpeg::codec::context::Context::from_parameters(input.parameters())?
            .decoder()
            .video()?;

        let mut scaler = ffmpeg::software::scaling::context::Context::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            ffmpeg::format::Pixel::RGB24,
            max_width,
            max_width,
            ffmpeg::software::scaling::flag::Flags::BILINEAR,
        )?;

        for (stream, packet) in ictx.packets() {
            if stream.index() == stream_index {
                decoder.send_packet(&packet)?;

                let mut decoded = ffmpeg::util::frame::Video::empty();
                if decoder.receive_frame(&mut decoded).is_ok() {
                    // First frame only
                    let mut rgb_frame = ffmpeg::util::frame::Video::empty();
                    scaler.run(&decoded, &mut rgb_frame)?;

                    // Save using `image` crate
                    let data = rgb_frame.data(0);
                    let width = rgb_frame.width();
                    println!("{}", width);
                    let height = rgb_frame.height();
                    println!("{}", height);

                    let img: RgbImage = ImageBuffer::from_raw(width, height, data.to_vec())
                        .ok_or("Failed to create image buffer")?;
                    match img.save(thumbnail_path.clone()) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("{:?}", e);
                        }
                    }

                    break;
                }
            }
        }

        Ok(thumbnail_path)
    }

    fn parse_ffmpeg_rational(rate: &str) -> Option<f32> {
        let parts: Vec<&str> = rate.split('/').collect();

        if parts.len() == 2 {
            if let (Ok(n), Ok(d)) = (parts[0].parse::<f32>(), parts[1].parse::<f32>()) {
                if d != 0.0 {
                    return Some(n / d);
                }
            }
        }
        None
    }
}
