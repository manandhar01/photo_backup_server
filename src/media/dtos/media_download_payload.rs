use serde::Deserialize;

#[derive(Deserialize)]
pub struct MediaDownloadPayload {
    pub chunk_size: usize,
    pub offset: u64,
}
