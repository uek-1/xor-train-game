use egui::Area;
use std::fmt::Write;

use crate::{Pos2, Color32, Vec2, Sense, Stroke};

pub fn neuron_ui(ui: &mut egui::Ui, radius : f32, weights : &mut Vec<f64> ) -> egui::Response { 
    let (response, painter) = ui.allocate_painter(Vec2::new(2.0 * radius, 2.0 * radius), Sense::click());
    let screen = response.rect;
    //ui.label(format!("NEURON {:?}", screen));

    //if ui.is_rect_visible(screen) {
        painter.circle_stroke(screen.center(), radius, Stroke::new(1.0, Color32::from_rgb(255, 255, 255)));
    //}
    
    if response.clicked() {
        response.request_focus();
    };

    if response.has_focus() {
        let mut weights_string = String::from("Weights: ["); 
        for weight in weights {
            write!(weights_string, "{:.3}, ", weight);
        }
        write!(weights_string, "]");
        ui.label(egui::RichText::new(weights_string).size(12.0));
    }

    response
}

pub fn neuron(radius: f32, weights : &mut Vec<f64>) -> impl egui::Widget + '_ {
    move |ui : &mut egui::Ui| neuron_ui(ui, radius, weights) 
}
