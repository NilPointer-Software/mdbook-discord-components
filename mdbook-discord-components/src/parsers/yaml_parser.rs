use std::{
    fmt::Display,
    error::Error,
    collections::HashMap
};
use convert_case::{Casing, Case};
use serde::Deserialize;
use anyhow::Result;

#[cfg(feature = "http")]
use super::DISCORD_CLIENT;
use super::{DiscordCodeBlock, Parser};
use crate::components::{
    components::*,
    message::*,
    embed::*,
    *,
};

pub struct YamlParser;

impl Parser for YamlParser {
    fn new() -> Self { Self{} }

    fn parse<P: Parser>(&self, code_block: &DiscordCodeBlock<P>) -> Result<Components> {
        let mut components = Components::default();
        match serde_yaml::from_str::<Vec<YamlMessage>>(&code_block.code) {
            Ok(mut m) => {
                for (i, mut mess) in m.drain(..).enumerate() {
                    if !mess.is_valid() {
                        return Err(YamlParserError::new(format!("Invalid message #{}", i+1)).anyhow());
                    }
                    mess.prepare();
                    mess.push_to_tree(&mut components);
                }
            },
            Err(_) => {
                let mut message = serde_yaml::from_str::<YamlMessage>(&code_block.code)?;
                if !message.is_valid() {
                    return Err(YamlParserError::new("Invalid message").anyhow());
                }
                message.prepare();
                message.push_to_tree(&mut components);
            }
        };
        Ok(components)
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum YamlMessage {
    System(YamlSystemMessage),
    Basic(YamlBasicMessage),
}

#[derive(Debug, Deserialize)]
struct YamlBasicMessage {
    #[cfg(feature = "http")]
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
    attachments: Option<Vec<YamlAttachment>>,
    components: Option<Vec<YamlActionRow>>,

    content: String,
}

#[derive(Debug, Deserialize)]
struct YamlSystemMessage {
    r#type: SystemMessageType,
    channel_name: Option<bool>,
    timestamp: Option<String>,
    content: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum SystemMessageType {
    Alert,
    Boost,
    Call,
    Edit,
    Error,
    Join,
    Leave,
    MissedCall,
    Pin,
    Thread,
}

impl YamlMessage {
    fn push_to_tree(self, tree: &mut Components) {
        let (message_roles, node) = self.into_component();
        if let Some(roles) = message_roles {
            tree.roles.extend(roles);
        }
        tree.tree.push(node)
    }

    fn prepare(&mut self) {
        if let YamlMessage::Basic(ref mut basic) = self {
            if let Some(embed) = basic.embed.take() {
                let mut single = vec![embed];
                if let Some(embeds) = basic.embeds.as_mut() {
                    embeds.splice(0..0, single.drain(..));
                } else {
                    basic.embeds = Some(single);
                }
            }
        }
    }

    #[cfg(feature = "http")]
    fn is_valid(&self) -> bool { // TODO: Rewrite to a Result and return a proper Error with information of what is wrong
        match self {
            YamlMessage::Basic(ref basic) => {
                if basic.components.is_some() && basic.components.as_ref().unwrap().len() > 5 {
                    return false
                }
                !((basic.user_id.is_none() && basic.username.is_none()) || basic.content.is_empty())
            },
            YamlMessage::System(ref system) => {
                !system.content.is_empty()
            },
        }
    }

    #[cfg(not(feature = "http"))]
    fn is_valid(&self) -> bool {
        match self {
            YamlMessage::Basic(ref basic) => {
                if basic.components.is_some() && basic.components.as_ref().unwrap().len() > 5 {
                    return false
                }
                !(basic.username.is_none() || basic.content.is_empty())
            },
            YamlMessage::System(ref system) => {
                !system.content.is_empty()
            },
        }
    }

    fn into_component(self) -> (Option<HashMap<String, String>>, ComponentTree) {
        match self {
            YamlMessage::Basic(basic) => {
                let mut message = Message::default();
                #[cfg(feature = "http")]
                if let Some(user_id) = basic.user_id {
                    if let Some(user) = DISCORD_CLIENT.user(user_id) {
                        message.author = user.display_name();
                        message.avatar = Some(user.avatar_url());
                        message.bot = user.is_bot();
                    }
                }
                if let Some(username) = basic.username {
                    message.author = username;
                }
                if message.avatar.is_none() {
                    message.avatar = basic.avatar;
                }
                message.role_color = basic.color;
                message.timestamp = basic.timestamp;
                if let Some(bot) = basic.bot {
                    message.bot = bot;
                }
                if let Some(edited) = basic.edited {
                    message.edited = edited;
                }
                if let Some(ephemeral) = basic.ephemeral {
                    message.ephemeral = ephemeral;
                }
                if let Some(highlight) = basic.highlight {
                    message.highlight = highlight;
                }
                if let Some(verified) = basic.verified {
                    message.verified = verified;
                }
                let mut tree = vec![ComponentTree::Text(basic.content)];
                if let Some(embeds) = basic.embeds {
                    for mut embed in embeds {
                        embed.prepare();
                        tree.push(embed.into_component());
                    }
                }
                if let Some(mut reactions) = basic.reactions {
                    tree.push(ComponentTree::Node {
                        data: Reactions.into(),
                        nodes: reactions.drain(..).map(|r| r.into_component()).collect(),
                    })
                }
                if let Some(mut attachments) = basic.attachments {
                    tree.append(&mut attachments.drain(..).map(|a| a.into_component()).collect())
                }
                if let Some(mut components) = basic.components {
                    tree.push(ComponentTree::Node {
                        data: Attachments.into(),
                        nodes: components.drain(..).map(|v| v.into_component()).collect(),
                    })
                }
                (basic.roles, ComponentTree::Node {
                    data: message.into(),
                    nodes: tree,
                })
            },
            YamlMessage::System(system) => {
                let data = SystemMessage{
                    r#type: format!("{:?}", system.r#type).to_case(Case::Kebab),
                    timestamp: system.timestamp,
                    channel_name: system.channel_name.unwrap_or(false),
                };
                (None, ComponentTree::Node{
                    data: data.into(),
                    nodes: vec![ComponentTree::Text(system.content)],
                })
            },
        }
    }
}

#[derive(Debug, Deserialize)]
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
                data: EmbedDescription.into(),
                nodes: vec![ComponentTree::Text(description)],
            })
        }
        if let Some(mut fields) = self.fields {
            tree.push(ComponentTree::Node {
                data: EmbedFields.into(),
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
                data: EmbedFooter{
                    footer_image: footer.image,
                    timestamp: footer.timestap,
                }.into(),
                nodes: inner,
            })
        }
        ComponentTree::Node{
            data: embed.into(),
            nodes: tree,
        }
    }
}

