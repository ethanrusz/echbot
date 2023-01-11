use crate::{Context, Error};

/// Picks a random something
#[poise::command(slash_command, prefix_command, subcommands("god", "mode"))]
pub(crate) async fn random(
    ctx: Context<'_>,
) -> Result<(), Error> {
    ctx.say("Pick a subcommand, idiot.").await?;
    Ok(())
}

/// Picks a random god to play
#[poise::command(slash_command, prefix_command)]
pub(crate) async fn god(
    ctx: Context<'_>,
) -> Result<(), Error> {
    ctx.say("Waiting for that sweet API access.").await?;
    Ok(())
}

/// Picks a random mode to play
#[poise::command(slash_command, prefix_command)]
pub(crate) async fn mode(
    ctx: Context<'_>,
) -> Result<(), Error> {
    ctx.say("Waiting for that sweet API access.").await?;
    Ok(())
}