use poise::CreateReply;
use serenity::all::Color;

use moete_core::{MoeteContext, MoeteError};

/// Lists all possible shortcut created by the Admin(s) for this server.
#[poise::command(
    prefix_command,
    slash_command,
    guild_only,
    category = "Utility",
    aliases("macro", "macros", "shortcuts"),
    subcommands("add", "remove", "update")
)]
pub async fn shortcut(ctx: MoeteContext<'_>) -> Result<(), MoeteError> {
    let state: &moete_core::State = ctx.data();

    if let Some(pool) = state.pool.as_ref() {
        let shortcuts =
            moete_core::shortcut::get_all_shortcuts_for_guild(pool, ctx.guild_id().unwrap().into())
                .await?;

        let icon_url = {
            if let Some(guild) = ctx.guild()
                && let Some(guild_url) = guild.icon_url()
            {
                guild_url
            } else {
                ctx.author().face()
            }
        };

        let mut embed = moete_discord::embed::create_embed()
            .title("Shortcuts | Main")
            .thumbnail(icon_url)
            .description(
                "`Admins can create a server-sided macro/shortcuts.` \n\n\
                **The shortcuts will run as a normal command, so use it with the bot's prefix.**",
            );

        // help
        embed = embed.field(
            "Commands",
            r#"
```
/shortcut add <trigger> <response>  
    - Adds a shortcut to the server*.
/shortcut remove <trigger>          
    - Removes a shortcut from the server*.
/shortcut update <trigger> <response>
    - Updates a shortcut's response for the server*.
/shortcut                           
    - Lists all shortcuts for the server.

* Requires `ADMINISTRATOR` permission to use.
```"#,
            false,
        );

        // list (if any)
        let shortcuts_text = if shortcuts.is_empty() {
            "`No shortcuts found for this server.`".to_string()
        } else {
            let mut description = String::new();
            description.push_str("```bash\n");
            for (n, shortcut) in shortcuts.iter().enumerate() {
                description.push_str(&format!(
                    "{}) {}\n{}\n",
                    n + 1,
                    shortcut.trigger,
                    shortcut
                        .responses()
                        .iter()
                        .map(|s| format!("\t- {}", s))
                        .collect::<Vec<_>>()
                        .join("\n")
                ));
            }
            description.push_str("```");
            description
        };

        embed = embed.field("Shortcuts", &shortcuts_text, false);

        ctx.send(CreateReply::default().embed(embed).reply(true))
            .await?;
    } else {
        ctx.say("Failed to get any info, database is not connected, if this happens in production, report to @rmhakurei.").await?;
    }

    Ok(())
}

/// Adds a shortcut to the server.
#[poise::command(
    prefix_command,
    slash_command,
    guild_only,
    category = "Utility",
    required_permissions = "ADMINISTRATOR"
)]
pub async fn add(
    ctx: MoeteContext<'_>,
    #[description = "Word to trigger the shortcut"] trigger: String,
    #[description = "Response for the shortcut"]
    #[rest]
    response: String,
) -> Result<(), MoeteError> {
    let state: &moete_core::State = ctx.data();
    let pool = match state.pool.as_ref() {
        Some(p) => p,
        None => {
            ctx.say("Database is not connected, cannot add shortcut.")
                .await?;
            return Ok(());
        },
    };
    let cache = state.shortcut_cache.clone();

    let mut embed = moete_discord::embed::create_embed()
        .title("Shortcuts | Add")
        .thumbnail({
            if let Some(guild) = ctx.guild()
                && let Some(guild_url) = guild.icon_url()
            {
                guild_url
            } else {
                ctx.author().face()
            }
        });

    // error handling
    {
        let mut error_occurred = false;
        match moete_core::shortcut::get_shortcut(pool, ctx.guild_id().unwrap().into(), &trigger)
            .await
        {
            Err(e) => {
                embed = embed
                    .field("Error", format!("Failed to add shortcut: {}", e), false)
                    .color(Color::RED);
                error_occurred = true;
            },
            Ok(Some(_)) => {
                embed = embed
                    .field("Error", "Shortcut with that trigger already exists.", false)
                    .color(Color::RED);
                error_occurred = true;
            },
            Ok(None) => {},
        }

        if error_occurred {
            ctx.send(CreateReply::default().embed(embed).reply(true))
                .await?;
            return Ok(());
        }
    }

    // add shortcut
    {
        match moete_core::shortcut::add_shortcut(
            pool,
            ctx.guild_id().unwrap().into(),
            &trigger,
            &response,
            &cache,
        )
        .await
        {
            Err(e) => {
                embed = embed
                    .field("Error", format!("Failed to add shortcut: {}", e), false)
                    .color(Color::RED);
            },
            Ok(_) => {
                embed = embed
                    .field(
                        "Success",
                        format!("Shortcut `{}` added successfully.", trigger),
                        false,
                    )
                    .color(Color::DARK_GREEN)
                    .field(
                        "Info",
                        format!("**Trigger**: `{}`\n**Response**: `{}`", trigger, response),
                        false,
                    );
            },
        }
    }

    ctx.send(CreateReply::default().embed(embed).reply(true))
        .await?;

    Ok(())
}

