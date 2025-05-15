use serde::Deserialize;

#[derive(Deserialize)]
pub struct MediaDownloadPayloadDto {
    pub chunk_size: usize,
    pub offset: u64,
}
