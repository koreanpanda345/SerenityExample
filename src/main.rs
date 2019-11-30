/**
! Requires the 'framework' feature flag be enabled in your project's `Cargo.toml`.
!
! This can be enabled by specifiying the feature in the dependecy section:
! ```toml
! [dependencies.serenity]
!  git = "https://github.com/serenity-rs/serenity.git"
!  features = ["framework", "standard_framework"]
! ```
*/
//! Create connection to the Mod file to interact with the commands.
mod commands;
//!Notice that we had to add this. without it, we can't use any of the serenity dependenics and can't interact with the commands.
//! Import files
use std::{
    collections::HashSet,
    env,
    sync::Arc,
};
use serenity::{
    client::bridge::gateway::ShardManager,
    framework::{
        standard::{
            Args, CheckResult, CommandOptions, CommandResult, CommandGroup,
            DispatchError, HelpOptions, help_commands, StandardFramework,
            macros::{groups, help, check}
        },
    },
    model::{event::ResumedEventm, gateway::Ready, id::UserId},
    prelude::*,
};

use log::{error, info};
use serenity::model::event::ResumedEvent;
use serenity::model::channel::Message;
//!We need to import the commands
use commands::{
    meta::*,
    owner::*,
};
//! This is for sharding, if you know that you're not going to use shards, the don't use lines 32-35
struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer{
    type Value = Arc<Mutex<ShardManager>>;
}

//! This is handling events.
struct Handler;
impl EventHandler for Handler{
//!Ready event
    fn ready(&self, _: Context, ready: Ready){
        info!("Connected as {}", ready.user.name);
    }
    //!Resumed Event
    fn resume(&self, _:Context, _: ResumedEvent){
        info!("Resumed");
    }
    //! This is a command group holds commands in groups, by sections.
    group!({
    name: "general",
    options: {},
    commands: [],
    });
}
//! We can have serenity make us a nice help embed.
//! notice that there is a thing with '#[]' this is a flag, or a tag, its similar to properties.
#[help]
#[individual_command_tip = "If you want more information about a specific command, just pass the command as argument"]
fn my_help(
    context: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult{
    help_commands::with_embeds(context, msg, args, help_options, groups, owners)
}

fn main(){
    // This will load the environment variables from the .env file.
    // if it can't find a .env, or can load it, it will throw this error vvv
    kankyo::load(true).expect("Failed to load .env file");

    //Initialize the logger to use environment variables
    env_logger::init();

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let mut client = Client::new(&token, Handler).expect("Err creating client");
    {
        let mut data = client.data.write();
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    let owners = match client.cache_and_http.http.get_current_application_info(){
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            set
        },
        Err(why) => panic!("Couldn't get application info: {:?}", why);
    };

    client.with_framework(StandardFramework::new()
        .configure(|c|c
            .owners(owners)
            .prefix("~"))
        //! Notice that the help and group names are all upper case. make sure you do this upper case, it will not work if you don't
        //! and if its a group, makes sure to add '_GROUP' to it so it knows that it needs to get the group.
        .help(&MY_HELP)
        .group(&GENERAL_GROUP));

    if let Err(why) = client.start(){
        error!("Client Error: {:?}", why);
    }

}