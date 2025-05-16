use chrono::{DateTime, NaiveDateTime};
use exif::{Reader, Tag};
use image::imageops::FilterType;
use imageinfo::ImageInfo;
use std::{
    fs::{self, File},
    io::Seek,
    str::FromStr,
};

use crate::media::models::MediaMetadataModel;
use crate::user::models::UserModel;

pub struct PhotoService {}

impl PhotoService {
    pub fn extract_photo_metadata(path: &str, metadata: &mut MediaMetadataModel) {
        let file = match File::open(path) {
            Ok(file) => file,
            Err(e) => return eprintln!("{:?}", e),
        };

        let mut bufreader = std::io::BufReader::new(file);

        match ImageInfo::from_reader(&mut bufreader) {
            Ok(info) => {
                metadata.width = Some(info.size.width as i32);
                metadata.height = Some(info.size.height as i32);
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }

        match bufreader.rewind() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }

        match Reader::new().read_from_container(&mut bufreader) {
            Ok(exif) => {
                for field in exif.fields() {
                    match field.tag {
                        Tag::PixelXDimension => {
                            if let (Ok(width), None) = (
                                i32::from_str(&field.display_value().to_string()),
                                metadata.width,
                            ) {
                                metadata.width = Some(width);
                            }
                        }
                        Tag::PixelYDimension => {
                            if let (Ok(height), None) = (
                                i32::from_str(&field.display_value().to_string()),
                                metadata.height,
                            ) {
                                metadata.height = Some(height);
                            }
                        }
                        Tag::Make => {
                            metadata.camera_make = Some(field.display_value().to_string());
                        }
                        Tag::Model => {
                            metadata.camera_model = Some(field.display_value().to_string());
                        }
                        Tag::FocalLength => {
                            metadata.focal_length =
                                Some(field.display_value().with_unit(&exif).to_string());
                        }
                        Tag::ApertureValue => {
                            metadata.aperture =
                                Some(field.display_value().with_unit(&exif).to_string());
                        }
                        Tag::DateTimeOriginal | Tag::DateTime => {
                            let dt_str = field.display_value().to_string();

                            if let Some(datetime) = Self::parse_exif_datetime(&dt_str) {
                                metadata.taken_at = Some(datetime);
                            }
                        }
                        _ => {}
                    }
                }
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
    }

    pub async fn generate_photo_thumbnail(
        filepath: &str,
        filename: &str,
        max_width: u32,
        user: &UserModel,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let img = image::open(filepath)?;

        let thumbnail = img.resize(max_width, max_width, FilterType::Lanczos3);

        let output_path = format!("./uploads/{}/thumbnails", user.uuid);
        fs::create_dir_all(&output_path)?;

        let thumbnail_path = format!("{}/{}", output_path, filename);

        thumbnail.save(&thumbnail_path)?;

        Ok(thumbnail_path)
    }

    fn parse_exif_datetime(dt_str: &str) -> Option<NaiveDateTime> {
        let formats = [
            "%Y:%m:%d %H:%M:%S",
            "%Y:%m:%d %H:%M:%S%:z",
            "%Y-%m-%d %H:%M:%S",
            "%Y-%m-%d %H:%M:%S%:z",
            "%+",
        ];

        for format in &formats {
            if let Ok(datetime) = DateTime::parse_from_str(dt_str, format) {
                return Some(datetime.naive_local());
            } else if let Ok(naive) = NaiveDateTime::parse_from_str(dt_str, format) {
                return Some(naive);
            }
        }

        None
    }
}
