use mdbook::{
    preprocess::{Preprocessor, PreprocessorContext},
    errors::Result,
    book::Book,
    utils,
    BookItem,
};
use pulldown_cmark::{Event, Tag, CodeBlockKind};
use crate::parsers::{DiscordCodeBlock, YamlParser};
use crate::generators::html::HTMLGenerator;

pub static PREPROCESSOR_NAME: &str = "mdbook-discord-components";
pub static BASE_CODE_BLOCK_NAME: &str = "discord";
pub struct DiscordComponentsPreprocessor;

impl Preprocessor for DiscordComponentsPreprocessor {
    fn name(&self) -> &str {
        PREPROCESSOR_NAME
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        let embed_script = ctx.config
            .get_preprocessor(self.name())
            .and_then(|c| c.get("embed-script"))
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        self.process_chapters(&mut book.sections, embed_script)?;

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }
}

impl DiscordComponentsPreprocessor {
    fn process_chapters<'a, I>(&self, items: I, embed_script: bool) -> Result<()> where I: IntoIterator<Item = &'a mut BookItem> + 'a {
        for item in items {
            if let BookItem::Chapter(ref mut chapter) = item {
                self.process_chapters(&mut chapter.sub_items, embed_script)?;

                let mut buf = String::with_capacity(chapter.content.len());
                let events = utils::new_cmark_parser(&chapter.content, false);

                let mut block: Option<DiscordCodeBlock<_>> = None;
                let mut block_name = BlockName::default();

                let mut buffer = Vec::new();
                for event in events {
                    if let Some(mut builder) = block.take() {
                        match event {
                            Event::Text(ref text) => {
                                builder.push_code(&**text);
                                block = Some(builder);
                            },
                            Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(ref info))) => {
                                assert_eq!(Some(0), (**info).find(&builder.block_name), "We must close our code block");
                                buffer.append(&mut builder.build::<HTMLGenerator>()?);
                            },
                            _ => block = Some(builder),
                        }
                    } else {
                        match event {
                            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(ref info)))
                                if (**info).find(&block_name.set("yaml")) == Some(0) => {
                                    block = Some(DiscordCodeBlock::<YamlParser>::new(block_name.string(), chapter.name.clone(), embed_script));
                                },
                            _ => buffer.push(event),
                        }
                    }
                }

                pulldown_cmark_to_cmark::cmark(buffer.iter(), &mut buf)?;

                chapter.content = buf;
            }
        }
        Ok(())
    }
}

#[derive(Default)]
struct BlockName {
    current: String,
}

impl BlockName {
    fn set(&mut self, new: &str) -> String {
        self.current = new.to_owned();
        self.string()
    }

    fn string(&self) -> String {
        format!("{} {}", BASE_CODE_BLOCK_NAME, self.current)
    }
}
