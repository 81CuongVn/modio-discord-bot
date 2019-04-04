use modio::users::User;
use serenity::builder::CreateEmbedAuthor;
use serenity::framework::standard::CommandError;

pub type CommandResult = Result<(), CommandError>;
pub type EmbedField = (&'static str, String, bool);

pub mod prelude {
    pub use std::fmt::Write;

    pub use futures::{Future, Stream};
    pub use modio::filter::Operator;
    pub use modio::users::User;
    pub use modio::ModioListResponse;
    pub use serenity::builder::{CreateEmbedAuthor, CreateMessage};
    pub use serenity::client::Context;
    pub use serenity::framework::standard::ArgError;
    pub use serenity::model::channel::Message;
    pub use serenity::model::id::ChannelId;

    pub use super::{CommandResult, EmbedField, UserExt};
    pub use crate::db::Settings;
    pub use crate::error::Error;
    pub use crate::util::{format_timestamp, Identifier};
}

pub mod basic;
mod game;
mod mods;
pub mod subs;

pub use game::{Game, ListGames};
pub use mods::{ListMods, ModInfo, Popular};

pub trait UserExt {
    fn create_author(&self, _: CreateEmbedAuthor) -> CreateEmbedAuthor;
}

impl UserExt for User {
    fn create_author(&self, mut a: CreateEmbedAuthor) -> CreateEmbedAuthor {
        a = a.name(&self.username).url(&self.profile_url.to_string());
        if let Some(avatar) = &self.avatar {
            let icon = avatar.original.to_string();
            a = a.icon_url(&icon);
        }
        a
    }
}
