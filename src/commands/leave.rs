use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::interaction::application_command::ApplicationCommandInteraction,
};

pub async fn run(
    command: &ApplicationCommandInteraction,
    ctx: &serenity::client::Context,
) -> Option<String> {
    if let Some(guild_id) = command.guild_id {
        let manager = songbird::get(ctx)
            .await
            .expect("Songbird voice client placed in at initialization.");
        if manager.get(guild_id).is_some() {
            if let Err(err) = manager.remove(guild_id).await {
                Some(format!("Failed: {:?}", err))
            } else {
                Some("Left voice channel".to_string())
            }
        } else {
            Some("Not in a voice channel".to_string())
        }
    } else {
        eprintln!("error: no guild_id found");
        Some("internal error".to_string())
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("atleave")
        .description("Leave the voice channel")
}
