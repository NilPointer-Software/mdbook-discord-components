use std::{
    collections::HashMap,
    fmt::Display,
    error::Error,
    sync::RwLock,
    env,
};
use mdbook::errors::Result;
use serde_aux::field_attributes::deserialize_number_from_string;
use serde::Deserialize;
use oxhttp::{
    model::{Request, Method, Status},
    Client,
};

static DISCORD_API: &str = "https://discord.com/api/v10/";
static DISCORD_CDN: &str = "https://cdn.discordapp.com/";

pub struct Discord {
    client: Client,
    token: Option<String>,
    cache: RwLock<HashMap<u64, User>>,
}

impl Default for Discord {
    fn default() -> Self {
        let token = if let Ok(token) = env::var("DISCORD_TOKEN") {
            Some(token)
        } else {
            None
        };
        Self { token, client: Client::new(), cache: RwLock::default() }
    }
}

impl Discord {
    pub fn user(&self, user_id: u64) -> Option<User> {
        if let Some(token) = self.token.clone() {
            if let Some(user) = self.cache.read().unwrap().get(&user_id) {
                return Some(user.clone())
            }
            match self.inner_user(user_id, &token) {
                Ok(user) => return Some(user),
                Err(err) => eprintln!("Failed to fetch user of ID '{user_id}. Error: {err}")
            }
        }
        None
    }

    fn inner_user(&self, user_id: u64, token: &str) -> Result<User> {
        let req = Request::builder(
            Method::GET,
            format!("{}users/{user_id}", DISCORD_API).parse()?)
            .with_header("Authorization", format!("Bot {token}"))?
            .build();
        let result = self.client.request(req)?;
        if result.status() != Status::OK {
            return Err(DiscordError::new(format!("Non-200 status. Body: {}", result.into_body().to_string().unwrap_or("Failed to read request".to_owned()))).anyhow())
        }
        let user = serde_json::from_slice::<User>(&result.into_body().to_vec()?)?;
        self.cache.write().unwrap().insert(user_id, user);
        Ok(self.cache.read().unwrap().get(&user_id).unwrap().clone())
    }
}

#[derive(Deserialize, Clone)]
pub struct User {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    id: u64,
    discriminator: String,
    username: String,
    global_name: Option<String>,
    avatar: Option<String>,
    bot: Option<bool>,
}

impl User {
    pub fn display_name(&self) -> String {
        if let Some(global_name) = self.global_name.as_ref() {
            global_name.clone()
        } else {
            self.username.clone()
        }
    }

    pub fn avatar_url(&self) -> String {
        if let Some(hash) = self.avatar.as_ref() {
            format!("{}avatars/{}/{}.png", DISCORD_CDN, self.id, hash)
        } else if &self.discriminator == "0" {
            format!("{}embed/avatars/{}.png", DISCORD_CDN, (self.id >> 22) % 6)
        } else {
            let discriminator = self.discriminator.parse::<u16>().unwrap_or(0_u16);
            format!("{}embed/avatars/{}.png", DISCORD_CDN, discriminator % 5)
        }
    }

    pub fn is_bot(&self) -> bool {
        if let Some(bot) = self.bot {
            return bot;
        }
        false
    }
}

#[derive(Debug)]
struct DiscordError {
    message: String,
}

impl DiscordError {
    fn new<S: Into<String>>(message: S) -> Self {
        Self { message: message.into() }
    }

    fn anyhow(self) -> mdbook::errors::Error {
        mdbook::errors::Error::new(self)
    }
}

impl Display for DiscordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for DiscordError {}
