use std::{
    fmt::Display,
    error::Error,
    collections::HashMap
};
use serde::Deserialize;
use mdbook::errors::Result;

use super::{DiscordCodeBlock, Parser, DISCORD_CLIENT};
use crate::components::{
    message::*,
    embed::*,
    *,
};

pub struct YamlParser;

impl Parser for YamlParser {
    fn new() -> Self { Self{} }

    fn parse<P: Parser>(&self, code_block: &DiscordCodeBlock<P>) -> Result<Components> {
        let mut messages = match serde_yaml::from_str::<Vec<YamlMessage>>(&code_block.code) {
            Ok(m) => {
                for (i, mess) in m.iter().enumerate() {
                    if !mess.is_valid() {
                        return Err(YamlParserError::new(format!("Invalid message #{}", i+1)).anyhow());
                    }
                }
                m
            },
            Err(_) => {
                let message = serde_yaml::from_str::<YamlMessage>(&code_block.code)?;
                if !message.is_valid() {
                    return Err(YamlParserError::new("Invalid message").anyhow());
                }
                vec![message]
            }
        };
        let mut components = Components::default();
        for mut message in messages.drain(..) {
            message.prepare();
            let (message_roles, node) = message.into_component();
            if let Some(roles) = message_roles {
                components.roles.extend(roles);
            }
            components.tree.push(node)
        }
        Ok(components)
    }
}

#[derive(Deserialize)]
struct YamlMessage {
    user_id: Option<u64>,
    username: Option<String>,
    avatar: Option<String>,
    color: Option<String>,
    timestamp: Option<String>,
    bot: Option<bool>,
    edited: Option<bool>,
    ephemeral: Option<bool>,
    highlight: Option<bool>,
    verified: Option<bool>,

    roles: Option<HashMap<String, String>>,

    embed: Option<YamlEmbed>,
    embeds: Option<Vec<YamlEmbed>>,

    reactions: Option<Vec<YamlReaction>>,

    content: String,
}

impl YamlMessage {
    fn prepare(&mut self) {
        if let Some(embed) = self.embed.take() {
            let mut single = vec![embed];
            if let Some(embeds) = self.embeds.as_mut() {
                embeds.splice(0..0, single.drain(..));
            } else {
                self.embeds = Some(single);
            }
        }
    }

    fn is_valid(&self) -> bool { // Message isn't valid if no username or user_id is provided, or the content is empty
        !((self.user_id.is_none() && self.username.is_none()) || self.content.is_empty())
    }

    fn into_component(self) -> (Option<HashMap<String, String>>, ComponentTree) {
        let mut message = Message::default();
        if let Some(user_id) = self.user_id {
            if let Some(user) = DISCORD_CLIENT.user(user_id) {
                message.author = user.display_name();
                message.avatar = Some(user.avatar_url());
                message.bot = user.is_bot();
            }
        }
        if let Some(username) = self.username {
            message.author = username;
        }
        message.avatar = self.avatar;
        message.role_color = self.color;
        message.timestamp = self.timestamp;
        if let Some(bot) = self.bot {
            message.bot = bot;
        }
        if let Some(edited) = self.edited {
            message.edited = edited;
        }
        if let Some(ephemeral) = self.ephemeral {
            message.ephemeral = ephemeral;
        }
        if let Some(highlight) = self.highlight {
            message.highlight = highlight;
        }
        if let Some(verified) = self.verified {
            message.verified = verified;
        }
        let mut tree = vec![ComponentTree::Text(self.content)];
        if let Some(embeds) = self.embeds {
            for mut embed in embeds {
                embed.prepare();
                tree.push(embed.into_component());
            }
        }
        if let Some(mut reactions) = self.reactions {
            tree.push(ComponentTree::Node {
                data: Box::new(Reactions{}),
                nodes: reactions.drain(..).map(|r| r.into_component()).collect(),
            })
        }
        (self.roles, ComponentTree::Node {
            data: Box::new(message),
            nodes: tree,
        })
    }
}

