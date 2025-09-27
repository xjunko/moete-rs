use moete_core::{MoeteContext, MoeteError};

/// Replies with a result of the math calculation.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Math",
    aliases("calculate", "eval")
)]
pub async fn calc(
    ctx: MoeteContext<'_>,
    #[description = "Math expression to evaluate"]
    #[rest]
    math_expr: String,
) -> Result<(), MoeteError> {
    if let Ok(result) = evalexpr::eval_int(&math_expr) {
        ctx.say(format!("{}", result)).await?;
    } else {
        ctx.say("Failed to evaluate the expression. Please ensure it's a valid integer math expression.").await?;
    }
    Ok(())
}
