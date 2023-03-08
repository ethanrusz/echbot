use poise::serenity_prelude as serenity;

mod commands;

pub struct Data {}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::ping::ping(),
                commands::team::team(),
                commands::random::random(),
                commands::register::register(),
                commands::profile::profile(),
            ], // IntelliJ doesn't like this, but it's fine.
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged()) // Set intents for Discord dev portal
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?; // Update slash commands
                ctx.set_activity(serenity::Activity::playing("SMITE")).await;
                Ok(Data {})
            })
        });
    framework.run().await.unwrap();
}
