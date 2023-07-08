use std::collections::HashMap;
use crate::generators::Generatable;

pub mod message;
pub mod embed;

#[derive(Default)]
pub struct Components {
    pub roles: HashMap<String, String>,
    pub tree: Vec<ComponentTree>,
}

pub enum ComponentTree {
    Text(String),
    Node{
        data: Box<dyn Generatable>,
        nodes: Vec<ComponentTree>,
    },
}
