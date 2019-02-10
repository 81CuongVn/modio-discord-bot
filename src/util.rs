use std::collections::HashMap;
use std::env;
use std::env::VarError;
use std::fmt;

use chrono::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use modio::auth::Credentials;
use serenity::client::EventHandler;
use serenity::client::{Client, Context};
use serenity::model::channel::Message;
use serenity::model::id::GuildId;

use crate::error::Error;
use crate::{DATABASE_URL, DISCORD_BOT_TOKEN, MODIO_API_KEY, MODIO_TOKEN};

pub type CliResult = std::result::Result<(), Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub struct Handler;

#[derive(Default)]
pub struct Settings {
    pub game: Option<u32>,
    pub prefix: Option<String>,
}

impl EventHandler for Handler {}

impl typemap::Key for Settings {
    type Value = HashMap<GuildId, Settings>;
}

pub struct PoolKey;

impl typemap::Key for PoolKey {
    type Value = Pool<ConnectionManager<SqliteConnection>>;
}

impl Settings {
    pub fn game(ctx: &mut Context, guild: GuildId) -> Option<u32> {
        let data = ctx.data.lock();
        let map = data.get::<Settings>().expect("failed to get settings map");
        map.get(&guild).and_then(|s| s.game)
    }

    pub fn set_game(ctx: &mut Context, guild: GuildId, game: u32) {
        let mut data = ctx.data.lock();
        data.get_mut::<Settings>()
            .expect("failed to get settings map")
            .entry(guild)
            .or_insert_with(Default::default)
            .game = Some(game);
    }

    pub fn prefix(ctx: &mut Context, msg: &Message) -> Option<String> {
        msg.guild_id.and_then(|id| {
            let data = ctx.data.lock();
            let map = data.get::<Settings>().expect("failed to get settings map");
            map.get(&id).and_then(|s| s.prefix.clone())
        })
    }

    pub fn set_prefix(ctx: &mut Context, guild: GuildId, prefix: Option<String>) {
        let mut data = ctx.data.lock();
        data.get_mut::<Settings>()
            .expect("failed to get settings map")
            .entry(guild)
            .or_insert_with(Default::default)
            .prefix = prefix;
    }
}

#[derive(Debug, Clone)]
pub enum Identifier {
    Id(u32),
    Search(String),
}

// impl FromStr & Display for Identifier {{{
impl std::str::FromStr for Identifier {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.parse::<u32>() {
            Ok(id) => Ok(Identifier::Id(id)),
            Err(_) => Ok(Identifier::Search(String::from(s))),
        }
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Identifier::Id(id) => id.fmt(fmt),
            Identifier::Search(id) => id.fmt(fmt),
        }
    }
}
// }}}

pub fn format_timestamp(seconds: i64) -> impl fmt::Display {
    NaiveDateTime::from_timestamp(seconds, 0).format("%Y-%m-%d %H:%M:%S")
}

pub fn var(key: &'static str) -> Result<String> {
    env::var(key).map_err(|e| Error::Env(key, e))
}

pub fn var_or<S: Into<String>>(key: &'static str, default: S) -> Result<String> {
    match env::var(key) {
        Ok(v) => Ok(v),
        Err(VarError::NotPresent) => Ok(default.into()),
        Err(e) => Err(Error::Env(key, e)),
    }
}

pub fn credentials() -> Result<Credentials> {
    use VarError::*;

    let api_key = env::var(MODIO_API_KEY);
    let token = env::var(MODIO_TOKEN);

    match (api_key, token) {
        (Ok(key), _) => Ok(Credentials::ApiKey(key)),
        (_, Ok(token)) => Ok(Credentials::Token(token)),
        (Err(NotUnicode(_)), Err(_)) => {
            Err("Environment variable 'MODIO_API_KEY' is not valid unicode".into())
        }
        (Err(_), Err(NotUnicode(_))) => {
            Err("Environment variable 'MODIO_TOKEN' is not valid unicode".into())
        }
        (Err(NotPresent), Err(NotPresent)) => {
            Err("Environment variable 'MODIO_API_KEY' or 'MODIO_TOKEN' not found".into())
        }
    }
}

pub fn discord() -> Result<Client> {
    let token = var(DISCORD_BOT_TOKEN)?;
    let database_url = var(DATABASE_URL)?;

    let mgr = ConnectionManager::new(database_url);
    let pool = Pool::new(mgr)?;

    let client = Client::new(&token, Handler)?;
    {
        let mut data = client.data.lock();
        data.insert::<PoolKey>(pool);
        data.insert::<Settings>(Default::default());
    }

    Ok(client)
}

// vim: fdm=marker
