use crate::commands::api::get_player;
use crate::serenity;
use crate::{Context, Error};

/// Looks up a player's profile
#[poise::command(slash_command)]
pub async fn profile(
    ctx: Context<'_>,
    #[rename = "player"] player_name: String,
) -> Result<(), Error> {
    let profiles = get_player(player_name).await?;
    let profile = profiles.first().unwrap();
    if profile.name.is_none() {
        ctx.send(|f| {
            f.embed(|f| {
                f.title("Hidden")
                    .description("This profile is hidden.")
                    .color(serenity::Colour::RED)
            })
        })
        .await?;
        return Ok(());
    }

    let winrate =
        (profile.wins as f32 / (profile.wins as f32 + profile.losses as f32)) * 100 as f32;
    ctx.send(|f| {
        f.embed(|f| {
            f.title(format!("{}", profile.name.as_ref().unwrap()))
                .description(format!(
                    "{}'s statistics.",
                    profile.hz_player_name.as_ref().unwrap()
                ))
                .field(
                    "Status",
                    format!("{}", profile.personal_status_message.as_ref().unwrap()),
                    false,
                )
                .field("Hours played", format!("{}", profile.hours_played), false)
                .field("Wins", format!("{}", profile.wins), true)
                .field("Losses", format!("{}", profile.losses), true)
                .field("Winrate", format!("{:.2}%", winrate), true)
                .color(serenity::Colour::BLURPLE)
        })
    })
    .await?;
    Ok(())
}
