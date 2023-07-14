use mdbook_discord_components_derive::Generatable;

#[derive(Generatable)]
#[gen(slot = "components")]
pub struct Attachments;

#[derive(Generatable)]
pub struct ActionRow;

#[derive(Generatable)]
pub struct Button {
    pub r#type: String,
    pub disabled: bool,
    pub emoji: Option<String>,
    pub emoji_name: Option<String>,
    pub url: Option<String>,
}
