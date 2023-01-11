mod commands;

use poise::serenity_prelude as serenity;

struct Data {}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::slur::slur(),
                commands::team_up::team_up(),
            ], // IntelliJ doesn't like this, but it's fine.
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN")
            .expect("Missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged()) // Set intents for Discord dev portal
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(
                    ctx,
                    &framework.options().commands,
                    serenity::GuildId(std::env::var("GUILD_ID")
                        .expect("Missing GUILD_ID") // Get GID from env and parse
                        .parse::<u64>().unwrap())).await?; // Update slash commands in GID
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}
