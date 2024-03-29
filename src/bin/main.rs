use any_track::Handler;
use serenity::{prelude::GatewayIntents, Client};
use songbird::SerenityInit;

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN")
        .or_else(|_| std::fs::read_to_string("config/bot.token"))
        .expect("Expected a token in the environment `DISCORD_TOKEN=YOUR_TOKEN_HERE` or from `config/bot.token`");

    let mut client = Client::builder(
        token,
        GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::MESSAGE_CONTENT
            | GatewayIntents::GUILD_VOICE_STATES
            | GatewayIntents::non_privileged(),
    )
    .event_handler(Handler)
    .register_songbird()
    .await
    .expect("Error creating client");

    if let Err(err) = client.start().await {
        println!("Client error: {:?}", err);
    }
}
