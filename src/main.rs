use const_format::formatcp;

use serenity:: {
    async_trait,
    model::{channel::Message, 
            gateway::Ready},
    prelude::*,
    client::ClientBuilder,
};

const PREFIX: &str  = ".";
const HELP_COMMAND: &str = formatcp!("{}help", PREFIX);

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == HELP_COMMAND {
            if let Err(why) = msg.channel_id.say(&ctx.http,
                                     "Testicles").await {
                println!("Error sending said messag {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("bot start? {}", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = "ODQxMzYyMDg2NjY2OTYwOTI2.YJlpgQ.IlqXgMo4LNP0RIyycxURdu5voZ8";

    let mut client = ClientBuilder::new(&token, GatewayIntents::all())
                            .event_handler(Handler)
                            .await.expect("Err creating client");
    if let Err(why) = client.start().await {
        println!("Error: {:?}", why);
    }
}