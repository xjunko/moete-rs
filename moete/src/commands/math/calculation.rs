use moete_core::{
    MoeteContext,
    MoeteError,
};

/// Does nothing for now...
#[poise::command(
    prefix_command,
    slash_command,
    category = "Math",
    aliases("calculate", "eval")
)]
pub async fn calc(
    _ctx: MoeteContext<'_>,
    #[description = "Math expression to evaluate"]
    #[rest]
    _code: String,
) -> Result<(), MoeteError> {
    Ok(())
}
