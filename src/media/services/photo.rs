use chrono::{DateTime, NaiveDateTime};
use exif::{Reader, Tag};
use imageinfo::ImageInfo;
use std::{fs::File, io::Seek, str::FromStr};

use crate::media::models::media::MediaAttributes;

pub struct PhotoService {}

impl PhotoService {
    pub fn extract_photo_metadata(path: &str, attributes: &mut MediaAttributes) {
        let file = match File::open(path) {
            Ok(file) => file,
            Err(e) => return eprintln!("{:?}", e),
        };

        let mut bufreader = std::io::BufReader::new(file);

        match ImageInfo::from_reader(&mut bufreader) {
            Ok(info) => {
                attributes.width = Some(info.size.width as u32);
                attributes.height = Some(info.size.height as u32);
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
                                u32::from_str(&field.display_value().to_string()),
                                attributes.width,
                            ) {
                                attributes.width = Some(width);
                            }
                        }
                        Tag::PixelYDimension => {
                            if let (Ok(height), None) = (
                                u32::from_str(&field.display_value().to_string()),
                                attributes.height,
                            ) {
                                attributes.height = Some(height);
                            }
                        }
                        Tag::Make => {
                            attributes.camera_make = Some(field.display_value().to_string());
                        }
                        Tag::Model => {
                            attributes.camera_model = Some(field.display_value().to_string());
                        }
                        Tag::FocalLength => {
                            attributes.focal_length =
                                Some(field.display_value().with_unit(&exif).to_string());
                        }
                        Tag::ApertureValue => {
                            attributes.aperture =
                                Some(field.display_value().with_unit(&exif).to_string());
                        }
                        Tag::DateTimeOriginal | Tag::DateTime => {
                            let dt_str = field.display_value().to_string();

                            if let Some(datetime) = Self::parse_exif_datetime(&dt_str) {
                                attributes.taken_at = Some(datetime);
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
