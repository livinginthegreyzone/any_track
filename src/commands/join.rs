use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::interaction::application_command::ApplicationCommandInteraction,
};

pub async fn run(
    command: &ApplicationCommandInteraction,
    ctx: &serenity::client::Context,
) -> Option<String> {
    if let Some(guild_id) = command.guild_id {
        match ctx.cache.guild_field(guild_id, |guild| {
            guild
                .voice_states
                .get(&command.user.id)
                .and_then(|voice_state| voice_state.channel_id)
        }) {
            Some(Some(channel_id)) => {
                let manager = songbird::get(ctx)
                    .await
                    .expect("Songbird voice client placed in at initialization.");
                let (handler, handler_join_error) = manager.join(guild_id, channel_id).await;
                if let Err(err) = handler_join_error {
                    eprintln!("error: failed to join voice channel due to \"{}\"", err);
                    return Some("Failed to join voice channel".to_string());
                }
                let mut handler = handler.lock().await;
                if let Err(err) = handler.deafen(true).await {
                    eprintln!("error: failed to deafen due to \"{}\"", err);
                    Some("joined voice channel but unable to deafen".to_string())
                } else {
                    Some("joined voice channel".to_string())
                }
            }
            Some(None) => {
                eprintln!("error: not in voice channel");
                Some("Not in a voice channel, join a voice channel first".to_string())
            }
            None => {
                eprintln!("error: no guild with guild_id \"{}\" in cache", guild_id);
                Some("internal error".to_string())
            }
        }
    } else {
        eprintln!("error: no guild_id found");
        Some("internal error".to_string())
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("atjoin").description("Join the voice channel")
}
