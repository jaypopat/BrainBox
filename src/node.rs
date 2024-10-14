use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Node {
    pub id: String,
    pub title: String,
    pub content: String,
    pub links: Vec<String>,
}

impl Node {
    pub(crate) fn new(id: String, title: String, content: String) -> Self {
        Self {
            id,
            title,
            content,
            links: Vec::<String>::new(),
        }
    }
}
