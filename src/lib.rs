use anyhow::anyhow;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::{async_trait, utils::MessageBuilder};
use shuttle_secrets::SecretStore;
use tracing::{error, info};

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if !msg.content.to_lowercase().contains("!") {
            let owo = msg.content.to_lowercase().contains("owo");
            if owo == true {
                number += 1;
            }
        } else {
            match msg.content.as_str() {
                "!hello" => if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {},
                "!owo" => {
                    let channel = msg.channel_id.to_channel(&ctx).await.unwrap();
                    let response = MessageBuilder::new()
                        .push("owo count: ")
                        .push(&number)
                        .build();
                    if let Err(err) = msg.channel_id.say(&ctx.http, &response).await {
                        println!("Error sending message: {:?}", err);
                    }
                }
                "!ping" => {
                    let channel = msg.channel_id.to_channel(&ctx).await.unwrap();
                    let response = MessageBuilder::new()
                        .push("User ")
                        .push_bold_safe(&msg.author)
                        .push(" used the 'ping' command in the ")
                        .mention(&channel)
                        .push(" channel")
                        .build();

                    if let Err(err) = msg.channel_id.say(&ctx.http, &response).await {
                        println!("Error sending message: {:?}", err);
                    };
                }
                _ => {}
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[shuttle_service::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_service::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents: GatewayIntents = GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client)
}
