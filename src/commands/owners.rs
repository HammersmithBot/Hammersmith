use crate::{Context, Error};

#[poise::command(prefix_command, owners_only, hide_in_help)]
pub async fn shutdown(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Shutting down bot!").await?;
    ctx.framework()
        .shard_manager()
        .lock()
        .await
        .shutdown_all()
        .await;
    Ok(())
}