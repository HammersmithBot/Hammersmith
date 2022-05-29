use crate::{Context, Error};
use poise::serenity_prelude as serenity;

#[derive(Debug, poise::ChoiceParameter)]
pub enum PunishType {
    Ban,
    Kick,
    Mute,
}

#[poise::command(
    slash_command,
    required_permissions = "BAN_MEMBERS | KICK_MEMBERS"
)]
pub async fn punish(
    ctx: Context<'_>,
    #[description = "Punishment type"]
    #[rename = "type"]
    punish_type: PunishType,
    #[description = "User to punish"] user: serenity::User,
    #[description = "Reason for punishment"] reason: Option<String>,
) -> Result<(), Error> {
    let response = match punish_type {
        PunishType::Ban => {
            let default_reason = "No reason given".to_string();
            let reason_message = reason.as_ref().unwrap_or(&default_reason);
            let guild_id = ctx.guild_id().unwrap().0;
            ctx.discord().http.ban_user(guild_id, user.id.0, 7, reason_message).await?;
            format!("{} has been banned", user.name)
        }
        PunishType::Kick => {
            format!("{} has been kicked", user.name)
        }
        PunishType::Mute => {
            format!("{} has been muted", user.name)
        }
    };
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(
slash_command,
required_permissions = "BAN_MEMBERS | KICK_MEMBERS"
)]
pub async fn punishid(
    ctx: Context<'_>,
    #[description = "Punishment type"]
    #[rename = "type"]
    punish_type: PunishType,
    #[description = "ID to punish"] user: String,
    #[description = "Reason for punishment"] reason: Option<String>,
) -> Result<(), Error> {
    let user_id = user.parse::<u64>().unwrap();
    let response = match punish_type {
        PunishType::Ban => {
            let default_reason = "No reason given".to_string();
            let reason_message = reason.as_ref().unwrap_or(&default_reason);
            let guild_id = ctx.guild_id().unwrap().0;
            ctx.discord().http.ban_user(guild_id, user_id, 7, reason_message).await?;
            format!("{} has been banned", user)
        }
        PunishType::Kick => {
            format!("{} has been kicked", user)
        }
        PunishType::Mute => {
            format!("{} has been muted", user)
        }
    };
    ctx.say(response).await?;
    Ok(())
}