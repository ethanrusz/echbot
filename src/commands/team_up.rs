use crate::{Context, Error};
use crate::serenity;

/// Split up users for custom joust matches
#[poise::command(slash_command, prefix_command)]
pub(crate) async fn team_up(
    ctx: Context<'_>,
    #[description = "Your voice channel"] channel: Option<serenity::Channel>,
) -> Result<(), Error> {
    let c = channel.as_ref().unwrap(); // Get channel info from object
    let mut v = ctx.guild().unwrap().voice_states; // Get hashmap of users' voice states within the guild
    v.retain(|_, s| s.channel_id == Some(c.id())); // Drop users not active in requested voice channel from hashmap
    let res = format!("Channel {} has {} active users", c.id(), v.keys().len());

    ctx.say(res).await?;
    Ok(())
}
