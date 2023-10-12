use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler
{
    async fn message(&self, ctx: Context, msg: Message)
    {
        println!("{}", &msg.content);

        if msg.content == "!quit"
        {
            let _ = msg.channel_id.say(&ctx.http, "***Terminating . . .***").await;
            std::process::exit(0);
        }
        else if msg.content == "!ping"
        {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong! Nya :3").await
            {
                println!("Failed sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready)
    {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main()
{
    let token = "";

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await
    {
        println!("Client error: {:?}", why);
    }
}