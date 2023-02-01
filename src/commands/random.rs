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
    if ctx.author().id == serenity::UserId(310437480216395777) {
        ctx.send(|f| {
            f.embed(|f| {
                f.title("Random God")
                    .description(format!("You main **Aphrodite**, idiot."))
                    .color(serenity::Colour::from_rgb(227, 28, 121))
            })
        })
        .await?;
        return Ok(());
    }

    let god: String = get_random_god().await?;
    ctx.send(|f| {
        f.embed(|f| {
            f.title("Random God")
                .description(format!("Try not to throw with **{}**, idiot.", god))
                .color(serenity::Colour::BLUE)
        })
    })
    .await?;
    Ok(())
}
