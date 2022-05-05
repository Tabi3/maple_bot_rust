use std::env;

use serenity::async_trait;
use serenity::builder::CreateInteractionResponseData;
use serenity::model::channel::Embed;
use serenity::model::gateway::Ready;
use serenity::model::id::{ChannelId, GuildId};
use serenity::model::interactions::application_command::{
    ApplicationCommand, ApplicationCommandInteractionDataOptionValue, ApplicationCommandOptionType,
};
use serenity::model::interactions::{ping, Interaction, InteractionResponseType};
use serenity::prelude::*;

struct Handler;

fn ping_func<'b, 'c>(
    message: &'b mut CreateInteractionResponseData<'c>,
) -> &'b mut CreateInteractionResponseData<'c> {
    return message.embed(|e| {
        e.field("Ping?", "Pong!", true)
            .field("Ping-Pong Result", "Success", true)
    });
}

fn not_impl_func<'b, 'c>(
    message: &'b mut CreateInteractionResponseData<'c>,
) -> &'b mut CreateInteractionResponseData<'c> {
    return message.content("not implemented :(");
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "ping" => ping_func,
                _ => not_impl_func,
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(&content)
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(906461867310977075);

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands.create_application_command(|command| {
                command.name("ping").description("A ping command")
            })
        })
        .await;
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = "ODQxMzYyMDg2NjY2OTYwOTI2.YJlpgQ.4w3mHvPHOYxwMKJwCDQh3eg2WQ8";

    // Build our client.
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
