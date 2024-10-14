use std::collections::HashMap;
use egui_commonmark::CommonMarkCache;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::node::Node;
#[derive(Serialize, Deserialize)]
pub(crate) struct AppState {
    pub(crate) nodes: HashMap<String, Node>,
    pub(crate) current_node: Option<String>,
    pub(crate) search_query: String,
    #[serde(skip)]
    pub(crate) markdown_cache: CommonMarkCache,
}
impl Default for AppState {
    fn default() -> Self {
        let mut state = Self {
            nodes: HashMap::new(),
            current_node: None,
            search_query: String::new(), // Initialize search_query
            markdown_cache: CommonMarkCache::default(),
        };
        let welcome_id = state.add_node(
            "Welcome".to_string(),
            "# Welcome to your Obsidian-like App!\n\nThis is a **Markdown** editor. You can use all standard Markdown syntax here.\n\n- Create lists\n- Add **bold** and *italic* text\n- Create [[links]] to other notes\n\nEnjoy organizing your thoughts!".to_string(),
        );
        // seeding for testing purposes
        state.add_node("Node 1".to_string(), "This is the content of Node 1".to_string());
        state.add_node("Node 2".to_string(), "This is the content of Node 2".to_string());
        state.add_node("Node 3".to_string(), "This is the content of Node 3".to_string());
        state.add_node("Node 4".to_string(), "This is the content of Node 4".to_string());
        state.add_node("Node 5".to_string(), "This is the content of Node 5".to_string());
        state.current_node = Some(welcome_id);

        state
    }
}
impl AppState {
    pub(crate) fn add_node(&mut self, title: String, content: String) -> String {
        let id = Uuid::new_v4().to_string();
        let node = Node::new(id.clone(), title, content);
        self.nodes.insert(id.clone(), node);
        id
    }

    pub(crate) fn delete_node(&mut self, id: &str) {
        self.nodes.remove(id);
        if self.current_node.as_deref() == Some(id) {
            self.current_node = None;
        }
        for node in self.nodes.values_mut() {
            node.links.retain(|link| link != id);
        }
    }
    pub(crate) fn update_links(&mut self, node_id: &str, content: &str) {
        // used regex - tried to use parser and event based approach but link was not part of the elements enum
        let mut new_links = Vec::new();
        for cap in regex::Regex::new(r"\[\[(.+?)]]").unwrap().captures_iter(content) {
            if let Some(link_title) = cap.get(1) {
                if let Some((id, _)) = self.nodes.iter().find(|(_, node)| node.title == link_title.as_str()) {
                    new_links.push(id.clone());
                }
            }
        }
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.links = new_links;
        }
    }
}

