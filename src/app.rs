use crate::node::Node;
use egui_commonmark::CommonMarkCache;
use regex::Regex;
use serde::Serialize;
use sled::Db;
use uuid::Uuid;
use crate::graph::MyGraph;
#[derive(Serialize)]
pub(crate) struct AppState {
    pub(crate) current_node: Option<String>,
    pub(crate) search_query: String,
    #[serde(skip)]
    pub(crate) markdown_cache: CommonMarkCache,
    pub(crate) graph: MyGraph,
    pub(crate) show_graph: bool,
    #[serde(skip)]
    pub(crate) db: Db,
}
impl AppState {
    pub fn new() -> sled::Result<Self> {
        let db = sled::open("Brainbox")?;

        let state = AppState {
            current_node: None,
            search_query: String::new(),
            markdown_cache: CommonMarkCache::default(),
            graph: MyGraph::new(&db).unwrap(),
            show_graph: false,
            db,
        };

        Ok(state)
    }
    pub(crate) fn add_node(&mut self, title: String, content: String) -> sled::Result<String> {
        let id = Uuid::new_v4().to_string();
        let node = Node::new(id.clone(), title, content);
        let s_node = serde_json::to_vec(&node).unwrap();
        self.db.insert(id.clone(), s_node)?;
        Ok(id)
    }

    pub(crate) fn delete_node(&mut self, id: &str) -> sled::Result<()> {
        self.db.remove(id)?;
        if self.current_node.as_deref() == Some(id) {
            self.current_node = None;
        }
        // Update links in all nodes
        for item in self.db.iter() {
            let (key, value) = item?;
            let mut node: Node = serde_json::from_slice(&value).unwrap();
            node.links.retain(|link| link != id);
            let updated_node = serde_json::to_vec(&node).unwrap();
            self.db.insert(key, updated_node)?;
        }
        Ok(())
    }
    // tried using parser for events but link was not a part of the elements enum hence the regex
    pub(crate) fn update_links(&mut self, node_id: &str, content: &str) -> sled::Result<()> {
        let mut new_links = Vec::new();
        let re = Regex::new(r"\[\[(.+?)]]").unwrap();
        for cap in re.captures_iter(content) {
            if let Some(link_title) = cap.get(1) {
                for item in self.db.iter() {
                    let (key, value) = item?;
                    let node: Node = serde_json::from_slice(&value).unwrap();
                    println!("Checking node: {} with ID: {}", node.title, node.id);
                    if node.title == link_title.as_str() {
                        println!("Link found: {}", link_title.as_str());
                        if let Ok(key_str) = String::from_utf8(key.to_vec()) {
                            println!("Link ID found: {}", key_str);
                            new_links.push(key_str);
                        }
                    }
                }
            }
        }
        if let Some(value) = self.db.get(node_id)? {
            let mut node: Node = serde_json::from_slice(&value).unwrap();
            node.links = new_links;
            let updated_node = serde_json::to_vec(&node).unwrap();
            self.db.insert(node_id, updated_node)?;
            println!("Node id- {:?} title{} Updated node links: {:?}", node.id, node.title, node.links);
        }
        Ok(())
    }
}
