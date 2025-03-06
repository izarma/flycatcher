use std::time::{SystemTime, UNIX_EPOCH};
use eframe::App;
use egui::Id;
use super::chat_message::ChatMessage;

pub struct ChatApp {
    username: String,
    input_text: String,
    messeges: Vec<ChatMessage>,

    // UI State
    show_username_popup: bool,
}

impl ChatApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Add dummy msgs for testing
        let mut dummy_messages = Vec::new();
        dummy_messages.push(ChatMessage {
            username: "peepee".to_string(),
            content: "poopoo".to_string(),
            timestamp: "before".to_string(),
        });

        dummy_messages.push(ChatMessage {
            username: "poopoo".to_string(),
            content: "peepee".to_string(),
            timestamp: "now".to_string(),
        });

        dummy_messages.push(ChatMessage {
            username: "peepoo".to_string(),
            content: "poopee".to_string(),
            timestamp: "after".to_string(),
        });

        Self {
            username: String::new(),
            input_text: String::new(),
            messeges: dummy_messages,
            show_username_popup: true,
        }
    }
}

impl App for ChatApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // login popup
            if self.show_username_popup {
                egui::Window::new("LOGIN")
                    .id(Id::new("username popup"))
                    .collapsible(false)
                    .resizable(false)
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                    .show(ctx, |ui| {
                        ui.label("Choose your Username:");
                        let response = ui.text_edit_singleline(&mut self.username);
                        ui.horizontal(|ui| {
                            // Inside username popup window
                            if ui.button("Join Chat").clicked() || response.lost_focus() {
                                if !self.username.is_empty() {
                                    self.show_username_popup = false;
                                }
                            }
                        });
                    });
            }

            // username display
            ui.horizontal(|ui| {
                ui.label("Username:");
                let _ = ui.selectable_label(false, &self.username);
            });

            // Seperator
            ui.separator();

            // Chat History Area
            let available_height = ui.available_height() - 100.0;
            let available_width = ui.available_width();
            egui::ScrollArea::vertical()
                .max_height(available_height)
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    for msg in &self.messeges {
                        ui.set_min_width(available_width);
                        ui.horizontal(|ui| {
                            ui.strong(format!("{}:", msg.username));
                            ui.label(&msg.content);
                            ui.weak(&msg.timestamp);
                        });
                        ui.add_space(4.0); // Spacing between messages
                    }
                });
        });
        egui::TopBottomPanel::bottom("chat_input_panel").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.set_min_size(egui::vec2(400.0, 24.0));
                let response = ui.text_edit_singleline(&mut self.input_text);

                if ui.button("Send").clicked() || response.lost_focus() {
                    if !self.input_text.is_empty() {
                        let msg = ChatMessage {
                            username: self.username.clone(),
                            content: self.input_text.clone(),
                            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string(),
                        };
                        self.messeges.push(msg);
                        self.input_text.clear();
                    }
                }
            })
        });
    }
}
