use crate::node::Node;
use egui::Ui;
use egui_graphs::{Graph, GraphView};
use petgraph::stable_graph::{NodeIndex, StableGraph};
use petgraph::Directed;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct MyGraph {
    #[serde(skip)]
    pub graph: Graph<String, ()>,
    #[serde(skip)]
    node_map: NodeMapType,
}

// type aliases
type GraphType = StableGraph<String, (), Directed>;
type NodeMapType = HashMap<String, NodeIndex>;
impl MyGraph {
    pub(crate) fn new(db: &sled::Db) -> sled::Result<Self> {
        let (g, node_map) = Self::create_graph_from_db(db)?;
        Ok(Self {
            graph: Graph::from(&g),
            node_map,
        })
    }

    pub fn show(&mut self, ui: &mut Ui) -> sled::Result<()> {
        let mut graph_view = GraphView::new(&mut self.graph);
        ui.add(&mut graph_view);
        Ok(())
    }

    pub fn update(&mut self, db: &sled::Db) -> sled::Result<()> {
        let (g, new_node_map) = Self::create_graph_from_db(db)?;
        self.graph = Graph::from(&g);
        self.node_map = new_node_map;
        Ok(())
    }

    fn create_graph_from_db(db: &sled::Db) -> sled::Result<(GraphType, NodeMapType)> {
        let mut g: StableGraph<String, (), Directed> = StableGraph::new();
        let mut node_map = HashMap::new();

        // Create nodes
        Self::create_nodes(db, &mut g, &mut node_map)?;

        // Create edges
        Self::create_edges(db, &mut g, &node_map)?;

        // returning graph as a tuple
        Ok((g, node_map))
    }

    fn create_nodes(
        db: &sled::Db,
        g: &mut GraphType,
        node_map: &mut NodeMapType,
    ) -> sled::Result<()> {
        for item in db.iter() {
            let (key, value) = item?;
            let node: Node = serde_json::from_slice(&value).expect("Failed to deserialize node");
            let id = String::from_utf8(key.to_vec()).expect("Failed to convert key to string");
            let node_index = g.add_node(node.title.clone());
            node_map.insert(id, node_index);
        }
        Ok(())
    }

    fn create_edges(db: &sled::Db, g: &mut GraphType, node_map: &NodeMapType) -> sled::Result<()> {
        for item in db.iter() {
            let (key, value) = item?;
            let node: Node = serde_json::from_slice(&value).expect("Failed to deserialize node");
            let id = String::from_utf8(key.to_vec()).expect("Failed to convert key to string");
            if let Some(&from_index) = node_map.get(&id) {
                for link in &node.links {
                    if let Some(&to_index) = node_map.get(link) {
                        g.add_edge(from_index, to_index, ());
                    }
                }
            }
        }
        Ok(())
    }
}
