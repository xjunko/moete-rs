#[derive(sqlx::FromRow, Debug)]
#[allow(dead_code)]
pub struct Counter {
    pub user_id: i64,
    pub word: String,
    pub count: i64,
}
