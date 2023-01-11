use poise::serenity_prelude as serenity;
use rand::seq::IteratorRandom;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Data {}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Basically a ping command.
#[poise::command(slash_command, prefix_command)]
async fn slur(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let file = File::open("quotes.txt").unwrap_or_else(|_e| panic!("Quote file missing.")); // Open quotes file
    let file = BufReader::new(file); // Read quotes file
    let quotes = file.lines().map(|res| res.expect("Failed to read line."));
    let quote = quotes.choose(&mut rand::thread_rng()).expect("No lines in file."); // Pick random quote

    ctx.say(quote).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn team_up(
    ctx: Context<'_>,
    #[description = "Your voice channel"] channel: Option<serenity::Channel>,
) -> Result<(), Error> {
    let c = channel.as_ref().unwrap(); // Get channel info from object
    let mut v = ctx.guild().unwrap().voice_states; // Get hashmap of users' voice states within the guild
    v.retain(|_, s| s.channel_id == Some(c.id())); // Drop users not active in requested voice channel from hashmap
    let res = format!("Channel {} has {} active users", c.id(), v.keys().len());
    ctx.say(res).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![slur(), team_up()], // IntelliJ doesn't like this, but it's fine.
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
