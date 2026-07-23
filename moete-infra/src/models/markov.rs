#[derive(sqlx::FromRow, Debug)]
#[allow(dead_code)]
pub struct User {
    pub id: i64,
    pub count: i64,
}

#[derive(sqlx::FromRow, Debug)]
#[allow(dead_code)]
pub struct Message {
    pub id: i64,
    pub user_id: i64,
    pub content: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct MarkovUser {
    pub id: i64,
    pub count: i64,
    pub messages: Vec<Message>,
}

impl From<(User, Vec<Message>)> for MarkovUser {
    fn from((user, messages): (User, Vec<Message>)) -> Self {
        Self { id: user.id, count: user.count, messages }
    }
}
