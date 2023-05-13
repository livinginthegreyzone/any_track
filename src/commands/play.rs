use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        command::CommandOptionType, interaction::application_command::ApplicationCommandInteraction,
    },
};

pub async fn run(
    command: &ApplicationCommandInteraction,
    ctx: &serenity::client::Context,
) -> Option<String> {
    if let Some(guild_id) = command.guild_id {
        let manager = songbird::get(ctx)
            .await
            .expect("Songbird voice client placed in at initialization.");
        if let Some(handler_lock) = manager.get(guild_id) {
            let url = command.data.options.iter().find_map(|option| {
                (option.name == "url")
                    .then_some(())
                    .and(option.value.as_ref())
                    .and_then(|value| value.as_str())
            });

            match url {
                Some(url) => match songbird::ytdl(url).await {
                    Ok(source) => {
                        let mut handler = handler_lock.lock().await;
                        let track_handle = handler.play_source(source);

                        Some(format!("playing {:?}", track_handle.metadata()))
                    }
                    Err(err) => {
                        eprintln!("error: couldn't start source: {}", err);

                        Some("couldn't source ffmpeg".to_string())
                    }
                },
                None => {
                    eprintln!("error: couldn't find url in the command");
                    Some("couldn't find URL in the command".to_string())
                }
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
        .name("atplay")
        .description("Play the audio from the specified URL")
        .create_option(|option| {
            option
                .kind(CommandOptionType::String)
                .name("url")
                .description("URL to play")
                .required(true)
        })
}
