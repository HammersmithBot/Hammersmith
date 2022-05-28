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
) -> Result<(), Error> {
    let response = match punish_type {
        PunishType::Ban => {
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
