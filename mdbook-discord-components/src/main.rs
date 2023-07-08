use std::{io, process};
use clap::{Arg, Command};
use mdbook::{
    preprocess::{CmdPreprocessor, Preprocessor},
    errors::Error,
};
use preprocessor::DiscordComponentsPreprocessor;

#[cfg(feature = "http")]
mod discord;
mod preprocessor;
mod parsers;
mod generators;
mod components;

fn main() {
    let matches = Command::new("discord-components-preprocessor")
        .about("A mdbook preprocessor which processes discord-components code blocks")
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        ).get_matches();

    let preprocessor = DiscordComponentsPreprocessor;

    if let Some(arg) = matches.subcommand_matches("supports") {
        if !preprocessor.supports_renderer(arg.get_one::<String>("renderer").expect("Required argument")) {
            process::exit(2);
        }
    } else if let Err(err) = handle_preprocessing(&preprocessor) {
        eprintln!("{err}");
        process::exit(1);
    }
    process::exit(0);
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    if ctx.mdbook_version != mdbook::MDBOOK_VERSION {
        // We should probably use the `semver` crate to check compatibility here...
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}
