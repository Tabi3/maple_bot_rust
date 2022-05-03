use const_format::formatcp;

use serenity::{
    async_trait,
    client::*,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

const PREFIX: &str = ".";
const HELP_COMMAND: &str = formatcp!("{}help", PREFIX);
const DICK_COMMAND: &str = formatcp!("{}dick", PREFIX);
const NROOT_COMMAND: &str = formatcp!("{}nroot", PREFIX);

struct Handler;
mod test_module;
use test_module::*;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        println!("{} <@{}>:{}", msg.author.name, msg.channel_id, msg.content);
        if msg.content == HELP_COMMAND {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Testicle").await {
                println!("Error sending said message {:?}", why);
            }
        }
        if msg.content == DICK_COMMAND {
            if let Err(why) = msg.channel_id.say(&ctx.http, "8======D").await {
                println!("Error sending said message {:?}", why);
            }
        }
        if msg.content.split(" ").collect::<Vec<&str>>()[0] == NROOT_COMMAND {
            let params = msg.content.split(" ").collect::<Vec<&str>>();
            if let Err(why) = msg.channel_id.say(&ctx.http, nroot(params[1].parse().unwrap(),
                                    params[2].parse().unwrap(), params[3].parse().unwrap())).await {
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
        .await
        .expect("Err creating client");
    if let Err(why) = client.start().await {
        println!("Error: {:?}", why);
    }
}
