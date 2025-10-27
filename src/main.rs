use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::all::{CreateEmbed, CreateMessage, GuildId, Member, User};
use serenity::model::colour::Colour;
use serenity::model::id::ChannelId;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, message: Message) {
        if message.channel_id.get() == 1431888612596912228 {
            let mut should_react = false;

            if !message.attachments.is_empty() {
                for attachment in &message.attachments {
                    if let Some(content_type) = &attachment.content_type {
                        if content_type.starts_with("image/") || content_type.starts_with("video/") {
                            should_react = true;

                            break;
                        }
                    }
                }
            }

            if !should_react && !message.embeds.is_empty() {
                for embed in &message.embeds {
                    if embed.image.is_some() || embed.video.is_some() {
                        should_react = true;
                        break;
                    }
                }
            }

            if should_react {
                for emoji in ['👍', '👎'] {
                    if let Err(why) = message.react(&context.http, emoji).await {
                        println!("Error reacting to message: {why:?}");
                    }
                } 
            }
        }
    }

    async fn guild_member_addition(&self, ctx: Context, member: Member) {
        let channel_id = ChannelId::new(1411337104042233908);
        let embed = CreateEmbed::new()
            .title(format!("{}", member.user.name))
            .description(format!("Just joined **Tahir's Tactical Crew**!\nThere are now **{}** members.", ctx.cache.user_count()))
            .colour(Colour::from_rgb(77, 255, 122))
            .thumbnail(member.user.face());

        let builder = CreateMessage::new().tts(true).embed(embed);

        if let Err(why) = channel_id
            .send_message(&ctx.http, builder)
            .await
        {
            println!("Error sending message: {why:?}");
        }
    }
    
    async fn guild_member_removal(&self, ctx: Context, _guild_id: GuildId, user: User,_member_dataa: Option<Member>) {
        let channel_id = ChannelId::new(1411337104042233908);
        let embed = CreateEmbed::new()
            .title(format!("{}", user.name))
            .description(format!("Just left **Tahir's Tactical Crew**.\nThere are now **{}** members.", ctx.cache.user_count()))
            .colour(Colour::from_rgb(255, 35, 35))
            .thumbnail(user.face());

        let builder = CreateMessage::new().tts(true).embed(embed);

        if let Err(why) = channel_id
            .send_message(&ctx.http, builder)
            .await
        {
            println!("Error sending message: {why:?}");
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS;

    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
