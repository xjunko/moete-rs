use moete_core::{MoeteContext, MoeteError};

/// Replies with a list of available commands and their descriptions.
#[poise::command(prefix_command, slash_command, category = "Utility")]
pub async fn help(
    ctx: MoeteContext<'_>,
    #[description = "Specific command"]
    #[rest]
    command: Option<String>,
) -> Result<(), MoeteError> {
    moete_discord::help::help(
        ctx,
        command.as_deref(),
        moete_discord::help::HelpConfiguration::default(),
    )
    .await?;
    Ok(())
}
