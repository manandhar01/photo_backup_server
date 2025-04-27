use serde::Deserialize;

#[derive(Deserialize)]
pub struct MediaListPayload {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
