#[derive(sqlx::FromRow, Debug)]
#[allow(dead_code)]
pub struct Shortcut {
    pub id: i64,
    pub guild_id: i64,
    pub trigger: String,
    pub response: String,
}

impl Shortcut {
    pub fn responses(&self) -> Vec<String> {
        self.response.split(",").map(|s| s.to_string()).collect()
    }
}
