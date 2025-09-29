use poise::CreateReply;

use moete_core::{MoeteContext, MoeteError};
use moete_discord as discord;

/// Converts a currency from one to another.
#[poise::command(prefix_command, slash_command, category = "Utility")]
pub async fn convert(
    ctx: MoeteContext<'_>,
    #[description = "Base currency"] base_currency: Option<String>,
    #[description = "Target currency"] target_currency: Option<String>,
    #[description = "Amount to convert"] amount: Option<f64>,
) -> Result<(), MoeteError> {
    // if all argument is valid
    if let Some(base) = base_currency
        && let Some(target) = target_currency
        && let Some(amount) = amount
    {
        let mut currency = ctx.data().currency.lock().await;
        let base_currency = currency.fetch(&base.to_lowercase()).await?;
        let target_currency = currency.fetch(&target.to_lowercase()).await?;

        if base_currency.is_none() {
            ctx.say("Unknown base currency").await?;
            return Ok(());
        }

        if target_currency.is_none() {
            ctx.say("Unknown target currency").await?;
            return Ok(());
        }

        let base_currency = base_currency.unwrap();
        let target_currency = target_currency.unwrap();
        let rate = base_currency.get_rate_to(&target_currency.name);

        if rate.is_none() {
            ctx.say(format!(
                "No exchange rate found from {} to {}",
                base_currency.name, target_currency.name
            ))
            .await?;
            return Ok(());
        }

        let rate = rate.unwrap();
        let converted_amount = readable::num::Float::from(amount * rate);
        ctx.say(format!(
            "{} {} = {:.2} {}",
            amount,
            base_currency.name.to_uppercase(),
            converted_amount,
            target_currency.name.to_uppercase()
        ))
        .await?;
    } else {
        let embed = discord::embed::create_embed()
            .title(format!(
                "{} | Currency Conversion",
                ctx.data().config.discord.name
            ))
            .description("Convert a currency from one to another.")
            .field(
                "Usage",
                "**<FROM>**: - What currency as a base\n \
            **<TO>**: - What currency to convert it to\n \
            **<AMOUNT>**: - The amount to convert",
                false,
            )
            .field(
                "Example",
                format!(
                    "`{}convert MYR IDR 5`\n \
            `{}convert IDR MYR 10000`",
                    ctx.prefix(),
                    ctx.prefix()
                ),
                false,
            );

        ctx.send(CreateReply::default().embed(embed)).await?;
    }

    Ok(())
}
