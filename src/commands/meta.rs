//! Import the serenity stuff that the command needs.
use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{
        CommandResult,
        macros::command,
    },
};
//! Since we didn't pass a name in the command flag, it will use the function's name, which in this case
//! is ping.
#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult{
    let _ = msg.channel_id.say(&ctx.http, "Pong!");
    //! Make sure you add this Ok(()). it will complain if you don't have this for any commands.
    Ok(())
}