use crate::App;
use egui::{RichText, Response, SidePanel};

fn control_panel_ui(ui: &mut egui::Ui, app: &mut App) -> Response {
    ui.collapsing(
        "Control Panel", |ui| {
            if ui.button(RichText::new("CREATE MODEL").size(24.0)).clicked() {
                app.initialize_model(); 
            }
            
            ui.add_space(2.0);

            if ui.button(RichText::new("TRAIN MODEL!").size(24.0)).clicked() {
                if let Ok(x) = app.data.learning_rate.parse() {
                    app.train_model(x);
                } 
            }

            ui.add(egui::TextEdit::singleline(&mut app.data.learning_rate).hint_text("Learning Rate"));
            
            ui.add_space(2.0);
            
            ui.collapsing(
                RichText::new("Predictions").size(16.0), |ui| { 
                    ui.label(RichText::new(app.get_evaluate_string()).size(12.0));
                }
            );

            ui.collapsing(
                RichText::new("Debug").size(16.0), |ui| {
                    ui.label(RichText::new(app.get_model_string()).size(12.0));
                }
            );
        }
    )
    .header_response
}

pub fn control_panel(app : &mut App) -> impl egui::Widget + '_{
   move |ui : &mut egui::Ui| control_panel_ui(ui, app) 
}
