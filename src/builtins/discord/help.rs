//! Contains the built-in help command and surrounding infrastructure

use crate::Context;
use crate::builtins::discord::embed;
use crate::builtins::discord::poise_builtins;
use crate::core;
use poise::{CreateReply, serenity_prelude as serenity};
use std::fmt::Write as _;

/// Optional configuration for how the help message from [`help()`] looks
pub struct HelpConfiguration<'a> {
    /// Extra text displayed at the bottom of your message. Can be used for help and tips specific
    /// to your bot
    pub extra_text_at_bottom: &'a str,
    /// Whether to make the response ephemeral if possible. Can be nice to reduce clutter
    pub ephemeral: bool,
    /// Whether to list context menu commands as well
    pub show_context_menu_commands: bool,
    /// Whether to include [`poise::Command::description`] (above [`poise::Command::help_text`]).
    pub include_description: bool,
    #[doc(hidden)]
    pub __non_exhaustive: (),
}

impl Default for HelpConfiguration<'_> {
    fn default() -> Self {
        Self {
            extra_text_at_bottom: "",
            ephemeral: true,
            show_context_menu_commands: false,
            include_description: true,
            __non_exhaustive: (),
        }
    }
}

/// Convenience function to align descriptions behind commands
struct TwoColumnList(Vec<(String, Option<String>)>);

impl TwoColumnList {
    /// Creates a new [`TwoColumnList`]
    fn new() -> Self {
        Self(Vec::new())
    }

    /// Add a line that needs the padding between the columns
    fn push_two_colums(&mut self, command: String, description: String) {
        self.0.push((command, Some(description)));
    }

