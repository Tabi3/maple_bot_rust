use build_app_comm::build_mfunc_app;
use build_comms::*;
use serenity::async_trait;
use serenity::builder::CreateInteractionResponseData;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::interactions::application_command::{
    ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
    ApplicationCommandOptionType,
};
use serenity::model::interactions::{Interaction, InteractionResponseType};
use serenity::prelude::*;

use crate::build_app_comm::build_froot_app;

struct Handler;

mod test_module;
mod build_comms;
mod build_app_comm;


#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "id" => build_id_func(&command),
                "ping" => build_ping_func(),
                "mfunc" => build_mfunc_func(&command),
                "froot" => build_froot_func(&command),
                _ => build_not_impl_func(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(content)
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
            commands
                .create_application_command(|command| {
                    command
                        .name("id")
                        .description("An id command")
                        .create_option(|option| {
                            option
                                .name("id")
                                .description("The user to lookup")
                                .kind(ApplicationCommandOptionType::User)
                                .required(true)
                        })
                })
                .create_application_command(|command| {
                    command.name("ping").description("Ping command")
                })
                .create_application_command(build_mfunc_app())
                .create_application_command(build_froot_app())
        })
        .await;
        println!("{:?}", commands);
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
