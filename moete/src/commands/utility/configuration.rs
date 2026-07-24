use moete_core::{
    MoeteContext,
    MoeteError,
};
use poise::CreateReply;

/// Replies with the current configuration for the guild.
#[poise::command(
    prefix_command,
    slash_command,
    guild_only,
    category = "Utility",
    aliases("config", "cfg"),
    subcommands("set")
)]
pub async fn configuration(ctx: MoeteContext<'_>) -> Result<(), MoeteError> {
    let state: &moete_core::State = ctx.data();
    if let Some(database) = state.database.as_ref() {
        let config = moete_infra::services::configuration::get(
            database,
            ctx.guild_id().unwrap().into(),
        )
        .await?;

        let mut embed = moete_discord::embed::create_embed()
            .title(format!(
                "{} | Current Configuration",
                state.config.discord.name
            ))
            .description("_ _");

        // help
        embed = embed.field(
            "Help",
            format!("**ID**: `{}`\nUse `{}{} set <option> <value>` to update a configuration option.", config.guild_id,ctx.prefix(), ctx.command().name),
            false,
        );

        // server
        embed = embed.field(
            "Server Configuration",
            {
                let server_json = serde_json::to_value(config.server)?;
                let mut result = String::new();
                result.push_str("```json\n{\n");
                for (key, value) in server_json.as_object().unwrap() {
                    let desc = value
                        .get("description")
                        .unwrap()
                        .as_str()
                        .unwrap_or("No description provided.");

                    let value = value.get("value").unwrap();

                    result.push_str(&format!("\t/// {}\n", desc));
                    result.push_str(&format!("\t{}: {}\n", key, value));
                }
                result.push_str("}\n```");
                result
            },
            false,
        );

        ctx.send(CreateReply::default().embed(embed)).await?;
    }
    Ok(())
}

#[poise::command(
    prefix_command,
    slash_command,
    category = "Utility",
    guild_only,
    required_permissions = "ADMINISTRATOR"
)]
pub async fn set(
    ctx: MoeteContext<'_>,
    #[description = "Name of the configuration option to set"] option: String,
    #[description = "The new value for the configuration option"] value: String,
) -> Result<(), MoeteError> {
    let state: &moete_core::State = ctx.data();
    let Some(database) = state.database.as_ref() else { return Ok(()) };

    let guild_id = ctx.guild_id().unwrap().into();
    let mut config =
        moete_infra::services::configuration::get(database, guild_id).await?;

    if let Err(err) = config.server.set_field(&option, &value) {
        ctx.reply(err.to_string()).await?;
        return Ok(());
    }

    moete_infra::services::configuration::update(database, &config).await?;
    ctx.reply(format!(
        "Configuration option `{}` updated to `{}`",
        option, value
    ))
    .await?;

    Ok(())
}
