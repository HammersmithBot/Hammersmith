use lazy_static::lazy_static;
use poise::serenity_prelude as serenity;

// Import commands
mod commands;
mod config;

use commands::*;

// Types used by everything
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
// User data, which is stored and accessible in all command invocations
pub struct Data {}

lazy_static! {
    pub static ref CONFIG: config::Data = config::read_config("config.toml");
}

/// Displays your or another user's account creation date
#[poise::command(slash_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or(ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

/// Register slash commands. @mention register
#[poise::command(prefix_command, owners_only, hide_in_help)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

/// Error handler
async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

/// Event handlers
async fn event_listeners(
    _ctx: &serenity::Context,
    event: &poise::Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _user_data: &Data,
) -> Result<(), Error> {
    match event {
        poise::Event::Ready {
            data_about_bot: bot,
        } => {
            println!("{} is connected!", bot.user.name)
        }
        poise::Event::Message {
            new_message: Message,
        } => {
            println!("new message: {}", Message.content);
        }
        _ => {}
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let framework = poise::Framework::build()
        .options(poise::FrameworkOptions {
            commands: vec![age(), register(), moderation::punish(), moderation::punishid(), owners::shutdown()],
            on_error: |error| Box::pin(on_error(error)),
            listener: |ctx, event, framework, userdata| {
                Box::pin(event_listeners(ctx, event, framework, userdata))
            },
            ..Default::default()
        })
        .token(&*CONFIG.bot.token)
        .intents(serenity::GatewayIntents::all())
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }));

    framework.run_autosharded().await.unwrap();
}
