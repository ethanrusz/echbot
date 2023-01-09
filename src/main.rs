use poise::serenity_prelude as serenity;

struct Data {}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// Basically a ping command. Should be moved to a mod.
#[poise::command(slash_command, prefix_command)]
async fn slur(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let response = "If you don't like my content or the way I act, don't watch me.";
    ctx.say(response).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![slur()], // Intellij doesn't like this, but it's fine.
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}
