use std::collections::HashMap;
use pulldown_cmark::Event;
use anyhow::Result;

use super::Generator;
use crate::components::{Components, ComponentTree};

pub struct HTMLGenerator;

impl Generator for HTMLGenerator {
    fn new() -> Self {
        HTMLGenerator{}
    }

    fn generate<'a>(&self, mut components: Components) -> Result<Event<'a>> {
        let html = "<discord-messages>\n".to_owned() +
            &components.tree.drain(..).map(|tree| generate_components(&components.roles, tree, 1)).collect::<String>() +
        "</discord-messages>";
        eprintln!("HTML Generator generated following:\n{html}");
        Ok(Event::Html(html.into()))
    }
}

fn generate_components(roles: &HashMap<String, String>, component: ComponentTree, indent_size: usize) -> String {
    let indent = "    ".repeat(indent_size);
    indent.clone() + &match component {
        ComponentTree::Text(text) => super::format_mentions(roles, text.trim_end_matches("\n").replace("\n", "<br />").to_owned()),
        ComponentTree::Node { data, mut nodes } => {
            let name = data.name().to_owned();
            let mut attr = data.attrubutes();
            let attr = if attr.is_empty() {
                String::new()
            } else {
                attr.drain()
                    .map(|(k, v)| {
                        " ".to_owned() + &if v.is_empty() {
                            k
                        } else {
                            k + "=\"" + &v +"\""
                        }
                    }).collect::<String>()
            };
            "<".to_owned() + &name + &attr + ">\n" +
            &nodes.drain(..).map(|n| generate_components(roles, n, indent_size + 1)).collect::<String>() +
            &indent + "</" + &name + ">"
        },
    } + "\n"
}
