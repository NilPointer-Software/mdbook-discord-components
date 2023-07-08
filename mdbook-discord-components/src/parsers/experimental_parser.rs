use std::collections::HashMap;

use pulldown_cmark::{Event, CowStr};
use regex::Regex;

const DISCORD_MESSAGES_START_TAG: Event<'static> = Event::Html(CowStr::Borrowed("<discord-messages>"));
const DISCORD_MESSAGES_END_TAG: Event<'static> = Event::Html(CowStr::Borrowed("</discord-messages>"));

pub struct DiscordCodeBlock { // TODO: rewrite into a proper parser, this is just for a proof of concept
    code: String,
}

impl DiscordCodeBlock {
    pub fn new() -> Self {
        Self { code: String::new() }
    }

    pub fn push_code<S: Into<String>>(&mut self, code: S) {
        self.code.push_str(&code.into());
    }

    pub fn build<'a>(&self) -> Vec<Event<'a>> {
        let mut buffer = vec![DISCORD_MESSAGES_START_TAG];
        let mut message = Message::default();

        for line in self.code.lines() {
            eprintln!("{line}");
            if line.starts_with(">") {
                let mut split = line[1..].trim().split_whitespace();
                if let Some(first) = split.next() {
                    let mut dirty = true;
                    match first {
                        "author" => message.username(split.collect::<String>()),
                        "avatar" => message.avatar(split.collect::<String>()),
                        "color" => message.color(split.collect::<String>()),
                        "bot" => message.bot = true,
                        "role" => {
                            let color = split.clone().last().unwrap(); // TODO: This might panic
                            message.role(split.collect::<String>(), color.to_owned());
                        },
                        _ => dirty = false,
                    }
                    if dirty {
                        continue;
                    }
                }
            } else if line == "" {
                buffer.push(message.build_event());
                message = Message::default();
                continue;
            }
            message.push_line(line.to_owned());
        }

        if !message.is_empty() {
            buffer.push(message.build_event());
        }
        buffer.push(DISCORD_MESSAGES_END_TAG);
        buffer
    }
}

#[derive(Default)]
struct Message {
    username: Option<String>,
    avatar: Option<String>,
    color: Option<String>,
    roles: HashMap<String, String>,
    bot: bool,
    message: String,
}

impl Message {
    fn is_empty(&self) -> bool {
        !(
            self.username.is_some() ||
            self.avatar.is_some() ||
            self.color.is_some() ||
            !self.roles.is_empty() ||
            self.bot ||
            !self.message.is_empty()
        )
    }

    fn username(&mut self, username: String) {
        self.username = Some(username);
    }

    fn avatar(&mut self, avatar: String) {
        self.avatar = Some(avatar);
    }

    fn color(&mut self, color: String) {
        self.color = Some(color);
    }

    fn role(&mut self, name: String, color: String) {
        self.roles.insert(name, color);
    }

    fn push_line(&mut self, line: String) {
        eprintln!("Pushing line: {line}");
        self.message.push_str(&line);
        eprintln!("Current message: {}", self.message);
    }
    
    fn build(mut self) -> String {
        let mut res = "<discord-message".to_owned();

        if let Some(username) = self.username {
            res += &format!(" author=\"{username}\"");
        }
        if let Some(avatar) = self.avatar {
            res += &format!(" avatar=\"{avatar}\"");
        }
        if let Some(color) = self.color {
            res += &format!(" roleColor=\"{color}\"");
        }
        if self.bot {
            res += " bot";
        }
        res += ">\n";

        for capture in MENTION_REGEX.captures_iter(&self.message.clone()) { // TODO: Maybe??
            let mut mention = "<discord-mention".to_owned();
            if &capture[1] == "@" {
                if let Some(color) = self.roles.get(&capture[2]) {
                    mention += &format!(" type=\"role\" color=\"{color}\"");
                }
            } else {
                mention += " type=\"channel\"";
            }
            if &capture[0] == "!" {
                mention += " highlight";
            }
            capture.expand(&format!("{mention}>{}</discord-mention>", &capture[2]), &mut self.message)
        }

        res + &self.message + "\n" + "</discord-message>\n"
    }

    fn build_event<'a>(self) -> Event<'a> {
        let built = self.build();
        eprintln!("Built Message: {}", &built);
        Event::Html(built.into())
    }
}
