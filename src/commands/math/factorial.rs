use poise::CreateReply;

use crate::builtins;
use crate::{Context, Error};

/// Calculate the factorial of a number.
#[poise::command(prefix_command, category = "Math", aliases("factorials"))]
pub async fn factorial(
    ctx: Context<'_>,
    #[description = "Number to calculate the factorial of"]
    #[rest]
    number: f64,
) -> Result<(), Error> {
    let embed = builtins::discord::embed::create_embed()
        .title(format!("Factorials of {}", number))
        .description({
            let mut result: String = String::new();
            let limit = (number.sqrt() + 1.0) as u64;

            for i in 1..limit {
                if (number as u64) % i == 0 && result.len() < 1024 {
                    result.push_str(&format!(
                        "`{} x {} = {}` \n",
                        i,
                        (number as u64) / i,
                        number
                    ));
                }
            }
            result
        });

    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}