    /// Convert the list into a string with aligned descriptions
    fn into_string(self) -> String {
        let longest_command = self
            .0
            .iter()
            .filter_map(|(command, description)| {
                if description.is_some() {
                    Some(command.len())
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(0);
        let mut text = String::new();
        for (command, description) in self.0 {
            if let Some(description) = description {
                let padding = " ".repeat(longest_command - command.len() + 3);
                writeln!(text, "{}{}{}", command, padding, description).unwrap();
            } else {
                writeln!(text, "{}", command).unwrap();
            }
        }
        text
    }
}

/// Get the prefix from options
async fn get_prefix_from_options<U, E>(ctx: poise::Context<'_, U, E>) -> Option<String> {
    let options = &ctx.framework().options().prefix_options;
    match &options.prefix {
        Some(fixed_prefix) => Some(fixed_prefix.clone()),
        None => match options.dynamic_prefix {
            Some(dynamic_prefix_callback) => {
                match dynamic_prefix_callback(poise::PartialContext::from(ctx)).await {
                    Ok(Some(dynamic_prefix)) => Some(dynamic_prefix),
                    _ => None,
                }
            }
            None => None,
        },
    }
}

/// Format context menu command name
fn format_context_menu_name<U, E>(command: &poise::Command<U, E>) -> Option<String> {
    let kind = match command.context_menu_action {
        Some(poise::ContextMenuCommandAction::User(_)) => "user",
        Some(poise::ContextMenuCommandAction::Message(_)) => "message",
        Some(poise::ContextMenuCommandAction::__NonExhaustive) => unreachable!(),
        None => return None,
    };
    Some(format!(
        "{} (on {})",
        command
            .context_menu_name
            .as_deref()
            .unwrap_or(&command.name),
        kind
    ))
}

/// Code for printing help of a specific command (e.g. `~help my_command`)
async fn help_single_command(
    ctx: Context<'_>,
    command_name: &str,
    config: HelpConfiguration<'_>,
) -> Result<(), serenity::Error> {
    let commands = &ctx.framework().options().commands;
    // Try interpret the command name as a context menu command first
    let mut command = commands.iter().find(|command| {
        if let Some(context_menu_name) = &command.context_menu_name {
            if context_menu_name.eq_ignore_ascii_case(command_name) {
                return true;
            }
        }
        false
    });
    // Then interpret command name as a normal command (possibly nested subcommand)
    if command.is_none() {
        if let Some((c, _, _)) = poise::find_command(commands, command_name, true, &mut vec![]) {
            command = Some(c);
        }
    }

    if let Some(command) = command {
        let mut invocations = Vec::new();
        let mut subprefix = None;
        if command.slash_action.is_some() {
            invocations.push(format!("`/{}`", command.name));
            subprefix = Some(format!("  /{}", command.name));
        }
        if command.prefix_action.is_some() {
            let prefix = match get_prefix_from_options(ctx).await {
                Some(prefix) => prefix,
                // None can happen if the prefix is dynamic, and the callback
                // fails due to help being invoked with slash or context menu
                // commands. Not sure there's a better way to handle this.
                None => String::from("<prefix>"),
            };
            invocations.push(format!("`{}{}`", prefix, command.name));
            if subprefix.is_none() {
                subprefix = Some(format!("  {}{}", prefix, command.name));
            }
        }
        if command.context_menu_name.is_some() && command.context_menu_action.is_some() {
            // Since command.context_menu_action is Some, this unwrap is safe
            invocations.push(format_context_menu_name(command).unwrap());
            if subprefix.is_none() {
                subprefix = Some(String::from("  "));
            }
        }
        // At least one of the three if blocks should have triggered
        assert!(subprefix.is_some());
        assert!(!invocations.is_empty());

        let description = match (&command.description, &command.help_text) {
            (Some(description), Some(help_text)) => {
                if config.include_description {
                    format!("{}\n\n{}", description, help_text)
                } else {
                    help_text.clone()
                }
            }
            (Some(description), None) => description.to_owned(),
            (None, Some(help_text)) => help_text.clone(),
            (None, None) => "No help available".to_string(),
        };

        // Fill in embed
        let aliases = if command.aliases.is_empty() {
            "None".to_string()
        } else {
            command.aliases.join(", ")
        };

        // Everything should be fine now
        // Construct embed for the help message.
        let data: &core::State = ctx.data();
        let mut embed =
            embed::create_embed().title(format!("{} | {}", data.config.discord.name, command_name));

        embed = embed.field(
            "Category",
            command.category.clone().unwrap_or("Undefined".to_string()),
            true,
        );
        embed = embed.field("Aliases", format!("`{}`", aliases), true);
        embed = embed.description(description);

        // Parameters
        if !command.parameters.is_empty() {
            let mut parameterlist = TwoColumnList::new();
            for parameter in &command.parameters {
                let name = parameter.name.clone();
                let description = parameter.description.as_deref().unwrap_or("");
                let description = format!(
                    "({}) {}",
                    if parameter.required {
                        "required"
                    } else {
                        "optional"
                    },
                    description,
                );
                parameterlist.push_two_colums(name, description);
            }
            embed = embed.field(
                "Parameters",
                format!("```\n{}```", parameterlist.into_string()),
                false,
            );
        }

        // Subcommands
        if !command.subcommands.is_empty() {
            let mut commandlist = TwoColumnList::new();
            preformat_subcommands(
                &mut commandlist,
                command,
                &subprefix.unwrap_or_else(|| String::from("  ")),
            );
            embed = embed.field(
                "Subcommands",
                format!("```\n{}```", commandlist.into_string()),
                false,
            );
        }

        let reply = CreateReply::default()
            .embed(embed)
            .ephemeral(config.ephemeral)
            .reply(true);

        ctx.send(reply).await?;
    } else {
        ctx.reply(format!("Could not find command named `{}`", command_name))
            .await?;
    }

    Ok(())
}

/// Recursively formats all subcommands
fn preformat_subcommands<U, E>(
    commands: &mut TwoColumnList,
    command: &poise::Command<U, E>,
    prefix: &str,
) {
    let as_context_command = command.slash_action.is_none() && command.prefix_action.is_none();
    for subcommand in &command.subcommands {
        let command = if as_context_command {
            let name = format_context_menu_name(subcommand);
            if name.is_none() {
                continue;
            };
            name.unwrap()
        } else {
            format!("{} {}", prefix, subcommand.name)
        };
        let description = subcommand.description.as_deref().unwrap_or("").to_string();
        commands.push_two_colums(command, description);
        // We could recurse here, but things can get cluttered quickly.
        // Instead, we show (using this function) subsubcommands when
        // the user asks for help on the subcommand.
    }
}

/// Create help text for `help_all_commands`
///
/// This is a separate function so we can have tests for it
async fn generate_all_commands(
    ctx: Context<'_>,
    config: &HelpConfiguration<'_>,
) -> Result<serenity::CreateEmbed, serenity::Error> {
    let data: &core::State = ctx.data();
    let mut embed =
        embed::create_embed().title(format!("{} | {}", data.config.discord.name, "Help"));

    let mut categories =
        poise_builtins::util::OrderedMap::<Option<&str>, Vec<&poise::Command<_, _>>>::new();
    for cmd in &ctx.framework().options().commands {
        categories
            .get_or_insert_with(cmd.category.as_deref(), Vec::new)
            .push(cmd);
    }

    let mut menu = String::from("```\n");

    for (category_name, commands) in categories {
        let commands = commands
            .into_iter()
            .filter(|cmd| {
                !cmd.hide_in_help && (cmd.prefix_action.is_some() || cmd.slash_action.is_some())
            })
            .collect::<Vec<_>>();
        if commands.is_empty() {
            continue;
        }

        let mut text = String::new();
        for command in commands {
            text += format!(
                "**{}** - {}\n",
                command.name,
                command.description.as_deref().unwrap_or("None")
            )
            .as_str();
        }

        embed = embed.field(
            "",
            format!("__**{}**__\n{}", category_name.unwrap_or("Commands"), text),
            false,
        );
    }

    if config.show_context_menu_commands {
        menu += "\nContext menu commands:\n";

        for command in &ctx.framework().options().commands {
            let name = format_context_menu_name(command);
            if name.is_none() {
                continue;
            };
            let _ = writeln!(menu, "  {}", name.unwrap());
        }
    }

    menu += "\n";
    menu += config.extra_text_at_bottom;
    menu += "\n```";

    Ok(embed)
}

/// Code for printing an overview of all commands (e.g. `~help`)
async fn help_all_commands(
    ctx: Context<'_>,
    config: HelpConfiguration<'_>,
) -> Result<(), serenity::Error> {
    let menu = generate_all_commands(ctx, &config).await?;
    let reply = CreateReply::default()
        .embed(menu)
        .ephemeral(config.ephemeral)
        .reply(true);

    ctx.send(reply).await?;
    Ok(())
}

/// A help command that outputs text in a code block, groups commands by categories, and annotates
/// commands with a slash if they exist as slash commands.
pub async fn help(
    ctx: Context<'_>,
    command: Option<&str>,
    config: HelpConfiguration<'_>,
) -> Result<(), serenity::Error> {
    match command {
        Some(command) => help_single_command(ctx, command, config).await,
        None => help_all_commands(ctx, config).await,
    }
}
