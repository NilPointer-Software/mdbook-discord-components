use mdbook_discord_components_derive::Generatable;

#[derive(Default, Generatable)]
pub struct Message { // TODO: implement attrubute macros to automate implementing Generatable
    pub author: String,
    pub avatar: Option<String>,
    pub bot: bool,
    pub edited: bool,
    pub ephemeral: bool,
    pub highlight: bool,
    pub role_color: Option<String>,
    pub timestamp: Option<String>,
    pub verified: bool,
}

#[derive(Generatable)]
pub struct SystemMessage {
    pub r#type: String,
    pub timestamp: Option<String>,
    pub channel_name: bool,
}

#[derive(Generatable)]
#[gen(slot = "reactions")]
pub struct Reactions;

#[derive(Generatable)]
pub struct Reaction {
    pub name: String,
    pub emoji: Option<String>,
    pub count: usize,
    pub interactive: bool,
    pub reacted: bool,
}

impl Default for Reaction {
    fn default() -> Self {
        Self{
            name: ":emoji:".to_owned(),
            emoji: None,
            count: 1,
            interactive: false,
            reacted: false,
        }
    }
}

#[derive(Generatable)]
#[gen(slot = "attachments")]
pub struct Attachment {
    pub url: String,
    pub height: Option<u64>,
    pub width: Option<u64>,
    pub alt: Option<String>,
}

#[derive(Generatable)]
#[gen(slot = "reply")]
pub struct Reply {
    pub author: String,
    pub avatar: Option<String>,
    pub role_color: Option<String>,
    pub attachment: Option<bool>,
    pub edited: Option<bool>,
    pub bot: Option<bool>,
    pub verified: Option<bool>,
    pub mentions: Option<bool>,
    pub op: Option<bool>,
    pub command: Option<bool>,
}

#[derive(Generatable)]
#[gen(slot = "reply")]
pub struct Command {
    pub command: String,
    pub author: String,
    pub avatar: Option<String>,
    pub role_color: Option<String>,
}
