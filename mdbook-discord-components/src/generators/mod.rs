use std::collections::HashMap;
use regex::{Regex, Captures};
use pulldown_cmark::Event;
use anyhow::Result;

use crate::components::Components;

pub mod html;

lazy_static::lazy_static! {
    static ref MENTION_REGEX: Regex = Regex::new("<(!?)(t:|e:|@|#)(.*?)>").unwrap();
}

pub trait Generator {
    fn new() -> Self;
    fn generate<'a>(&self, components: Components) -> Result<Event<'a>>;
}

pub trait Generatable {
    fn name(&self) -> &str;
    fn attrubutes(self: Box<Self>) -> HashMap<String, String>;
}

fn format_mentions(roles: &HashMap<String, String>, text: String) -> String {
    MENTION_REGEX.replace_all(&text, |captures: &Captures| {
        if &captures[2] == "t:" {
            return format!("<discord-time>{}</discord-time>", &captures[3]);
        }
        if &captures[2] == "e:" {
            return format!("<discord-custom-emoji url=\"{}\"></discord-custom-emoji>", &captures[3]);
        }
        let attr = if &captures[2] == "#" {
            " type=\"channel\"".to_owned()
        } else if roles.contains_key(&captures[3]) {
            format!(" type=\"role\" color=\"{}\"", roles.get(&captures[3]).unwrap())
        } else {
            String::new()
        } + if &captures[1] == "!" {
            " highlight"
        } else {
            ""
        };
        format!("<discord-mention{}>{}</discord-mention>", attr, &captures[3])
    }).into_owned()
}
