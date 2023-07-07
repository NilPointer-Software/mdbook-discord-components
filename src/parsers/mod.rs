use std::fmt::Display;
use pulldown_cmark::Event;
use mdbook::errors::{Result, Error};

mod yaml_parser;

pub use yaml_parser::YamlParser;

use crate::{
    components::Components,
    generators::Generator,
    discord::Discord,
};

pub static DISCORD_COMPONENTS_SCRIPT: &str = "<script type=\"module\" src=\"https://unpkg.com/@skyra/discord-components-core\"></script>\n";

lazy_static::lazy_static! {
    static ref DISCORD_CLIENT: Discord = Discord::default();
}

pub trait Parser: Sized {
    fn new() -> Self;
    fn parse<P: Parser>(&self, code_block: &DiscordCodeBlock<P>) -> Result<Components>;
}

pub struct DiscordCodeBlock<P: Parser> {
    pub block_name: String,
    chapter_name: String,
    embed_script: bool,
    code: String,
    parser: P,
}

impl<P: Parser> DiscordCodeBlock<P> {
    pub fn new(block_name: String, chapter_name: String, embed_script: bool) -> Self {
        Self { block_name, chapter_name, embed_script, code: String::new(), parser: P::new() }
    }

    pub fn push_code<S: Into<String>>(&mut self, code: S) {
        self.code.push_str(&code.into());
    }

    pub fn build<'a, G: Generator>(&self) -> Result<Vec<Event<'a>>> {
        match self.parser.parse(self) {
            Ok(result) => {
                let res = G::new().generate(result)?;
                Ok(if self.embed_script {
                    vec![
                        Event::Html((DISCORD_COMPONENTS_SCRIPT.to_owned()).into()),
                        res,
                    ]
                } else {
                    vec![res]
                })
            },
            Err(err) => {
                Err(Error::new(ParseError{
                    chapter: self.chapter_name.clone(),
                    source: err,
                }))
            },
        }
    }
}

#[derive(Debug)]
struct ParseError {
    chapter: String,
    source: Error,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse a discord code block in chapter '{}'! Error: {}", self.chapter, self.source)
    }
}

impl std::error::Error for ParseError {}
