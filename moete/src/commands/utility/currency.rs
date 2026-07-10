use chrono::{
    Duration,
    TimeZone,
};
use moete_core::{
    MoeteContext,
    MoeteError,
};
use moete_discord as discord;
use once_cell::sync::Lazy;
use plotters::prelude::*;
use plotters::style::Color as PlotColor;
use poise::{
    CreateReply,
    ReplyHandle,
};
use serenity::all::{
    Color,
    CreateAttachment,
    CreateEmbedFooter,
};

static FMT_NUMBER: Lazy<human_format::Formatter> = Lazy::new(|| {
    let mut formatter = human_format::Formatter::new();
    formatter.with_decimals(4);
    formatter.with_separator("");
    formatter
});

/// Returns the date string in "YYYY-MM-DD" format for a given optional date else uses today's date.
fn get_date_string(
    date_opt: Option<chrono::DateTime<chrono::Local>>,
) -> String {
    if let Some(date) = date_opt {
        date.format("%Y-%m-%d").to_string()
    } else {
        chrono::Local::now().format("%Y-%m-%d").to_string()
    }
}

/// Returns a number in a human readable format.
fn readable_number(num: f64) -> String {
    let abs = num.abs();

    if abs >= 1.0 {
        return FMT_NUMBER.format(num);
    }

    // we handle small numbers with fixed decimal places, but we don't want to show too many zeros, so we trim them.
    if abs == 0.0 {
        return "0".to_string();
    }

    if abs >= 1e-4 {
        return format!("{:.8}", num)
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string();
    }

    format!("{:.4e}", num)
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

/// Converts a currency from one to another.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Utility",
    aliases("cvt")
)]
pub async fn convert(
    ctx: MoeteContext<'_>,
    #[description = "Base currency"] base_currency: Option<String>,
    #[description = "Target currency"] target_currency: Option<String>,
    #[description = "Amount to convert"] amount: Option<String>,
) -> Result<(), MoeteError> {
    // NOTE: we might get stuck for a while somewhere, would be nice to let the user know what's happening.
    let progress_update_msg: ReplyHandle = ctx
        .send(CreateReply::default().content("Loading...").reply(true))
        .await?;

    // NOTE: because some people are dumb, user might give commas in the amount
    // IE: 1,000 -> 1000
    // also default to 1 if no amount is given
    let amount = amount.map(|a| a.replace(",", "")).unwrap_or("1".to_string());

    // if all argument is valid
    if let Some(base) = base_currency
        && let Some(target) = target_currency
        && let Some(amount) = parse_shorthand(&amount)
    {
        let mut currency = ctx.data().currency.lock().await;

        let today = chrono::Local::now();
        let tomorrow = today + Duration::days(1); // QUIRKS: needs to offset this by a day
        let a_week_ago = today - Duration::days(7);

        let today_str = get_date_string(Some(today));
        let tomorrow_str = get_date_string(Some(tomorrow));
        let a_week_ago_str = get_date_string(Some(a_week_ago));

        // default is green
        let mut embed =
            discord::embed::create_embed().color(Color::from_rgb(0, 255, 0));

        if let Ok((rates, level_of_accuracy)) = currency
            .fetch_range(&base, &target, &a_week_ago_str, &tomorrow_str)
            .await
        {
            if let Some(latest_rate) = rates.get(&get_date_string(Some(today)))
            {
                // NOTE: optimally if we can get the latest rate, that should mean get all the rates.
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
                            base.to_uppercase(),
                            readable_number(amount * latest_rate),
                            target.to_uppercase(),
                        ),
                        true,
                    )
                    .field(
                        "Accuracy",
                        format!("**{}**", level_of_accuracy),
                        true,
                    )
                    .footer(CreateEmbedFooter::new("The accuracy might be off by a few margins due to the data source."));

                // build the data

                let mut data: Vec<_> = rates
                    .iter()
                    .map(|(date_str, rate)| {
                        let date = chrono::NaiveDate::parse_from_str(
                            date_str, "%Y-%m-%d",
                        )
                        .unwrap_or_else(|_| today.naive_local().date());

                        let datetime = chrono::Local
                            .from_local_datetime(
                                &date.and_hms_opt(0, 0, 0).unwrap(),
                            )
                            .unwrap();

                        (datetime, *rate)
                    })
                    .collect();

                data.sort_by_key(|(datetime, _)| *datetime);

                // graph hell here we go
                let smallest_rate = rates
                    .values()
                    .cloned()
                    .fold(f64::INFINITY, |a, b| a.min(b));
                let largest_rate = rates
                    .values()
                    .cloned()
                    .fold(f64::NEG_INFINITY, |a, b| a.max(b));

                let plot_id = ctx.id();

                let path =
                    format!(".tmp/charts/{}_{}-{}.png", plot_id, base, target);
                let plot_path = path.clone();

                tokio::task::spawn_blocking( move || -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
                      let root = BitMapBackend::new(&plot_path, (1024, 768))
                    .into_drawing_area();
                root.fill(&RGBColor(56, 58, 64))?;

                // chart fitting
                let chart_start = data
                    .first()
                    .map(|(date, _)| *date)
                    .unwrap_or(a_week_ago);

                let chart_end = data
                    .last()
                    .map(|(date, _)| *date)
                    .unwrap_or(today);


                // padding
                let padding = (largest_rate - smallest_rate) * 0.05;
                let y_min = smallest_rate - padding;
                let y_max = largest_rate + padding;

                // use red/green based on rate change
                let rate_color =  if rates.get(&today_str).unwrap_or(&0.0)
                    >= rates.get(&a_week_ago_str).unwrap_or(&0.0)
                {
                    RGBColor(0, 128, 255)
                } else {
                    RGBColor(255, 0, 0)
                };

                let mut chart = ChartBuilder::on(&root)
                    .margin(40)
                    .x_label_area_size(40)
                    .y_label_area_size(70)
                    .caption(
                        format!(
                            "{} → {} (Last 7 Days)",
                            base.to_uppercase(),
                            target.to_uppercase()
                        ),
                        ("sans-serif", 40).into_font().color(&WHITE),
                    )
                    .build_cartesian_2d(chart_start..chart_end, y_min..y_max)?;

                chart
                    .configure_mesh()
                    .x_labels(7)
                    .x_label_style(("sans-serif", 25).into_font().color(&WHITE))
                    .x_label_formatter(&|date| {
                        format!("{}", date.format("%m/%d"))
                    })
                    .y_labels(16)
                    .y_label_style(("sans-serif", 25).into_font().color(&WHITE))
                    .y_label_formatter(&|rate| readable_number(*rate))
                    .light_line_style(RGBAColor(1, 1, 1, 0.1))
                    .draw()?;

                chart
                    .draw_series(LineSeries::new(data.clone(), &rate_color))?;

                chart.draw_series(AreaSeries::new(
                    data.clone(),
                    y_min,
                    rate_color.mix(0.15),
                ))?;

                chart.draw_series(data.iter().map(|(date, rate)| {
                    Circle::new((*date, *rate), 5, rate_color.filled())
                }))?;
                  root.present()?;
                  Ok(())
                },).await??;

                // attach the picture
                let bytes = std::fs::read(&path)?;
                let attachment = CreateAttachment::bytes(bytes, "greg.png");
                embed = embed.image("attachment://greg.png");

                progress_update_msg
                    .edit(
                        ctx,
                        CreateReply::default()
                            .content("")
                            .embed(embed)
                            .attachment(attachment)
                            .reply(true),
                    )
                    .await?;

                // should safe ot delete now
                std::fs::remove_file(&path)?;
            } else {
                embed = embed
                    .description(format!(
                        "No exchange rate data available for {} to {} on {}.",
                        base.to_uppercase(),
                        target.to_uppercase(),
                        get_date_string(Some(today))
                    ))
                    .color(Color::from_rgb(255, 0, 0));
                progress_update_msg
                    .edit(
                        ctx,
                        CreateReply::default()
                            .content("")
                            .embed(embed)
                            .reply(true),
                    )
                    .await?;
                return Ok(());
            }
        } else {
            embed = embed
                .description(
                    "Something went wrong while fetching rates. Please try again later.",
                )
                .color(Color::from_rgb(255, 0, 0));
            progress_update_msg
                .edit(
                    ctx,
                    CreateReply::default().content("").embed(embed).reply(true),
                )
                .await?;
            return Ok(());
        }
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
            ;
        ctx.send(CreateReply::default().embed(embed).reply(true)).await?;
    }

    Ok(())
}
