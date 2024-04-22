#![feature(let_chains)]

use std::{env, fs};
use std::sync::Arc;
use database::Database;
use dotenv;

use proxie::tokio::AsyncProxy;
use serenity::all::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::async_trait;
use serenity::gateway::ShardManager;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::model::application::{Command, Interaction};

use proxie::SOCKS5Proxy;

mod database;
mod commands;

const PATH: &str = "./testdata";

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<ShardManager>;
}

struct GlobalDatabase;

impl TypeMapKey for GlobalDatabase {
    type Value = Arc<Database>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(ref command) = interaction {
            println!("Received command interaction: {command:#?}");

            let content = match command.data.name.as_str() {
                "iowe" => {
                    let interaction = &interaction.to_owned();
                    let command_interaction = interaction.as_command();
                    Some(commands::iowe::run(&ctx, &command.data.options(), &command_interaction.unwrap()).await)
                },
                "getuserdebt" => {
                    let interaction = &interaction.to_owned();
                    let command_interaction = interaction.as_command();
                    Some(commands::getuserdebt::run(&ctx, &command.data.options(), &command_interaction.unwrap()).await)
                },
                "youowe" => {
                     let interaction = &interaction.to_owned();
                    let command_interaction = interaction.as_command();
                    Some(commands::youowe::run(&command.data.options(), &command_interaction.unwrap()).await)
                },
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }

            let data = ctx.data.read().await;
            data.get::<GlobalDatabase>().unwrap().sync().unwrap();
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        Command::set_global_commands(&ctx.http, commands::get_commands()).await.unwrap();

        println!("Registered commands: {:?}", Command::get_global_commands(&ctx.http).await);
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    let db = match fs::read(PATH) {
        Ok(_) => {
            println!("Opening db");
            database::open_database(PATH).unwrap()
        }

        Err(_) => {
            println!("Creating db");
            database::create_database(PATH).unwrap()
        }
    };

    {
        let mut data = client.data.write().await;
        data.insert::<GlobalDatabase>(Arc::new(db));
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");
        shard_manager.shutdown_all().await;
    });

    client.start().await.unwrap();
}