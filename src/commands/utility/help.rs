use crate::builtins;
use crate::{Context, Error};

/// Replies with a list of available commands and their descriptions.
#[poise::command(prefix_command, category = "Utility")]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command"] command: Option<String>,
) -> Result<(), Error> {
    builtins::discord::help::help(
        ctx,
        command.as_deref(),
        builtins::discord::help::HelpConfiguration::default(),
    )
    .await?;
    Ok(())
}
