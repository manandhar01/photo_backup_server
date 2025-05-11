pub struct LoginActivityDto {
    pub user_id: Option<i32>,
    pub email: String,
    pub success: bool,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}
