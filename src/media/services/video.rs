use ffprobe::ffprobe;

use crate::media::models::media_metadata::MediaMetadata;

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
