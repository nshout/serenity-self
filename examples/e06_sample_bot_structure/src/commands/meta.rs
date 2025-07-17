use serenity_self::framework::standard::macros::command;
use serenity_self::framework::standard::CommandResult;
use serenity_self::model::prelude::*;
use serenity_self::prelude::*;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}
