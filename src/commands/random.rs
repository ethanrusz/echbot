use crate::{Context, Error};

use crate::commands::api::get_random_god;

/// Picks a random something
#[poise::command(slash_command, subcommands("god", "mode"))]
pub(crate) async fn random(
    _ctx: Context<'_>,
) -> Result<(), Error> {
    Ok(())
}

/// Picks a random god
#[poise::command(slash_command)]
pub(crate) async fn god(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let god = "some god";
    get_random_god().await?;
    // ctx.say(format!("{}", god)).await?;
    Ok(())
}

/// Picks a random mode
#[poise::command(slash_command)]
pub(crate) async fn mode(
    ctx: Context<'_>,
) -> Result<(), Error> {
    ctx.say("Waiting for that sweet API access.").await?;
    Ok(())
}
