use egui::{Area, Vec2, ScrollArea};
use rust_mlp::Model;

use super::neuron::neuron;

pub fn model_ui(ui: &mut egui::Ui, model: &mut Model<f64>) -> egui::Response {
    let max_size = ui.available_size();

    ui.allocate_ui(
        max_size, |ui| {
            let origin = ui.next_widget_position();
            let radius = 20.0;    

            for (i, layer) in model.layers.iter_mut().enumerate(){
                let mut loc = origin + Vec2::new(i as f32 * radius * 5.0, 0.0);

                for weights in &mut layer.weights { 
                    ui.add(neuron(loc, 20.0, weights));
                    loc += Vec2::new(0.0, radius * 4.0);
                    ui.skip_ahead_auto_ids(1);
                }
            }
        }
    ).response
}

pub fn model(model: &mut Model<f64>) -> impl egui::Widget + '_ {
    move |ui : &mut egui::Ui| model_ui(ui, model) 
}
