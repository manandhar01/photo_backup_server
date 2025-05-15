use serde::Deserialize;

#[derive(Deserialize)]
pub struct MediaListPayloadDto {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