#[derive(Debug, Deserialize)]
struct Author {
    text: String,
    image: Option<String>,
    url: Option<String>,
}

#[derive(Debug, Deserialize)]
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
            data: data.into(),
            nodes: vec![ComponentTree::Text(self.value)]
        }
    }
}

#[derive(Debug, Deserialize)]
struct Footer {
    text: Option<String>,
    image: Option<String>,
    timestap: Option<String>,
}

#[derive(Debug, Deserialize)]
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
            data: data.into(),
            nodes: vec![],
        }
    }
}

#[derive(Debug, Deserialize)]
struct YamlAttachment {
    url: String,
    height: Option<u64>,
    width: Option<u64>,
    alt: Option<String>,
}

impl YamlAttachment {
    fn into_component(self) -> ComponentTree {
        let data = Attachment{
            url: self.url,
            height: self.height,
            width: self.width,
            alt: self.alt,
        };
        ComponentTree::Node {
            data: data.into(),
            nodes: vec![],
        }
    }
}

#[derive(Debug, Deserialize)]
struct YamlActionRow(Vec<YamlButton>);

impl YamlActionRow {
    fn into_component(mut self) -> ComponentTree {
        ComponentTree::Node {
            data: ActionRow.into(),
            nodes: self.0.drain(..).map(|v| v.into_component()).collect(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct YamlButton {
    r#type: YamlButtonType,
    label: String,
    disabled: Option<bool>,
    emoji: Option<String>,
    emoji_name: Option<String>,
    url: Option<String>,
}

impl YamlButton {
    fn into_component(self) -> ComponentTree {
        let data = Button{
            r#type: format!("{:?}", self.r#type).to_case(Case::Kebab),
            disabled: self.disabled.unwrap_or(false),
            emoji: self.emoji,
            emoji_name: self.emoji_name,
            url: self.url,
        };
        ComponentTree::Node {
            data: data.into(),
            nodes: vec![ComponentTree::Text(self.label)],
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum YamlButtonType {
    Primary,
    Secondary,
    Success,
    Destructive,
}

#[derive(Debug)]
struct YamlParserError {
    message: String,
}

impl YamlParserError {
    fn new<S: Into<String>>(message: S) -> Self {
        Self { message: message.into() }
    }

    fn anyhow(self) -> anyhow::Error {
        anyhow::Error::new(self)
    }
}

impl Display for YamlParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for YamlParserError {}
