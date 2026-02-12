use chrono::Duration;
use moete_core::{MoeteContext, MoeteError};
use moete_discord as discord;
use once_cell::sync::Lazy;
use plotters::prelude::*;
use plotters::style::Color as PlotColor;
use poise::CreateReply;
use serenity::all::{Color, CreateAttachment, CreateEmbedFooter};
use tokio::sync::Mutex;

static FMT_NUMBER: Lazy<human_format::Formatter> = Lazy::new(|| {
    let mut formatter = human_format::Formatter::new();
    formatter.with_decimals(2);
    formatter.with_separator("");
    formatter
});

static LAST_REFRESH: Lazy<Mutex<Option<std::time::Instant>>> = Lazy::new(|| Mutex::new(None));
const REFRESH_INTERVAL: std::time::Duration = std::time::Duration::from_secs(60 * 60 * 6); // 6 hours

/// Returns the date string in "YYYY-MM-DD" format for a given optional date else uses today's date.
fn get_date_string(date_opt: Option<chrono::DateTime<chrono::Local>>) -> String {
    if let Some(date) = date_opt {
        date.format("%Y-%m-%d").to_string()
    } else {
        chrono::Local::now().format("%Y-%m-%d").to_string()
    }
}

/// Returns a number in a human readable format.
fn readable_number(num: f64) -> String {
    match num {
        n if n >= 1.00 => FMT_NUMBER.format(num),
        _ => format!("{:.6}", num),
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
    subcommands("refresh"),
    aliases("cvt")
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
        let today = chrono::Local::now();
        let today_fmt = get_date_string(Some(today));

        let mut currency = ctx.data().currency.lock().await;
        let base_currency = currency
            .fetch(&base.to_lowercase(), Some(&today_fmt))
            .await?;
        let target_currency = currency
            .fetch(&target.to_lowercase(), Some(&today_fmt))
            .await?;

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
            .title(format!(
                "{} | Currency Conversion",
                ctx.data().config.discord.name
            ))
            .field(
                "Latest",
                format!(
                    "**{} {} = {} {}**",
                    readable_number(amount),
                    base_currency.name.to_uppercase(),
                    converted_amount,
                    target_currency.name.to_uppercase()
                ),
                false,
            )
            .footer(CreateEmbedFooter::new(""));

        // generate plots
        // fetches history over the past 7 days
        let mut rates = Vec::new();
        for days_ago in (0..7).rev() {
            let date = today - Duration::days(days_ago);
            let date_fmt = get_date_string(Some(date));
            if let Some(base_rate) = currency.fetch(&base_currency.name, Some(&date_fmt)).await? {
                if let Some(_) = currency
                    .fetch(&target_currency.name, Some(&date_fmt))
                    .await?
                {
                    if let Some(rate) = base_rate.get_rate_to(&target_currency.name) {
                        rates.push(rate);
                    }
                }
            }
        }

        let smallest_rate = rates.iter().cloned().fold(f64::INFINITY, |a, b| a.min(b));
        let largest_rate = rates
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, |a, b| a.max(b));
        let plot_id = ctx.id();

        let path = format!(
            ".tmp/charts/{}_{}-{}.png",
            plot_id, base_currency.name, target_currency.name
        );
        let plot_path = path.clone();

        // plot the rates
        tokio::task::spawn_blocking(
            move || -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
                let root = BitMapBackend::new(&plot_path, (1024, 768)).into_drawing_area();
                root.fill(&RGBColor(56, 58, 64))?;

                let (to_date, from_date) = (today, today - Duration::days(6));

                // padding
                let padding = (largest_rate - smallest_rate) * 0.05;
                let y_min = smallest_rate - padding;
                let y_max = largest_rate + padding;

                // use red/green based on rate change
                let rate_color = if rates.last().unwrap_or(&0.0) >= rates.first().unwrap_or(&0.0) {
                    RGBColor(0, 128, 255)
                } else {
                    RGBColor(255, 0, 0)
                };

                let mut chart = ChartBuilder::on(&root)
                    .margin(40)
                    .x_label_area_size(40)
                    .y_label_area_size(60)
                    .caption(
                        format!(
                            "{} → {} (Last 7 Days)",
                            base_currency.name.to_uppercase(),
                            target_currency.name.to_uppercase()
                        ),
                        ("sans-serif", 40).into_font().color(&WHITE),
                    )
                    .build_cartesian_2d(from_date..to_date, y_min..y_max)?;

                chart
                    .configure_mesh()
                    .x_labels(7)
                    .x_label_style(("sans-serif", 25).into_font().color(&WHITE))
                    .x_label_formatter(&|date| format!("{}", date.format("%m/%d")))
                    .y_labels(16)
                    .y_label_style(("sans-serif", 25).into_font().color(&WHITE))
                    .y_label_formatter(&|rate| readable_number(*rate))
                    .light_line_style(RGBAColor(1, 1, 1, 0.1))
                    .draw()?;

                let data: Vec<_> = rates
                    .iter()
                    .enumerate()
                    .map(|(i, rate)| {
                        let date = today - Duration::days((6 - i) as i64);
                        (date, *rate)
                    })
                    .collect();

                chart.draw_series(AreaSeries::new(data.clone(), y_min, &rate_color.mix(0.15)))?;

                chart.draw_series(LineSeries::new(data.clone(), &rate_color))?;

                chart.draw_series(
                    data.iter()
                        .map(|(date, rate)| Circle::new((*date, *rate), 5, rate_color.filled())),
                )?;

                root.present()?;
                Ok(())
            },
        )
        .await??;

        // attach the picture
        let bytes = std::fs::read(&path)?;
        let attachment = CreateAttachment::bytes(bytes, "greg.png");
        embed = embed.image("attachment://greg.png");

        ctx.send(
            CreateReply::default()
                .embed(embed)
                .attachment(attachment)
                .reply(true),
        )
        .await?;

        // should safe ot delete now
        std::fs::remove_file(&path)?;
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
