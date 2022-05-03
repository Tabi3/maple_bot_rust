use serenity::{
    async_trait,
    client::{*, self},
    framework::standard::{macros::{group, hook, command}, *},
    http::Http,
    model::{*, channel::Message},
    prelude::*,
    utils::*,
};
mod test_module;
use test_module::*;

use std::{collections::*, env, fmt::Write, sync::Arc};

use tokio::sync::Mutex;
struct ShardManagerContainer;
struct Handler;
struct CommandCounter;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<bridge::gateway::ShardManager>>;
}

impl TypeMapKey for CommandCounter {
    type Value = HashMap<String, u64>;
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: prelude::Ready) {
        print!("{} is Running\n---------------\n", ready.user.name);
    }
}

#[group]
#[description = "A group of Math commands"]
#[prefixes(math, m)]
#[commands(anroot)]
struct MathCommands;

#[hook]
async fn before(ctx: &Context, _msg: &Message, command_name: &str) -> bool {
    print!("user {} called command {}\n",_msg.author.name ,command_name);
    
    let mut data = ctx.data.write().await;
    let counter = data.get_mut::<CommandCounter>().expect("Weird shit");
    let entry = counter.entry(command_name.to_string()).or_insert(0);
    *entry += 1;

    
    return true;
}

#[hook]
async fn after(_ctx: &Context, _msg: &Message, command_name: &str, command_result: CommandResult) {
    match command_result {
        Ok(()) => println!("Command '{}' was succesful", command_name),
        Err(why) => println!("Command '{}' returned error {:?}", command_name, why),
    }
}


#[tokio::main]
async fn main() {
    let token = "ODQxMzYyMDg2NjY2OTYwOTI2.YJlpgQ.IlqXgMo4LNP0RIyycxURdu5voZ8";
    let http = Http::new(&token);
    let (owner_id, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        }
        Err(why) => panic!("Couldn't access application info {:?}", why),
    };
    let framework = StandardFramework::new()
        .configure(|c| {
            c.with_whitespace(true)
                .on_mention(Some(bot_id))
                .prefix(".")
                .delimiters(vec![", ", ","])
                .owners(owner_id)
        })
        .before(before)
        .after(after)
        .group(&MATHCOMMANDS_GROUP);

    let mut client = Client::builder(token, GatewayIntents::all())
                        .event_handler(Handler).framework(framework)
                        .type_map_insert::<CommandCounter>(HashMap::default())
                        .await.expect("Something went wrong creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}


#[command]
async fn anroot(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let arg1 = args.single::<f64>()?;
    let arg2 = args.single::<f64>()?;
    msg.channel_id.say(&ctx.http, format!("{}", nroot(arg1, arg2, 1e-1))).await?;

    Ok(())
}