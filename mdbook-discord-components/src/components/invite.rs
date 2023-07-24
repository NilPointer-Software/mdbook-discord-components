use mdbook_discord_components_derive::Generatable;

#[derive(Generatable)]
pub struct Invite { // This model is intentionally incomplete, since in real Discord a user can "change" only the following data
    pub online: u64,
    pub members: u64,
    pub name: String,
    pub icon: Option<String>,
    pub partnered: bool,
    pub verified: bool,
}
