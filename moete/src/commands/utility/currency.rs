use lazy_static::lazy_static;
use poise::CreateReply;

use moete_core::{MoeteContext, MoeteError};
use moete_discord as discord;

lazy_static! {
    static ref FMT_NUMBER: human_format::Formatter = {
        let mut formatter = human_format::Formatter::new();
        formatter.with_decimals(2);
        formatter.with_separator("");
        formatter
    };
}

/// Returns a number in a human readable format.
fn readable_number(num: f64) -> String {
    FMT_NUMBER.format(num)
}

/// Parses a shorthand number (e.g., 1k, 2.5M) into a f64.
fn parse_shorthand(input: &str) -> Option<f64> {
    let input = input.trim().to_lowercase();
    if input.is_empty() {
        return None;
    }

    // Extract numeric part and optional suffix
    let (num_str, suffix) = input
        .chars()
        .partition::<String, _>(|c| c.is_ascii_digit() || *c == '.');

    let num: f64 = num_str.parse().ok()?;
    let multiplier = match suffix.as_str() {
        "" => 1.0,
        "k" => 1_000.0,
        "m" => 1_000_000.0,
        "b" => 1_000_000_000.0,
        "t" => 1_000_000_000_000.0,
        _ => return None,
    };

    Some(num * multiplier)
}

/// Refreshes the currencies information.
#[poise::command(prefix_command, slash_command, category = "Utility")]
pub async fn refresh(ctx: MoeteContext<'_>) -> Result<(), MoeteError> {
    let mut currency = ctx.data().currency.lock().await;
    currency.refresh().await;
    ctx.say("Currency rates refreshed").await?;
    Ok(())
}

/// Converts a currency from one to another.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Utility",
    subcommands("refresh")
)]
pub async fn convert(
    ctx: MoeteContext<'_>,
    #[description = "Base currency"] base_currency: Option<String>,
    #[description = "Target currency"] target_currency: Option<String>,
    #[description = "Amount to convert"] amount: Option<String>,
) -> Result<(), MoeteError> {
    // fuck it, we balling
    // randomly clear the cache here so that the data stays up to date.
    if rand::random_range(0..100) < 10 {
        let mut currency = ctx.data().currency.lock().await;
        currency.refresh().await;
    }

    // if all argument is valid
    if let Some(base) = base_currency
        && let Some(target) = target_currency
        && let Some(amount) = amount
        && let Some(amount) = parse_shorthand(&amount)
    {
        let mut currency = ctx.data().currency.lock().await;
        let base_currency = currency.fetch(&base.to_lowercase()).await?;
        let target_currency = currency.fetch(&target.to_lowercase()).await?;

        if base_currency.is_none() {
            ctx.say(format!(
                "Couldn't find a currency info with the name: {}",
                base
            ))
            .await?;
            return Ok(());
        }

        if target_currency.is_none() {
            ctx.say(format!(
                "Couldn't find a currency info with the name: {}",
                target
            ))
            .await?;
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
        let converted_amount = readable_number(amount * rate);
        ctx.say(format!(
            "{} {} = {} {}",
            readable_number(amount),
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
                "```<FROM>: - What currency as a base\n\
            <TO>: - What currency to convert it to\n\
            <AMOUNT>: - The amount to convert```",
                false,
            )
            .field(
                "1 USD to other currencies",
                {
                    let mut currency = ctx.data().currency.lock().await;
                    let base_currency = currency.fetch("usd").await?;

                    if let Some(base_currency) = base_currency {
                        format!(
                            "```{}```",
                            currency
                                .rates
                                .iter()
                                .filter(|(code, _)| *code != "usd")
                                .map(|(code, rate)| {
                                    format!(
                                        "{} | {} | {}",
                                        rate.date,
                                        code.to_uppercase(),
                                        readable_number(
                                            base_currency.get_rate_to(code).unwrap_or(0.0)
                                        )
                                    )
                                })
                                .collect::<Vec<String>>()
                                .join("\n")
                        )
                    } else {
                        "No currency loaded yet".to_string()
                    }
                },
                false,
            )
            .field(
                "Example",
                format!(
                    "`{}convert MYR IDR {}`\n \
                `{}convert IDR MYR {}`",
                    ctx.prefix(),
                    readable_number(100.0_f64),
                    ctx.prefix(),
                    readable_number(10000.0_f64)
                ),
                false,
            );
        ctx.send(CreateReply::default().embed(embed)).await?;
    }

    Ok(())
}
