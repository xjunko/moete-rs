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

// pub async fn get_shortcut(
//     pool: &postgres::PgPool,
//     guild_id: i64,
//     trigger: &str,
// ) -> Result<Option<Shortcut>, sqlx::Error> {
//     let shortcut: Option<Shortcut> = sqlx::query_as::<_, Shortcut>(
//         "SELECT id, guild_id, trigger, response FROM shortcuts WHERE guild_id = $1 AND trigger = $2",
//     ).bind(guild_id).bind(trigger).fetch_optional(pool).await?;
//     Ok(shortcut)
// }

// pub async fn get_all_shortcuts_for_guild(
//     pool: &postgres::PgPool,
//     guild_id: i64,
// ) -> Result<Vec<Shortcut>, sqlx::Error> {
//     let shortcuts: Vec<Shortcut> = sqlx::query_as::<_, Shortcut>(
//         "SELECT id, guild_id, trigger, response FROM shortcuts WHERE guild_id = $1",
//     )
//     .bind(guild_id)
//     .fetch_all(pool)
//     .await?;
//     Ok(shortcuts)
// }

// pub async fn remove_shortcut(
//     pool: &postgres::PgPool,
//     guild_id: i64,
//     trigger: &str,
//     cache: &ShortcutCache,
// ) -> Result<bool, sqlx::Error> {
//     let res = sqlx::query(
//         "DELETE FROM shortcuts WHERE guild_id = $1 AND trigger = $2",
//     )
//     .bind(guild_id)
//     .bind(trigger)
//     .execute(pool)
//     .await?;

//     if res.rows_affected() > 0 {
//         cache.remove(guild_id);
//         return Ok(true);
//     }

//     Ok(false)
// }

// pub async fn add_shortcut(
//     pool: &postgres::PgPool,
//     guild_id: i64,
//     trigger: &str,
//     response: &str,
//     cache: &ShortcutCache,
// ) -> Result<(), sqlx::Error> {
//     sqlx::query("INSERT INTO shortcuts (guild_id, trigger, response) VALUES ($1, $2, $3)")
//         .bind(guild_id)
//         .bind(trigger)
//         .bind(response)
//         .execute(pool)
//         .await?;
//     cache.remove(guild_id);

//     Ok(())
// }

// pub async fn edit_shortcut(
//     pool: &postgres::PgPool,
//     guild_id: i64,
//     trigger: &str,
//     new_response: &str,
//     cache: &ShortcutCache,
// ) -> Result<bool, sqlx::Error> {
//     // abstract away the addition if it doesn't exist
//     {
//         if get_shortcut(pool, guild_id, trigger).await?.is_none() {
//             add_shortcut(pool, guild_id, trigger, new_response, cache).await?;
//             return Ok(true);
//         }
//     }

//     let res =
//         sqlx::query("UPDATE shortcuts SET response = $1 WHERE guild_id = $2 AND trigger = $3")
//             .bind(new_response)
//             .bind(guild_id)
//             .bind(trigger)
//             .execute(pool)
//             .await?;

//     if res.rows_affected() > 0 {
//         cache.remove(guild_id);
//         return Ok(true);
//     }

//     Ok(false)
// }
