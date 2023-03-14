use crate::serenity;
use crate::{Context, Error};

#[poise::command(slash_command, subcommands("up", "down"))]
pub async fn pog(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Increases the pogs
#[poise::command(slash_command)]
pub async fn up(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(|f| {
        f.embed(|f| {
            f.title("Pog Status")
                .description("The pog level has been increased.")
                .color(serenity::Colour::DARK_GREEN)
        })
    })
        .await?;
    Ok(())
}

/// Decreases the pogs
#[poise::command(slash_command)]
pub async fn down(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(|f| {
        f.embed(|f| {
            f.title("Pog Status")
                .description("The pog level has been decreased.")
                .color(serenity::Colour::RED)
        })
    })
        .await?;
    Ok(())
}
