use poise::CreateReply;

use moete_core::{MoeteContext, MoeteError};
use moete_discord as discord;

/// Send host machine information.
#[poise::command(slash_command, prefix_command, category = "Utility")]
pub async fn machine(ctx: MoeteContext<'_>) -> Result<(), MoeteError> {
    let memory = sys_info::mem_info()?;

    let embed = discord::embed::create_embed()
        .title(format!("{} | Machine Info", ctx.data().config.discord.name))
        .field(
            "Main",
            format!(
                "**Operating System**: `{}` [Release: `{}`]\n\
                **Host name**: `{}`\n\
                **Machine**: `{}`\n",
                sys_info::os_type().unwrap_or_default(),
                sys_info::os_release().unwrap_or_default(),
                sys_info::hostname().unwrap_or_default(),
                std::env::consts::ARCH,
            ),
            false,
        )
        .field(
            "Memory",
            format!(
                "**RAM**: `{:.2}` / `{:.2}` GB \n\
                **Swap**: `{:.2}` / `{:.2}` GB",
                (memory.total - memory.avail) as f64 / 1e+6,
                memory.total as f64 / 1e+6,
                (memory.swap_total - memory.swap_free) as f64 / 1e+6,
                memory.swap_total as f64 / 1e+6
            ),
            false,
        );

    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}
