use egui::{Color32, FontFamily, FontId, RichText, Rounding, Stroke, Vec2};
use egui_commonmark::CommonMarkViewer;
use sled::IVec;

use crate::app::AppState;
use crate::node::Node;

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut node_to_delete = None;

        let mut visuals = egui::Visuals::dark();
        visuals.window_rounding = Rounding::same(12.0);
        visuals.widgets.noninteractive.rounding = Rounding::same(8.0);
        visuals.widgets.inactive.rounding = Rounding::same(8.0);
        visuals.widgets.hovered.rounding = Rounding::same(10.0);
        visuals.widgets.active.rounding = Rounding::same(10.0);
        visuals.selection.stroke = Stroke::new(1.5, Color32::from_rgb(100, 200, 255));
        visuals.window_shadow.color = Color32::from_black_alpha(80);
        ctx.set_visuals(visuals);

        // Sidebar for navigation
        egui::SidePanel::left("sidebar")
            .resizable(true)
            .default_width(260.0)
            .min_width(220.0)
            .max_width(320.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(12.0);
                    ui.heading(
                        RichText::new("BrainBox")
                            .size(32.0)
                            .color(Color32::from_rgb(100, 200, 255)),
                    );
                    ui.add_space(12.0);
                });

                // Search Bar
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut self.search_query)
                            .hint_text("🔍 Search notes...")
                            .desired_width(ui.available_width() - 40.0),
                    );
                    if ui.button("❌").clicked() {
                        self.search_query.clear();
                    }
                });
                ui.add_space(10.0);

                // Note List
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let nodes_to_display: Vec<(IVec, IVec)> = self
                        .db
                        .iter()
                        .collect::<Result<Vec<_>, _>>()
                        .expect("Failed to get nodes");

                    // search results
                    for (id, node) in nodes_to_display.iter().filter(|(_, value)| {
                        let node: Node =
                            serde_json::from_slice(value).expect("Failed to deserialize node");
                        node.title
                            .to_lowercase()
                            .contains(&self.search_query.to_lowercase())
                            || node
                                .content
                                .to_lowercase()
                                .contains(&self.search_query.to_lowercase())
                    }) {
                        let node: Node =
                            serde_json::from_slice(node).expect("Failed to deserialize node");
                        let is_selected = self.current_node.as_deref()
                            == Some(
                                &String::from_utf8(id.to_vec())
                                    .expect("Failed to convert id to string"),
                            );

                        ui.add_space(6.0);
                        let response =
                            ui.selectable_label(is_selected, format!("📝 {}", node.title));

                        if response.clicked() {
                            self.current_node = Some(
                                String::from_utf8(id.to_vec())
                                    .expect("Failed to convert id to string"),
                            );
                        }
                    }
                });
            });

        // Main content area
        egui::CentralPanel::default().show(ctx, |ui| {
            let current_node_id = self.current_node.clone();
            if let Some(current_id) = current_node_id {
                if let Some(value) = self.db.get(&current_id).expect("Failed to get node") {
                    let mut node: Node =
                        serde_json::from_slice(&value).expect("Failed to deserialize node");

                    ui.vertical(|ui| {
                        // Title of note
                        ui.add_space(10.0);
                        let _title_response = ui.add(
                            egui::TextEdit::singleline(&mut node.title)
                                .font(FontId::new(24.0, FontFamily::Proportional))
                                .desired_width(ui.available_width()),
                        );
                        ui.add_space(10.0);

                        // user input box
                        let content_response = ui.add_sized(
                            Vec2::new(ui.available_width(), 320.0),
                            egui::TextEdit::multiline(&mut node.content)
                                .code_editor()
                                .desired_width(f32::INFINITY)
                                .font(FontId::new(18.0, FontFamily::Monospace)),
                        );

                        if content_response.changed() {
                            self.update_links(&current_id, &node.content)
                                .expect("Failed to update links");
                            self.graph.update(&self.db).expect("Failed to update graph");
                        }

                        // Rendered Markdown preview
                        ui.add_space(20.0);
                        ui.label(RichText::new("📄 Rendered Markdown").size(20.0).strong());
                        ui.add_space(10.0);
                        egui::ScrollArea::vertical()
                            .max_height(320.0)
                            .show(ui, |ui| {
                                CommonMarkViewer::new().show(
                                    ui,
                                    &mut self.markdown_cache,
                                    &node.content,
                                );
                            });

                        // Link section
                        ui.add_space(20.0);
                        ui.label(RichText::new("🔗 Links").size(20.0).strong());
                        ui.add_space(10.0);
                        ui.horizontal_wrapped(|ui| {
                            for link in &node.links {
                                if let Ok(Some(linked_node)) = self.db.get(link) {
                                    let linked_node: Node = serde_json::from_slice(&linked_node)
                                        .expect("Failed to deserialize linked node");
                                    if ui.button(&linked_node.title).clicked() {
                                        self.current_node = Some(link.clone());
                                    }
                                }
                            }
                        });
                        if ui
                            .button(RichText::new("🗑️ Delete Note").color(Color32::RED))
                            .clicked()
                        {
                            node_to_delete = Some(current_id.clone());
                        }
                    });

                    // Update node in the database
                    let updated_node = serde_json::to_vec(&node).expect("Failed to serialize node");
                    self.db
                        .insert(current_id.clone(), updated_node)
                        .expect("Failed to update node");
                }
            } else {
                ui.vertical_centered(|ui| {
                    ui.add_space(ui.available_height() / 3.0);
                    ui.label(
                        RichText::new("📜 Select a note or create a new one")
                            .size(24.0)
                            .color(Color32::LIGHT_GRAY),
                    );
                });
            }
        });

        egui::Window::new("Toolbar")
            .anchor(egui::Align2::LEFT_BOTTOM, egui::vec2(15.0, -15.0))
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("➕ New Note").clicked() {
                        let id = self.add_node("New Note".to_string(), String::new());
                        self.current_node = Some(id.expect("Failed to add new note"));
                    }
                    if ui.button("🔍 Toggle Graph View").clicked() {
                        self.show_graph = !self.show_graph;
                    }
                });
            });

        if self.show_graph {
            egui::SidePanel::right("graph_panel")
                .resizable(true)
                .default_width(320.0)
                .min_width(220.0)
                .max_width(420.0)
                .show(ctx, |ui| {
                    ui.heading("🌐 Graph View");
                    if let Err(e) = self.graph.show(ui) {
                        eprintln!("Failed to show graph: {:?}", e);
                    }
                });
        }

        // Handle note deletion
        if let Some(id) = node_to_delete {
            self.delete_node(&id).expect("Failed to delete note");
            self.current_node = None;
        }
    }
}
