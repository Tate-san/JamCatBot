use crate::prelude::*;

#[poise::command(prefix_command, track_edits, aliases("h"), category = "Utility")]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Command to get help for"]
    #[rest]
    mut command: Option<String>,
) -> Result<(), BotError> {
    // This makes it possible to just make `help` a subcommand of any command
    // `/fruit help` turns into `/help fruit`
    // `/fruit help apple` turns into `/help fruit apple`
    if ctx.invoked_command_name() != "help" {
        command = match command {
            Some(c) => Some(format!("{} {}", ctx.invoked_command_name(), c)),
            None => Some(ctx.invoked_command_name().to_string()),
        };
    }

    let prefix = ctx.prefix();
    let extra_text_at_bottom = &format!(
        "\
    Type `{prefix}help command` for more info on a command.
    You can edit your `{prefix}help` message to the bot and the bot will edit its response."
    );

    let config = poise::samples::HelpConfiguration {
        show_subcommands: true,
        show_context_menu_commands: true,
        ephemeral: true,
        extra_text_at_bottom,

        ..Default::default()
    };
    poise::builtins::help(ctx, command.as_deref(), config).await?;
    Ok(())
}

#[poise::command(prefix_command, category = "Utility")]
pub async fn test(ctx: Context<'_>) -> Result<(), Error> {
    let _ = ctx.send_message(Message::Success("Works".into())).await;
    Ok(())
}

