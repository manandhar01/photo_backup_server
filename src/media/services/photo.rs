use axum::http::StatusCode;
use chrono::{NaiveDateTime, TimeZone, Utc};
use exif::Tag;
use std::fs::File;

use crate::media::models::media::MediaAttributes;

pub struct PhotoService {}

impl PhotoService {
    pub fn extract_photo_metadata(
        path: &str,
        attributes: &mut MediaAttributes,
    ) -> Result<(), (StatusCode, String)> {
        let file =
            File::open(path).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        let mut bufreader = std::io::BufReader::new(file);

        let exifreader = exif::Reader::new();
        let exif = exifreader
            .read_from_container(&mut bufreader)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        for field in exif.fields() {
            match field.tag {
                Tag::Make => {
                    attributes.camera_make = Some(field.display_value().to_string());
                }
                Tag::Model => {
                    attributes.camera_model = Some(field.display_value().to_string());
                }
                Tag::ImageWidth => {
                    attributes.width = field.value.get_uint(0);
                }
                Tag::ImageLength => {
                    attributes.height = field.value.get_uint(0);
                }
                Tag::LensModel => {
                    attributes.lens_model = Some(field.display_value().to_string());
                }
                Tag::FocalLength => {
                    attributes.focal_length = field.value.get_uint(0).map(|v| v as f32);
                }
                Tag::ISOSpeed => {
                    attributes.iso = field.value.get_uint(0);
                }
                Tag::ApertureValue => {
                    attributes.aperture = field.value.get_uint(0).map(|v| v as f32);
                }
                Tag::ShutterSpeedValue => {
                    attributes.shutter_speed = field.value.get_uint(0).map(|v| v as u16);
                }
                Tag::DateTimeOriginal | Tag::DateTime => {
                    let dt_str = field.display_value().to_string();
                    if let Ok(naive) = NaiveDateTime::parse_from_str(&dt_str, "%Y:%m:%d %H:%M:%S") {
                        attributes.taken_at = Some(Utc.from_utc_datetime(&naive));
                    }
                }
                Tag::GPSDestLongitude => {
                    attributes.longitude = field.value.get_uint(0).map(|v| v as f64);
                }
                Tag::GPSDestLatitude => {
                    attributes.latitude = field.value.get_uint(0).map(|v| v as f64);
                }
                _ => {}
            }
        }

        Ok(())
    }
}
