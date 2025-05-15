use serde::Serialize;

#[derive(Serialize)]
pub struct PaginationMetadataDto {
    pub limit: i64,
    pub offset: i64,
    pub total: i64,
}
