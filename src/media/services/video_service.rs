use ffprobe::ffprobe;
use std::{
    fs,
    path::Path,
    process::{Command, Stdio},
};

use crate::{
    media::models::media_metadata_model::MediaMetadataModel, user::models::user_model::UserModel,
};

pub struct VideoService {}

impl VideoService {
    pub fn extract_video_metadata(path: &str, metadata: &mut MediaMetadataModel) {
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
        user: &UserModel,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let output_path = format!("./uploads/{}/thumbnails", user.uuid);
        fs::create_dir_all(&output_path)?;

        let stem = Path::new(filename)
            .file_stem() // gets the filename without extension
            .and_then(|s| s.to_str())
            .ok_or("Invalid filename")?;

        let thumbnail_path = format!("{}/{}.webp", output_path, stem);

        let status = Command::new("ffmpeg")
            .args([
                "-y",
                "-hide_banner",
                "-loglevel",
                "error",
                "-ss",
                "00:00:01",
                "-t",
                "3",
                "-i",
                filepath,
                "-vf",
                &format!("fps=10, scale={}:-1:flags=lanczos", max_width),
                "-loop",
                "0",
                &thumbnail_path,
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;

        if !status.success() {
            return Err("Failed to generate WebP thumbnail".into());
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
