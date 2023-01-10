use poise::serenity_prelude as serenity;
use rand::seq::IteratorRandom;
use std::{
    fs::File,
    io::{BufRead, BufReader}
};

struct Data {}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// Basically a ping command. Should be moved to a mod.
#[poise::command(slash_command, prefix_command)]
async fn slur(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let file = File::open("quotes.txt").unwrap_or_else(|_e| panic!("Quote file missing."));
    let file = BufReader::new(file);
    let quotes = file.lines().map(|res| res.expect("Failed to read line."));
    let quote = quotes.choose(&mut rand::thread_rng()).expect("No lines in file.");

    ctx.say(quote).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![slur()], // IntelliJ doesn't like this, but it's fine.
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
