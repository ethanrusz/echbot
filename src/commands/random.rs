use crate::commands::api::get_random_god;
use crate::serenity;
use crate::{Context, Error};

/// Picks a random something
#[poise::command(slash_command, subcommands("god"))]
pub async fn random(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Picks a random god
#[poise::command(slash_command)]
pub async fn god(ctx: Context<'_>) -> Result<(), Error> {
    let god: String = get_random_god().await?;
    ctx.send(|f| {
        f.embed(|f| {
            f.title("Random God")
                .description(format!("Try not to throw with **{god}**, idiot."))
                .color(serenity::Colour::BLUE)
        })
    })
    .await?;
    Ok(())
}
