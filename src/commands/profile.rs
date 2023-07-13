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

    let winrate = (profile.wins as f32 / (profile.wins as f32 + profile.losses as f32)) * 100f32;
    ctx.send(|f| {
        f.embed(|f| {
            f.title(format!("{}", profile.name.as_ref().unwrap()))
                .description(format!(
                    "{}'s statistics.",
                    profile.hz_player_name.as_ref().unwrap()
                ))
                .field(
                    "Clan Name",
                    format!("{}", profile.clan.as_ref().unwrap_or(&String::from(""))),
                    true,
                )
                .field(
                    "Status Message",
                    format!("{}", profile.personal_status_message.as_ref().unwrap()),
                    false,
                )
                .field("Level", format!("{}", profile.level), true)
                .field("Hours Played", format!("{}", profile.hours_played), true)
                .field("Leaves", format!("{}", profile.leaves), true)
                .field(
                    "Platform",
                    format!("{}", profile.platform.as_ref().unwrap()),
                    false,
                )
                .field("Wins", format!("{}", profile.wins), true)
                .field("Losses", format!("{}", profile.losses), true)
                .field("Winrate", format!("{:.2}%", winrate), true)
                .color(serenity::Colour::BLURPLE)
        })
    })
    .await?;
    Ok(())
}
