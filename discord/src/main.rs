use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::fs;

struct Handler;

/*
struct Command<'a>
{
    cmd: &'a str,
    len: usize
}

impl<'a> Command<'a>
{
    const fn new(data: &'a str) -> Self
    {
        Self
        {
            cmd: &data,
            len: data.len()
        }
    }
}


const COMMAND_LIST: [Command; 3] =
[
    Command::new("!test"),
    Command::new("!cd"),
    Command::new("!cmd")
];
*/

#[async_trait]
impl EventHandler for Handler
{
    async fn message(&self, ctx: Context, msg: Message)
    {
        let content = &msg.content;

        let content: Vec<String> = content.split_whitespace().map(str::to_string).collect();
        let content_len = content.len();

        if content_len >= 1
        {
            println!("conent_len: {}", content_len);

            if content[0] == "!quit"
            {
                let _ = msg.channel_id.say(&ctx.http, "***Terminating . . .***").await;
                std::process::exit(0);
            }
            else if content[0] == "!ping"
            {
                if let Err(why) = msg.channel_id.say(&ctx.http, "Pong! Nya :3").await
                {
                    println!("Failed sending message: {:?}", why);
                }
            }
            else if content[0] == "!cd"
            {
                let content = &content[1];

                //let root = std::path::Path::new(content);

                let _ = std::env::set_current_dir(content);

                //println!("Successfully changed working directory to {}!", root.display());
            }
            else if content[0] == "!pwd"
            {
                let path = std::env::current_dir();

                match path
                {
                    Ok(path_ok) =>
                    {
                        let _ = msg.channel_id.say(&ctx.http, path_ok.display()).await;
                    },
                    Err(path_err) =>
                    {
                        let _ = msg.channel_id.say(&ctx.http, path_err).await;
                    }
                }
                //println!("The current directory is {}", path.display());
            }
            else if content[0] == "!get"
            {
                let paths = vec![&*content[1]];

                let _ = &msg.channel_id.send_files(&ctx.http, paths, |m| m.content("")).await;
            }
            else if content[0] == "!ls"
            {
                let content = &content[1];

                let mut file_list = vec![];

                match fs::read_dir(content)
                {
                    Ok(files) =>
                    {
                        for file in files
                        {
                            match file
                            {
                                Ok(f) =>
                                {
                                    let file_name = f.file_name();

                                    file_list.push(String::from(file_name.to_str().unwrap()));


                                    //println!("FILE: {}", file_name.to_str().unwrap());
                                },
                                Err(error) => { let _ = &msg.channel_id.say(&ctx.http, error); }
                            }
                        }
                    },
                    Err(error) =>
                    {
                        let _ = &msg.channel_id.say(&ctx.http, error);
                    }
                }

                /*for file in file_list
                {
                    println!("File {}", file);
                    let _ = &msg.channel_id.say(&ctx, "file");
                    let _ = &msg.channel_id.say(&ctx, file);
                }*/

                let joined = file_list.join("\n");
                println!("{}", joined);
                let _ = &msg.channel_id.say(&ctx, joined);
            }

            else if content[0] == "!cmd"
            {
                let output = std::process::Command::new("echo")
                    .arg("echo hello")
                    .output()
                    .expect("failed to execute process");

                let _ = &msg.channel_id.say(&ctx.http, &output.status);
            }

            for piece in content
            {
                println!("{}", piece);
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
    let token = "NDU3NDk1NTAyMzQ3MzA0OTYw.GQj8j6.Q3PPceVMA8oQfXCtrvoV16gbqGBH8zk7WE_feA";

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
