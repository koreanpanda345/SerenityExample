//!Import serenity, and use crate to get ShardManagerContainer.
use crate::ShardManagerContainer;
use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{
        CommandResult,
        macros::command,
    },
};
//!Notice that we add a owners_only flag. this is a check. it makes sure that only the owners can use this command.
#[command]
#[owners_only]
fn quit(ctx: &mut Context, msg: &Message)-> CommandResult{
    let data = ctx.data.read();
    if let Some(manager) = data.get::<ShardManagerContainer>(){
        manager.lock().shutdown_all();
    }
    else{
        let _ = msg.reply(&ctx, "THere was a problem getting the shard manager.");

        return Ok(());
    }

    let _ = msg.reply(&ctx, "Shutting down");

    Ok(())
}