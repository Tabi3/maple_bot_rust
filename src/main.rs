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

struct Handler;

fn build_ping_func() -> Box<
    dyn for<'b, 'c> Fn(
            &'b mut CreateInteractionResponseData<'c>,
        ) -> &'b mut CreateInteractionResponseData<'c>
        + Sync
        + Send,
> {
    return Box::new(
        move |message: &mut CreateInteractionResponseData| -> &mut CreateInteractionResponseData {
            message
                .content("Ping Pong")
                .embed(|e| e.field("Ping", "Pong", true).field("Pong", "Ping", true))
        },
    );
}
fn build_id_func(
    command: &ApplicationCommandInteraction,
) -> Box<
    dyn for<'b, 'c> Fn(
            &'b mut CreateInteractionResponseData<'c>,
        ) -> &'b mut CreateInteractionResponseData<'c>
        + Sync
        + Send
        + '_,
> {
    let optionsvar = &*command
        .data
        .options
        .get(0)
        .expect("jfsk")
        .resolved
        .as_ref()
        .expect("yes");
    Box::new(
        move |message: &mut CreateInteractionResponseData| -> &mut CreateInteractionResponseData {
            if let ApplicationCommandInteractionDataOptionValue::User(user, _member) =
                &optionsvar.clone()
            {
                message.content(format!("{}'s id is {}", user.tag(), user.id))
            } else {
                message.content("Please provide a valid user".to_string())
            }
        },
    )
}

fn build_not_impl_func() -> Box<
    dyn for<'b, 'c> Fn(
            &'b mut CreateInteractionResponseData<'c>,
        ) -> &'b mut CreateInteractionResponseData<'c>
        + Sync
        + Send,
> {
    return Box::new(
        move |message: &mut CreateInteractionResponseData| -> &mut CreateInteractionResponseData {
            message.content("not implemented :(")
        },
    );
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "id" => build_id_func(&command),
                "ping" => build_ping_func(),
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

        let _commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
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
