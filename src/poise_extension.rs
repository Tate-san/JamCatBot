use crate::{messages::MessageParams, Message};
use poise::{CreateReply, ReplyHandle};
use serenity::all::{CreateEmbed, VoiceState};
use songbird::Call;
use std::{future::Future, sync::Arc};
use tokio::sync::Mutex;

pub trait ContextExtension<'a> {
    fn get_bot_call(&'a self) -> impl Future<Output = anyhow::Result<Arc<Mutex<Call>>>>;

    fn get_author_voice_state(&'a self) -> impl Future<Output = Option<VoiceState>>;

    fn send_message_params(
        &'a self,
        params: MessageParams,
    ) -> impl Future<Output = anyhow::Result<ReplyHandle<'a>>>;

    fn send_message(
        &'a self,
        message: Message,
    ) -> impl Future<Output = anyhow::Result<ReplyHandle<'a>>>;

    fn reply_message(
        &'a self,
        message: Message,
    ) -> impl Future<Output = anyhow::Result<ReplyHandle<'a>>>;

    fn send_embed(
        &'a self,
        embed: CreateEmbed,
    ) -> impl Future<Output = anyhow::Result<ReplyHandle<'a>>>;

    fn reply_embed(
        &'a self,
        embed: CreateEmbed,
    ) -> impl Future<Output = anyhow::Result<ReplyHandle<'a>>>;
}

impl<'a> ContextExtension<'a> for crate::types::Context<'a> {
    /// Gets bot's call in current group.
    async fn get_bot_call(&'a self) -> anyhow::Result<Arc<Mutex<Call>>> {
        let guild_id = self
            .guild_id()
            .ok_or(anyhow::anyhow!("You have to be in a guild"))?;

        let manager = self.data().songbird.clone();

        manager
            .get(guild_id)
            .ok_or(anyhow::anyhow!("Bot is not in a call"))
    }

    async fn get_author_voice_state(&'a self) -> Option<VoiceState> {
        let guild = self
            .guild()
            .expect("Should be called only in guild")
            .clone();

        if let Some(state) = guild.voice_states.get(&self.author().id) {
            Some(state.clone())
        } else {
            None
        }
    }

    async fn send_message_params(
        &'a self,
        params: MessageParams,
    ) -> anyhow::Result<ReplyHandle<'a>> {
        let mut reply = CreateReply::default();
        let text_content = params.message.to_string();

        if params.as_embed {
            reply = reply.embed(
                params.embed.unwrap_or(
                    CreateEmbed::new()
                        .description(text_content)
                        .colour(params.color),
                ),
            );
        } else {
            reply = reply.content(text_content);
        }

        reply = reply.reply(params.reply).ephemeral(params.ephemeral);

        match self.send(reply).await {
            Ok(handle) => Ok(handle),
            Err(e) => {
                tracing::error!("{e}");
                Err(e.into())
            }
        }
    }

    async fn send_message(&'a self, message: Message) -> anyhow::Result<ReplyHandle<'a>> {
        self.send_message_params(message.into()).await
    }

    async fn reply_message(&'a self, message: Message) -> anyhow::Result<ReplyHandle<'a>> {
        let params: MessageParams = message.into();
        let params = params.with_reply(true);
        self.send_message_params(params).await
    }

    async fn send_embed(&'a self, embed: CreateEmbed) -> anyhow::Result<ReplyHandle<'a>> {
        let params = MessageParams::default()
            .with_embed(Some(embed))
            .with_reply(false);
        self.send_message_params(params).await
    }

    async fn reply_embed(&'a self, embed: CreateEmbed) -> anyhow::Result<ReplyHandle<'a>> {
        let params = MessageParams::default()
            .with_embed(Some(embed))
            .with_reply(true);
        self.send_message_params(params).await
    }
}