/// Removes a shortcut from the server.
#[poise::command(
    prefix_command,
    slash_command,
    guild_only,
    category = "Utility",
    required_permissions = "ADMINISTRATOR"
)]
pub async fn remove(
    ctx: MoeteContext<'_>,
    #[description = "Shortcut name to remove"] trigger: String,
) -> Result<(), MoeteError> {
    let state: &moete_core::State = ctx.data();
    let pool = match state.pool.as_ref() {
        Some(p) => p,
        None => {
            ctx.say("Database is not connected, cannot remove shortcut.")
                .await?;
            return Ok(());
        },
    };
    let cache = state.shortcut_cache.clone();

    let mut embed = moete_discord::embed::create_embed()
        .title("Shortcuts | Remove")
        .thumbnail({
            if let Some(guild) = ctx.guild()
                && let Some(guild_url) = guild.icon_url()
            {
                guild_url
            } else {
                ctx.author().face()
            }
        });

    match moete_core::shortcut::remove_shortcut(
        pool,
        ctx.guild_id().unwrap().into(),
        &trigger,
        &cache,
    )
    .await
    {
        Err(e) => {
            embed = embed
                .description(format!("Failed to remove shortcut: {}", e))
                .color(Color::RED);
        },
        Ok(deleted) => {
            if deleted {
                embed = embed
                    .description(format!("Shortcut `{}` removed successfully.", trigger))
                    .color(Color::DARK_GREEN);
            } else {
                embed = embed
                    .description(format!("No shortcut found with the trigger: `{}`", trigger))
                    .color(Color::DARK_RED);
            }
        },
    }

    ctx.send(CreateReply::default().embed(embed).reply(true))
        .await?;

    Ok(())
}

/// Updates a shortcut from the server.
#[poise::command(
    prefix_command,
    slash_command,
    guild_only,
    category = "Utility",
    required_permissions = "ADMINISTRATOR"
)]
pub async fn update(
    ctx: MoeteContext<'_>,
    #[description = "Shortcut name to update"] trigger: String,
    #[description = "New response for the shortcut"]
    #[rest]
    new_response: String,
) -> Result<(), MoeteError> {
    let state: &moete_core::State = ctx.data();
    let pool = match state.pool.as_ref() {
        Some(p) => p,
        None => {
            ctx.say("Database is not connected, cannot remove shortcut.")
                .await?;
            return Ok(());
        },
    };
    let cache = state.shortcut_cache.clone();

    let mut embed = moete_discord::embed::create_embed()
        .title("Shortcuts | Remove")
        .thumbnail({
            if let Some(guild) = ctx.guild()
                && let Some(guild_url) = guild.icon_url()
            {
                guild_url
            } else {
                ctx.author().face()
            }
        });

    match moete_core::shortcut::edit_shortcut(
        pool,
        ctx.guild_id().unwrap().into(),
        &trigger,
        &new_response,
        &cache,
    )
    .await
    {
        Err(e) => {
            embed = embed
                .description(format!("Failed to update shortcut: {}", e))
                .color(Color::RED);
        },
        Ok(deleted) => {
            if deleted {
                embed = embed
                    .description(format!(
                        "Shortcut `{}` updated successfully to `{}`",
                        trigger, new_response
                    ))
                    .color(Color::DARK_GREEN);
            } else {
                embed = embed
                    .description(format!("No shortcut found with the trigger: `{}`", trigger))
                    .color(Color::DARK_RED);
            }
        },
    }

    ctx.send(CreateReply::default().embed(embed).reply(true))
        .await?;

    Ok(())
}
