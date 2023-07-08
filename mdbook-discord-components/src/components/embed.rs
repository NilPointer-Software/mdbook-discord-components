use mdbook_discord_components_derive::Generatable;

#[derive(Default, Generatable)]
#[gen(slot = "embeds")]
pub struct Embed {
    pub embed_title: Option<String>,
    pub url: Option<String>,
    pub color: Option<String>,

    pub image: Option<String>,
    pub thumbnail: Option<String>,

    pub author_name: Option<String>,
    pub author_url: Option<String>,
    pub author_image: Option<String>,
}

#[derive(Generatable)]
#[gen(slot = "description")]
pub struct EmbedDescription;

#[derive(Generatable)]
#[gen(slot = "fields")]
pub struct EmbedFields;

#[derive(Generatable)]
pub struct EmbedField {
    pub field_title: String,
    pub inline: bool,
    pub inline_index: Option<usize>,
}

#[derive(Generatable)]
#[gen(slot = "footer")]
pub struct EmbedFooter {
    pub footer_image: Option<String>,
    pub timestamp: Option<String>,
}
