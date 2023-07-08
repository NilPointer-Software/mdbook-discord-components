use mdbook_discord_components::{parsers::{DiscordCodeBlock, YamlParser}, generators::html::HTMLGenerator};
use pulldown_cmark::Event;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub fn parse(parser_name: String, input: String) -> Result<String, String> {
    let mut context = match parser_name.as_str() {
        "yaml" => DiscordCodeBlock::<YamlParser>::new("yaml".to_owned(), "wasm input".to_owned(), false),
        _ => return Err("".to_owned()),
    };

    context.push_code(input);

    let events = context.build::<HTMLGenerator>();
    match events {
        Ok(ev) => {
            if let Some(Event::Html(inner)) = ev.first() {
                Ok(inner.to_string())
            } else {
                Err("invalid event generation".to_owned())
            }
        },
        Err(err) => Err(err.to_string())
    }
}

#[wasm_bindgen]
pub fn available_parsers() -> js_sys::Array {
    let array = js_sys::Array::new();
    array.push(&JsValue::from_str("yaml"));
    array
}
