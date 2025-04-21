use sqlx::PgPool;

pub trait ModelOps: Sized + Send {
    // async fn find() -> sqlx::Result<Option<Self>>;
    // async fn create(&self) -> sqlx::Result<Self>;
    async fn soft_delete(&mut self, pool: &PgPool) -> sqlx::Result<()>;
}
