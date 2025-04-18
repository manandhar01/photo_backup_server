use axum::http::StatusCode;

use crate::media::models::media::MediaAttributes;

pub struct VideoService {}

impl VideoService {
    pub fn extract_video_metadata(
        path: &str,
        attributes: &mut MediaAttributes,
    ) -> Result<(), (StatusCode, String)> {
        println!("{path}");
        println!("{attributes:?}");

        Ok(())
    }
}