#[derive(Deserialize)]
struct YamlEmbed {
    title: Option<String>,
    url: Option<String>,
    color: Option<String>,

    description: Option<String>,

    image: Option<String>,
    thumbnail: Option<String>,
    timestamp: Option<String>,

    author: Option<Author>,
    fields: Option<Vec<Field>>,
    footer: Option<Footer>,
}

impl YamlEmbed {
    fn prepare(&mut self) {
        if let Some(timestamp) = self.timestamp.take() {
            if let Some(footer) = self.footer.as_mut() {
                footer.timestap = Some(timestamp);
            } else {
                self.footer = Some(Footer { text: None, image: None, timestap: Some(timestamp) })
            }
        }
    }

    fn into_component(self) -> ComponentTree {
        let mut embed = Embed::default();
        embed.embed_title = self.title;
        embed.url = self.url;
        embed.color = self.color;
        embed.image = self.image;
        embed.thumbnail = self.thumbnail;
        if let Some(author) = self.author {
            embed.author_name = Some(author.text);
            embed.author_url = author.url;
            embed.author_image = author.image;
        }
        let mut tree = Vec::<ComponentTree>::new();
        if let Some(description) = self.description {
            tree.push(ComponentTree::Node {
                data: Box::new(EmbedDescription{}),
                nodes: vec![ComponentTree::Text(description)],
            })
        }
        if let Some(mut fields) = self.fields {
            tree.push(ComponentTree::Node {
                data: Box::new(EmbedFields{}),
                nodes: fields.drain(..).map(|f| f.into_component()).collect(),
            })
        }
        if let Some(footer) = self.footer {
            let inner = if let Some(text) = footer.text {
                vec![ComponentTree::Text(text)]
            } else {
                vec![]
            };
            tree.push(ComponentTree::Node {
                data: Box::new(EmbedFooter{
                    footer_image: footer.image,
                    timestamp: footer.timestap,
                }),
                nodes: inner,
            })
        }
        ComponentTree::Node{
            data: Box::new(embed),
            nodes: tree,
        }
    }
}

#[derive(Deserialize)]
struct Author {
    text: String,
    image: Option<String>,
    url: Option<String>,
}

#[derive(Deserialize)]
struct Field {
    name: String,
    value: String,
    #[serde(default)]
    inline: bool,
    #[serde(default)]
    inline_index: usize,
}

impl Field {
    fn into_component(self) -> ComponentTree {
        let mut data = EmbedField{
            field_title: self.name,
            inline: self.inline,
            inline_index: None,
        };
        if self.inline {
            data.inline_index = Some(self.inline_index)
        }
        ComponentTree::Node {
            data: Box::new(data),
            nodes: vec![
                ComponentTree::Text(self.value),
            ]
        }
    }
}

#[derive(Deserialize)]
struct Footer {
    text: Option<String>,
    image: Option<String>,
    timestap: Option<String>,
}

#[derive(Deserialize)]
struct YamlReaction {
    emoji: String,
    name: Option<String>,
    count: Option<usize>,
    interactive: Option<bool>,
    reacted: Option<bool>,
}

impl YamlReaction {
    fn into_component(self) -> ComponentTree {
        let mut data = Reaction::default();
        data.emoji = Some(self.emoji);
        if let Some(name) = self.name {
            data.name = name;
        }
        if let Some(count) = self.count {
            data.count = count;
        }
        if let Some(interacive) = self.interactive {
            data.interactive = interacive
        }
        if let Some(reacted) = self.reacted {
            data.reacted = reacted
        }
        ComponentTree::Node {
            data: Box::new(data),
            nodes: vec![],
        }
    }
}

#[derive(Debug)]
struct YamlParserError {
    message: String,
}

impl YamlParserError {
    fn new<S: Into<String>>(message: S) -> Self {
        Self { message: message.into() }
    }

    fn anyhow(self) -> mdbook::errors::Error {
        mdbook::errors::Error::new(self)
    }
}

impl Display for YamlParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for YamlParserError {}
