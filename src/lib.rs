mod commands;

use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        gateway::Ready,
        id::GuildId,
        prelude::interaction::{Interaction, InteractionResponseType},
    },
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Set a handler to be called on the `ready` event. This is called
    // when a shard is booted, and a READY payload is sent by
    // Discord. This payload contains data like the current user's
    // guild Ids, current user data, private channels, and more.
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("info: {} is connected!", ready.user.name);

        for guild_id in ctx.cache.guilds().iter() {
            let commands = GuildId::set_application_commands(guild_id, &ctx.http, |commands| {
                commands
                    .create_application_command(|command| commands::ping::register(command))
                    .create_application_command(|command| commands::join::register(command))
                    .create_application_command(|command| commands::leave::register(command))
                    .create_application_command(|command| commands::play::register(command))
            })
            .await;

            println!(
                "I now have the following guild slash commands: {:#?}",
                commands
            );
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("info: received command interaction: {:#?}", command);

            let message = match command.data.name.as_str() {
                "atping" => Some(commands::ping::run(&command.data.options)),
                "atjoin" => commands::join::run(&command, &ctx).await,
                "atleave" => commands::leave::run(&command, &ctx).await,
                "atplay" => commands::play::run(&command, &ctx).await,
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = message {
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
    }
}
