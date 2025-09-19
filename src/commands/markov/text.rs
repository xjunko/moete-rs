use poise::CreateReply;
use rand::Rng;
use serenity::all::{ExecuteWebhook, UserId};

use crate::builtins;
use crate::{Context, Error};

const ALLOWED: &[u64] = &[
    224785877086240768, // Me
    255365914898333707, // Asperl
    393201541630394368, // Nity
    261815692150571009, // Neko
    383468859211907083, // Mattsu
    546976983012212746, // Arissa
    658976239100362753, // aXccel
    223367426526412800, // Bright
    655023950689992706, // Nia
    406463187052134411, // Mooth
    315116686850129922, // Mystia
    443022415451127808, // Salty
    360256398933753856, // Zavents
    286438559341215746, // Kagi
    210065177574375424, // Eima
    369858698941693953, // Amano
    824537345541799976, // Piqi
    304512418976104450, // WilsonYogi
    922423503909687317, // Zed
    736223131240497183, // rmhakurei
];

fn load_data(id: u64) -> Option<String> {
    std::fs::read_to_string(format!("data/{}.txt", id)).ok()
}

fn generate(picked: i32) -> Option<(String, u64)> {
    if picked <= 0 || picked > ALLOWED.len() as i32 {
        return None;
    }

    let user_id = ALLOWED[picked as usize - 1];
    let result = {
        let data = load_data(user_id)?;
        let text = marukov::Text::new(data);
        let mut rng = rand::rng();
        let res = text.generate(marukov::TextOptions {
            tries: 9999,
            min_words: rng.random_range(0..10),
            max_words: rng.random_range(50..100),
        });

        std::mem::drop(text);
        std::mem::drop(rng);

        res
    };

    // text generation uses a lot of memory, trim the memory here.
    unsafe {
        libc::malloc_trim(0);
    }

    Some((result.clone(), user_id))
}

/// Generates a random text based on the user's messages.
#[poise::command(prefix_command, category = "Markov", aliases("deep"))]
pub async fn markov(
    ctx: Context<'_>,
    #[description = "User to generate text for"] picked: Option<i32>,
) -> Result<(), Error> {
    if let Some(picked) = picked
        && let Some((content, user_id)) = generate(picked)
    {
        if let Ok(user) = UserId::new(user_id).to_user(ctx.http()).await
            && let Some(webhook) = builtins::discord::webhook::get_or_create_webhook(
                ctx.serenity_context(),
                ctx.channel_id(),
            )
            .await
        {
            let _ = webhook
                .execute(
                    ctx.serenity_context(),
                    true,
                    ExecuteWebhook::new()
                        .username(user.display_name())
                        .avatar_url(user.avatar_url().unwrap_or(user.default_avatar_url()))
                        .content(content),
                )
                .await;
        }
    } else {
        // Show everyone's stats on error.
        let mut available_users = Vec::new();
        for (n, id) in ALLOWED.iter().enumerate() {
            if let Ok(user) = UserId::new(*id).to_user(ctx.http()).await {
                available_users.push(format!("{}. {} | {} messages", n + 1, user.name, -1));
            } else {
                available_users.push(format!("{}. Unknown User | {} messages", n + 1, -1));
            }
        }

        let embed = builtins::discord::embed::create_embed()
            .title("Markovify | Main")
            .field("Available", available_users.join("\n"), true);
        ctx.send(CreateReply::default().embed(embed)).await?;
    }

    Ok(())
}
