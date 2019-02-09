use std::sync::Arc;

use serenity::client::Context;
use serenity::framework::standard::{Args, Command, CommandOptions};
use serenity::model::channel::Message;
use serenity::model::permissions::Permissions;

use crate::commands::CommandResult;

pub struct Invite;

impl Command for Invite {
    fn options(&self) -> Arc<CommandOptions> {
        Arc::new(CommandOptions {
            desc: Some("Displays a link to invite modbot.".to_string()),
            ..Default::default()
        })
    }

    fn execute(&self, _: &mut Context, msg: &Message, _: Args) -> CommandResult {
        serenity::http::raw::get_current_user().and_then(|u| {
            let perms = Permissions::READ_MESSAGES
                | Permissions::SEND_MESSAGES
                | Permissions::EMBED_LINKS
                | Permissions::ADD_REACTIONS;
            let url = u.invite_url(perms)?;
            msg.channel_id.send_message(|m| {
                m.embed(|e| e.description(format!("Visit the link to [invite modbot]({}).", url)))
            })
        })?;
        Ok(())
    }
}

pub struct Guide;

impl Command for Guide {
    fn options(&self) -> Arc<CommandOptions> {
        Arc::new(CommandOptions {
            desc: Some("Link to 'Getting Started' blog post.".to_string()),
            aliases: vec!["tutorial".to_string(), "getting-started".to_string()],
            ..Default::default()
        })
    }

    fn execute(&self, _: &mut Context, msg: &Message, _: Args) -> CommandResult {
        msg.channel_id
            .say("https://apps.mod.io/guides/getting-started")?;
        Ok(())
    }
}