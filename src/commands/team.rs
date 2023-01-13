use std::collections::HashMap;
use crate::{Context, Error};
use crate::serenity;
use poise::serenity_prelude::{UserId, VoiceState};
use rand::seq::SliceRandom;
use std::string::String;

fn team_to_ping(team: &[&String]) -> String {
    return team.iter().map(|o| format!("<@{}>", o)).collect::<Vec<String>>().join(", ");
}

/// Splits up players for custom matches
#[poise::command(slash_command)]
pub(crate) async fn team(
    ctx: Context<'_>,
    #[description = "Your voice channel"]
    #[channel_types("Voice")] channel: serenity::Channel,
    #[description = "Team size"]
    #[min = 1] size: u8,
) -> Result<(), Error> {
    let mut v: HashMap<UserId, VoiceState> = ctx.guild().unwrap().voice_states; // Get hashmap of users' voice states within the guild
    v.retain(|_, s: &mut VoiceState| s.channel_id == Some(channel.id())); // Drop users not active in requested voice channel from hashmap

    if v.keys().len() < size as usize * 2 { // Make sure there are enough members in the voice channel
        ctx.send(|f| f
            .embed(|f| f
                .title(format!("Custom {}v{} Teams", size, size))
                .description("You don't have enough friends for that, idiot.")
                .color(serenity::Colour::RED)
            )).await?; // Insult the user for not having enough members in call
    } else {
        let users: Vec<String> = Vec::from_iter(v.keys().map(|u| u.to_string())); // Get vec of PIDs
        let players: Vec<&String> = users.choose_multiple(
            &mut rand::thread_rng(), size as usize * 2).collect(); // Pick players randomly into slice
        let (order, chaos) = players.split_at(players.len() / 2); // Split slice into two teams

        ctx.send(|f| f
            .embed(|f| f
                .title(format!("Custom {}v{} Teams", size, size))
                .description("Good luck have fun.")
                .field("Order", team_to_ping(order), false)
                .field("Chaos", team_to_ping(chaos), false)
                .color(serenity::Colour::DARK_GREEN)
            )).await?; // Send embed with team picks
    }
    Ok(())
}
