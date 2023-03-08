use std::collections::HashMap;
use std::string::String;

use poise::serenity_prelude::{Guild, Member, UserId, VoiceState};
use rand::seq::SliceRandom;

use crate::serenity;
use crate::{Context, Error};

/// Return a string of pingable IDs from a slice of string UserIds
fn team_to_ping(team: &[&String]) -> String {
    team.iter()
        .map(|o| format!("<@{o}>"))
        .collect::<Vec<String>>()
        .join(", ")
}

/// Splits up players for custom matches
#[poise::command(slash_command)]
pub async fn team(
    ctx: Context<'_>,
    #[description = "Your order voice channel"]
    #[rename = "order"]
    #[channel_types("Voice")]
    order_channel: serenity::Channel, // Channel to pick all members from
    #[description = "Your chaos voice channel"]
    #[rename = "chaos"]
    #[channel_types("Voice")]
    chaos_channel: serenity::Channel, // Channel to move chaos team members into
    #[description = "Team size"]
    #[min = 1]
    size: u8, // Number of members on each team
) -> Result<(), Error> {
    let mut voice_states: HashMap<UserId, VoiceState> = ctx.guild().unwrap().voice_states; // Get hashmap of users' voice states within the guild
    voice_states.retain(|_, state: &mut VoiceState| state.channel_id == Some(order_channel.id())); // Drop users not active in requested voice channel from hashmap

    if voice_states.keys().len() < size as usize * 2 {
        // Make sure there are enough members in the voice channel
        ctx.send(|f| {
            f.embed(|f| {
                f.title(format!("Custom {size}v{size} Teams"))
                    .description("There are not enough members in the call!")
                    .color(serenity::Colour::RED)
            })
        })
        .await?;
        return Ok(()); // Break out early if there are not enough members
    }

    let uuid_team: u64 = ctx.id(); // Grab context ID for action row
    let users: Vec<String> = Vec::from_iter(voice_states.keys().map(|u| u.to_string())); // Get vec of PIDs
    let players: Vec<&String> = users
        .choose_multiple(&mut rand::thread_rng(), size as usize * 2)
        .collect(); // Pick players randomly into slice
    let (order, chaos) = players.split_at(players.len() / 2); // Split slice into two teams

    ctx.send(|f| {
        f.embed(|f| {
            f.title(format!("Custom {size}v{size} Teams"))
                .description("Click the button below to move the Chaos players.")
                .field("Order", team_to_ping(order), false)
                .field("Chaos", team_to_ping(chaos), false)
                .color(serenity::Colour::DARK_GREEN)
        })
        .components(|c| {
            c.create_action_row(|a| {
                // Create an action row with button
                a.create_button(
                    |b| {
                        b.style(serenity::ButtonStyle::Primary)
                            .label("Swap Channels")
                            .custom_id(uuid_team)
                    }, // Use the context ID as button ID
                )
            })
        })
    })
    .await?; // Send embed with team picks

    while let Some(mci) = serenity::CollectComponentInteraction::new(ctx) // Handle the interaction
        .await
    {
        let guild: Guild = ctx.guild().unwrap(); // Grab guild from context
        for user in chaos {
            let member: Member = guild.member(ctx, UserId(user.parse()?)).await?; // Get the member in the correct guild
            member
                .move_to_voice_channel(ctx, chaos_channel.id())
                .await?; // Move the member to the correct voice channel
        }

        mci.create_interaction_response(ctx, |ir| {
            // Edit embed
            ir.kind(serenity::InteractionResponseType::UpdateMessage)
                .interaction_response_data(|f| {
                    f.embed(|f| {
                        f.title(format!("Custom {size}v{size} Teams"))
                            .description("Good luck! Have fun!")
                            .field("Order", team_to_ping(order), false)
                            .field("Chaos", team_to_ping(chaos), false)
                            .color(serenity::Colour::DARK_GREEN)
                    })
                    .components(|c| {
                        c.create_action_row(|a| {
                            // Create an action row with button
                            a.create_button(
                                |b| {
                                    b
                            .disabled(true) // with disabled button
                            .style(serenity::ButtonStyle::Primary)
                            .label("Quit Sibelius") // and new text
                                }, // Use the context ID as button ID
                            )
                        })
                    })
                })
        })
        .await?;
    }
    Ok(())
}
