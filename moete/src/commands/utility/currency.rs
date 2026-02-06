use moete_core::{MoeteContext, MoeteError};
use moete_discord as discord;
use once_cell::sync::Lazy;
use poise::CreateReply;
use serenity::all::{Color, CreateEmbedFooter};
use tokio::sync::Mutex;

static FMT_NUMBER: Lazy<human_format::Formatter> = Lazy::new(|| {
    let mut formatter = human_format::Formatter::new();
    formatter.with_decimals(2);
    formatter.with_separator("");
    formatter
});

static FMT_NUMBER_SMALL: Lazy<human_format::Formatter> = Lazy::new(|| {
    let mut formatter = human_format::Formatter::new();
    formatter.with_decimals(16);
    formatter.with_separator("");
    formatter
});

static LAST_REFRESH: Lazy<Mutex<Option<std::time::Instant>>> = Lazy::new(|| Mutex::new(None));
const REFRESH_INTERVAL: std::time::Duration = std::time::Duration::from_secs(60 * 60 * 6); // 6 hours

/// Returns a number in a human readable format.
fn readable_number(num: f64) -> String {
    match num {
        n if n >= 0.01 => FMT_NUMBER.format(num),
        _ => FMT_NUMBER_SMALL.format(num),
    }
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
    let multiplier = match suffix.to_lowercase().as_str() {
        "" => 1.0,
        "k" => 1e3,
        "m" => 1e6,
        "b" => 1e9,
        "t" => 1e12,
        "q" => 1e15,
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
    // refresh if needed
    {
        let mut last_refresh = LAST_REFRESH.lock().await;
        match *last_refresh {
            Some(t) if t.elapsed() < REFRESH_INTERVAL => {},
            _ => {
                let mut currency = ctx.data().currency.lock().await;
                currency.refresh().await;

                *last_refresh = Some(std::time::Instant::now());
            },
        }
    }

    // NOTE: because some people are dumb, user might give commas in the amount
    // IE: 1,000 -> 1000
    let amount = amount.map(|a| a.replace(",", ""));

    // if all argument is valid
    if let Some(base) = base_currency
        && let Some(target) = target_currency
        && let Some(amount) = amount
        && let Some(amount) = parse_shorthand(&amount)
    {
        let mut currency = ctx.data().currency.lock().await;
        let base_currency = currency.fetch(&base.to_lowercase()).await?;
        let target_currency = currency.fetch(&target.to_lowercase()).await?;

        // default is green
        let mut embed = discord::embed::create_embed().color(Color::from_rgb(0, 255, 0));

        if base_currency.is_none() {
            embed = embed
                .description(format!(
                    "Base: Couldn't find a currency info with the name: {}",
                    base
                ))
                .color(Color::from_rgb(255, 0, 0));
            ctx.send(CreateReply::default().embed(embed).reply(true))
                .await?;
            return Ok(());
        }

        if target_currency.is_none() {
            embed = embed
                .description(format!(
                    "Target: Couldn't find a currency info with the name: {}",
                    target
                ))
                .color(Color::from_rgb(255, 0, 0));
            ctx.send(CreateReply::default().embed(embed).reply(true))
                .await?;
            return Ok(());
        }

        let base_currency = base_currency.unwrap(); // safe due to the is_none check above.
        let target_currency = target_currency.unwrap(); // same as above.
        let rate = base_currency.get_rate_to(&target_currency.name);

        if rate.is_none() {
            embed = embed
                .description(format!(
                    "No exchange rate found from {} to {}",
                    base_currency.name, target_currency.name
                ))
                .color(Color::from_rgb(255, 0, 0));
            ctx.send(CreateReply::default().embed(embed).reply(true))
                .await?;
            return Ok(());
        }

        let rate = rate.unwrap(); // safe due to the is_none check above.
        let converted_amount = readable_number(amount * rate);
        embed = embed
            .description(format!(
                "**{} {} = {} {}**",
                readable_number(amount),
                base_currency.name.to_uppercase(),
                converted_amount,
                target_currency.name.to_uppercase()
            ))
            .footer(CreateEmbedFooter::new(""));
        ctx.send(CreateReply::default().embed(embed).reply(true))
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
            )
            .field("Note", "```You can use shorthand notation for amounts (e.g., 1k = 1000, 2.5M = 2500000, etc.)\nIt only goes up to 1Q (quadrillion).```", false)
            .field(
                "Last updated",
                format!(
                    "```{} ago```",
                    humantime::format_duration(
                        LAST_REFRESH
                            .lock()
                            .await
                            .unwrap_or(std::time::Instant::now())
                            .elapsed()
                    )
                ),
                false,
            );
        ctx.send(CreateReply::default().embed(embed).reply(true))
            .await?;
    }

    Ok(())
}
