use crate::{Context, Error};
use crate::serenity;

/// Splits up players for custom matches
#[poise::command(slash_command)]
pub(crate) async fn team(
    ctx: Context<'_>,
    #[description = "Your voice channel"]
    #[channel_types("Voice")] channel: serenity::Channel,
    #[description = "Team size"]
    #[min = 1] size: u8,
) -> Result<(), Error> {
    let mut v = ctx.guild().unwrap().voice_states; // Get hashmap of users' voice states within the guild
    v.retain(|_, s| s.channel_id == Some(channel.id())); // Drop users not active in requested voice channel from hashmap
    let res = format!("Channel {} has {} active users.", channel.id(), v.keys().len());

    ctx.send(|f| f
        .content(res)
        .embed(|f| f
            .title(format!("Custom {}v{} Teams", size, size))
            .description("I'm not done with this yet.")
            .field("Order", "Some names", true)
            .field("Chaos", "Other names", true)
            .field("Spectators", "You guessed it, names.", false)
            .color(serenity::Colour(16711680)) // Red
        )).await?;
    Ok(())
}
