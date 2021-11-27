use std::collections::HashMap;
use std::{env, str};

use serenity::{
    async_trait,
    model::{
        gateway::Ready,
        interactions::{
            application_command::ApplicationCommand, Interaction, InteractionResponseType,
        },
    },
    prelude::*,
};

struct Handler;

async fn get_time_json() -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://tigtbifhimva.sarda.dev/api/v1/time_in_4_hours")
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    let result = format!(
        "The time in Melbourne Victoria Australia in four hours is  {}",
        resp["result"].to_string()
    );
    Ok(result)
}

async fn get_time_brainfuck() -> Result<String, Box<dyn std::error::Error>> {
    let resp =
        reqwest::get("https://tigtbifhimva.sarda.dev/api/v1/time_in_4_hours?return_type=brainfuck")
            .await?
            .bytes()
            .await?;

    let result = str::from_utf8(&resp).unwrap();
    Ok(result.to_owned())
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "four_hours_melbourne" => match get_time_json().await {
                    Err(_) => "cannot reach the server".to_string(),
                    Ok(time_str) => time_str,
                },
                "four_hours_melbourne_brainfuck" => match get_time_brainfuck().await {
                    Err(_) => "cannot reach the server".to_string(),
                    Ok(time_str) => time_str,
                },
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        // let commands = match ApplicationCommand::get_global_application_commands(&ctx.http).await {
        //     Ok(commands) => commands,
        //     Err(_) => panic!("fuck"),
        // };

        // for cmd in commands {
        //     ApplicationCommand::delete_global_application_command(&ctx.http, cmd.id);
        // }

        let guild_command =
            ApplicationCommand::create_global_application_command(&ctx.http, |command| {
                command
                    .name("four_hours_melbourne")
                    .description("get the time in four hours in melbourne.")
            })
            .await;

        println!(
            "I created the following global slash command: {:#?}",
            guild_command
        );

        let guild_command =
            ApplicationCommand::create_global_application_command(&ctx.http, |command| {
                command
                    .name("four_hours_melbourne_brainfuck")
                    .description("get the time in four hours in melbourne in brainfuck.")
            })
            .await;

        println!(
            "I created the following global slash command: {:#?}",
            guild_command
        );
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // The Application Id is usually the Bot User Id.
    let application_id: u64 = env::var("APPLICATION_ID")
        .expect("Expected an application id in the environment")
        .parse()
        .expect("application id is not a valid id");

    // Build our client.
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .application_id(application_id)
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
