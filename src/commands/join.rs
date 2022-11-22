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
                let _handler = manager.join(guild_id, channel_id).await;
                Some("joined voice channel".to_string())
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
