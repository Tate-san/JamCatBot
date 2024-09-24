use super::prelude::*;

pub async fn error(ctx: &Context<'_>, message: impl ToString) {
    check_msg(
        ctx.send(
            poise::CreateReply::default().embed(
                serenity::CreateEmbed::new()
                    .color(serenity::Colour::RED)
                    .description(format!("❗ **{}**", message.to_string())),
            ),
        )
        .await,
    );
}

pub async fn success(ctx: &Context<'_>, message: impl ToString) {
    check_msg(
        ctx.send(
            poise::CreateReply::default().embed(
                serenity::CreateEmbed::new()
                    .color(serenity::Colour::DARK_GREEN)
                    .description(format!("✅ **{}**", message.to_string())),
            ),
        )
        .await,
    );
}
