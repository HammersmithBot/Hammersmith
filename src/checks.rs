use crate::{CONFIG, Context, Error};


// useless due to owners_only existing. Keeping as example
async fn is_bot_owner(ctx: Context<'_>) -> Result<bool, Error> {
    let user_id = ctx.author().id.0;
    if (*CONFIG.bot.owners).contains(&user_id) {
        Ok(true)
    } else {
        Ok(false)
    }
}