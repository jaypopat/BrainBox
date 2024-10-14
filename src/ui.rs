use egui_commonmark::CommonMarkViewer;
use crate::app::AppState;

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut node_to_delete = None;

        egui::SidePanel::left("node_list").show(ctx, |ui| {
            ui.heading("Notes");

            // Search Bar
            ui.horizontal(|ui| {
                ui.label("Search:");
                ui.text_edit_singleline(&mut self.search_query);
            });

            let mut node_to_select = None;
            let nodes_to_display: Vec<_> = self.nodes.iter().collect();

            // Filter nodes based on the search query
            for (id, node) in nodes_to_display.iter().filter(|(_, node)| {
                node.title.to_lowercase().contains(&self.search_query.to_lowercase()) ||
                    node.content.to_lowercase().contains(&self.search_query.to_lowercase())
            }) {
                if ui
                    .selectable_label(self.current_node.as_deref() == Some(id), &node.title)
                    .clicked()
                {
                    node_to_select = Some(id);
                }
            }

            if let Some(id) = node_to_select {
                self.current_node = Some(id.to_string());
            }
            if ui.button("New Note").clicked() {
                let id = self.add_node("New Note".to_string(), String::new());
                self.current_node = Some(id);
            }
        });

        if let Some(current_id) = self.current_node.clone() {
            if let Some(node) = self.nodes.get(&current_id) {
                let mut node_title = node.title.clone();
                let mut node_content = node.content.clone();
                let node_links = node.links.clone();

                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.heading("Title:");
                        ui.text_edit_singleline(&mut node_title);
                    });

                    ui.label("Content:");
                    let content_response = ui.add(
                        egui::TextEdit::multiline(&mut node_content)
                            .desired_width(f32::INFINITY)
                            .desired_rows(20)
                            .code_editor()
                    );

                    if content_response.changed() {
                        self.update_links(&current_id, &node_content);
                    }

                    ui.separator();

                    ui.label("Rendered Markdown:");
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        CommonMarkViewer::new().show(ui, &mut self.markdown_cache, &node_content);
                    });

                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.label("Links:");
                        for link in &node_links {
                            if let Some(linked_node) = self.nodes.get(link) {
                                if ui.button(&linked_node.title).clicked() {
                                    self.current_node = Some(link.clone());
                                }
                            }
                        }
                    });

                    if ui.button("Delete Note").clicked() {
                        node_to_delete = Some(current_id.clone());
                    }
                });

                if let Some(node) = self.nodes.get_mut(&current_id) {
                    node.title = node_title;
                    node.content = node_content;
                }
            } else {
                self.current_node = None;
            }
        } else {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label("Select a note or create a new one");
            });
        }

        if let Some(id) = node_to_delete {
            self.delete_node(&id);
            self.current_node = None;
        }
    }
}
