use super::prelude::*;

#[poise::command(prefix_command, track_edits, category = "Utility")]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Command to get help for"]
    #[rest]
    mut command: Option<String>,
) -> Result<(), Error> {
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
    let menu = serenity::CreateSelectMenu::new(
        "test",
        serenity::CreateSelectMenuKind::String {
            options: vec![
                serenity::CreateSelectMenuOption::new("FirstLabel", "FirstValue"),
                serenity::CreateSelectMenuOption::new("SecondLabel", "SecondValue"),
                serenity::CreateSelectMenuOption::new("ThirdLabel", "ThirdValue"),
            ],
        },
    );

    let handle = ctx
        .send(
            poise::CreateReply::default()
                .components(vec![serenity::CreateActionRow::SelectMenu(menu)]),
        )
        .await?;

    while let Some(selected) = serenity::collector::ComponentInteractionCollector::new(ctx)
        .filter(move |selected| selected.data.custom_id.starts_with("test"))
        .timeout(std::time::Duration::from_secs(3600))
        .await
    {
        if selected.data.custom_id == "test" {
            if let serenity::ComponentInteractionDataKind::StringSelect { values } =
                selected.data.kind
            {
                ctx.say(format!("You have selected {}", values[0])).await?;
            }
            break;
        }
    }

    handle.delete(ctx).await?;

    Ok(())
}
