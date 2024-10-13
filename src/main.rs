use eframe::egui;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct AppState {
    nodes: HashMap<String, Node>,
    current_node: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
struct Node {
    id: String,
    title: String,
    content: String,
    links: Vec<String>,
}

impl Default for AppState {
    fn default() -> Self {
        let mut state = Self {
            nodes: HashMap::new(),
            current_node: None,
        };
        let welcome_id = state.add_node(
            "Welcome".to_string(),
            "Welcome to your Obsidian-like App!".to_string(),
        );
        state.current_node = Some(welcome_id);
        state
    }
}

impl AppState {
    fn add_node(&mut self, title: String, content: String) -> String {
        let id = Uuid::new_v4().to_string();
        let node = Node {
            id: id.clone(),
            title,
            content,
            links: vec![],
        };
        self.nodes.insert(id.clone(), node);
        id
    }

    fn delete_node(&mut self, id: &str) {
        self.nodes.remove(id);

        // If the current node is the one being deleted, set it to None
        if self.current_node.as_deref() == Some(id) {
            self.current_node = None;
        }

        // Removing links to this node from other nodes
        for node in self.nodes.values_mut() {
            node.links.retain(|link| link != id);
        }
    }

    fn add_link(&mut self, from: &str, to: &str) {
        if let Some(node) = self.nodes.get_mut(from) {
            if !node.links.contains(&to.to_string()) {
                node.links.push(to.to_string());
            }
        }
    }
    fn remove_link(&mut self, from: &str, to: &str) {
        if let Some(node) = self.nodes.get_mut(from) {
            node.links.retain(|link| link != to);
        }
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut node_to_delete = None;

        egui::SidePanel::left("node_list").show(ctx, |ui| {
            ui.heading("Notes");
            let mut node_to_select = None;
            let nodes_to_display: Vec<_> = self.nodes.iter().collect();
            for (id, node) in nodes_to_display {
                if ui
                    .selectable_label(self.current_node.as_deref() == Some(id), &node.title)
                    .clicked()
                {
                    node_to_select = Some(id.clone());
                }
            }
            if let Some(id) = node_to_select {
                self.current_node = Some(id);
            }
            if ui.button("New Note").clicked() {
                let id = self.add_node("New Note".to_string(), String::new());
                self.current_node = Some(id);
            }
        });

        if let Some(current_id) = self.current_node.clone() {
            if let Some(node) = self.nodes.get(&current_id).cloned() {
                let node_id = node.id.clone();
                let mut node_title = node.title;
                let mut node_content = node.content;
                let node_links = node.links.clone();

                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.heading("Title:");
                        ui.text_edit_singleline(&mut node_title);
                    });
                    ui.label("Content:");
                    ui.text_edit_multiline(&mut node_content);

                    ui.horizontal(|ui| {
                        ui.label("Links:");
                        let mut link_to_follow = None;
                        for link in &node_links {
                            if let Some(linked_node) = self.nodes.get(link) {
                                if ui.button(&linked_node.title).clicked() {
                                    link_to_follow = Some(link.clone());
                                }
                            }
                        }
                        if let Some(link) = link_to_follow {
                            self.current_node = Some(link);
                        }
                    });

                    if ui.button("Add Link").clicked() {
                        todo!()
                    }
                    if ui.button("Delete Note").clicked() {
                        node_to_delete = Some(node_id.clone());
                    }
                });

                self.nodes.insert(
                    current_id.clone(),
                    Node {
                        id: node_id,
                        title: node_title,
                        content: node_content,
                        links: node_links,
                    },
                );
            } else {
                self.current_node = None;
            }
        } else {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label("Select a note or create a new one");
            });
        }

        // Handle node deletion after the UI has been drawn
        if let Some(id) = node_to_delete {
            self.delete_node(&id);
            self.current_node = None;
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Brainbox - Second Brain",
        options,
        Box::new(|_cc| Ok(Box::new(AppState::default()))),
    )
}
