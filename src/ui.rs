use egui_commonmark::CommonMarkViewer;

use crate::app::AppState;
use crate::node::Node;

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut node_to_delete = None;

        // graph panel on the right
        egui::SidePanel::right("graph_panel").show(ctx, |ui| {
            if ui.button("View Graph").clicked() {
                self.graph.update(&self.db).expect("Failed to update graph");
                self.show_graph = !self.show_graph;
            }

            if self.show_graph {
                if let Err(e) = self.graph.show(ui) {
                    eprintln!("Failed to show graph: {:?}", e);
                }
            }
        });

        // node list panel on the left
        egui::SidePanel::left("node_list").show(ctx, |ui| {
            ui.heading("BrainBox");

            // Search Bar
            ui.horizontal(|ui| {
                ui.label("Search:");
                ui.text_edit_singleline(&mut self.search_query);
            });

            let mut node_to_select = None;
            let nodes_to_display: Vec<_> = self.db.iter().collect::<Result<Vec<_>, _>>().expect("Failed to get nodes");

            // Filtering nodes based on the search query
            for (id, node) in nodes_to_display.iter().filter(|(_, value)| {
                let node: Node = serde_json::from_slice(value).expect("Failed to deserialize node");
                node.title.to_lowercase().contains(&self.search_query.to_lowercase()) ||
                    node.content.to_lowercase().contains(&self.search_query.to_lowercase())
            }) {
                let node: Node = serde_json::from_slice(node).expect("Failed to deserialize node");
                if ui
                    .selectable_label(self.current_node.as_deref() == Some(&String::from_utf8(id.to_vec()).expect("Failed to convert id to string ")), &node.title)
                    .clicked()
                {
                    node_to_select = Some(id);
                }
            }

            if let Some(id) = node_to_select {
                self.current_node = Some(String::from_utf8(id.to_vec()).expect("Failed to convert id to string"));
            }
            if ui.button("New Note").clicked() {
                let id = self.add_node("New Note".to_string(), String::new());
                self.current_node = Some(id.expect("Failed to add new note"));
            }
        });

        // central panel for viewing and editing notes
        egui::CentralPanel::default().show(ctx, |ui| {
            let current_node_id = self.current_node.clone();

            if let Some(current_id) = current_node_id {
                if let Some(value) = self.db.get(&current_id).expect("Failed to get node") {
                    let mut node: Node = serde_json::from_slice(&value).expect("Failed to deserialize node");
                    // println!("Current node: {:?}", node);

                    // Update the links and refresh the graph
                    self.update_links(&current_id, &node.content).expect("Failed to update links");

                    let updated_value = self.db.get(&current_id).expect("Failed to get node").expect("Node not found");
                    node = serde_json::from_slice(&updated_value).expect("Failed to deserialize node");

                    let mut node_title = node.title.clone();
                    let mut node_content = node.content.clone();
                    let node_links = node.links.clone();

                    ui.horizontal(|ui| {
                        ui.heading("Title:");
                        ui.text_edit_singleline(&mut node_title);
                    });

                    ui.label("Content:");
                    let content_response = ui.add(
                        egui::TextEdit::multiline(&mut node_content)
                            .desired_width(f32::INFINITY)
                            .desired_rows(20)
                            .code_editor(),
                    );

                    if content_response.changed() {
                        self.update_links(&current_id, &node_content).expect("Failed to update links");
                    }

                    ui.separator();

                    // Render the markdown content
                    ui.label("Rendered Markdown:");
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        CommonMarkViewer::new().show(ui, &mut self.markdown_cache, &node_content);
                    });

                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.label("Links:");
                        for link in &node_links {
                            if let Ok(Some(linked_node)) = self.db.get(link) {
                                let linked_node: Node = serde_json::from_slice(&linked_node).expect("Failed to deserialize linked node");
                                if ui.button(&linked_node.title).clicked() {
                                    self.current_node = Some(link.clone());
                                }
                            }
                        }
                    });

                    if ui.button("Delete Note").clicked() {
                        node_to_delete = Some(current_id.clone());
                    }

                    node.title = node_title;
                    node.content = node_content;
                    let updated_node = serde_json::to_vec(&node).expect("Failed to serialize node");
                    self.db.insert(current_id.clone(), updated_node).expect("Failed to update node");
                } else {
                    self.current_node = None;
                }
            } else {
                ui.label("Select a note or create a new one");
            }
        });

        if let Some(id) = node_to_delete {
            self.delete_node(&id).expect("Failed to delete note");
            self.current_node = None;
        }
    }
}
