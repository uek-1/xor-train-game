use egui::Area;

use crate::{Pos2, Color32, Vec2, Sense, Stroke};

pub fn neuron_ui(ui: &mut egui::Ui, pos : Pos2, radius : f32, weights : &mut Vec<f64> ) -> egui::Response { 
    Area::new(ui.next_auto_id())
        .fixed_pos(pos)
        .show(
            ui.ctx(), |ui| {
                let (response, painter) = ui.allocate_painter(Vec2::new(2.0 * radius, 2.0 * radius), Sense::click());
                let screen = response.rect;

                if ui.is_rect_visible(screen) {
                    painter.circle_stroke(screen.center(), radius, Stroke::new(1.0, Color32::from_rgb(255, 255, 255)));
                }
   
                
                if response.hovered() {
                    ui.heading(format!("{:?}", weights));
                    //painter.text(screen.center(), egui::Align2::CENTER_CENTER, "Hovered", egui::FontId::default(), Color32::from_rgb(255, 255, 255));
                }
            }
        ).response
}

pub fn neuron(pos: Pos2, radius: f32, weights : &mut Vec<f64>) -> impl egui::Widget + '_ {
    move |ui : &mut egui::Ui| neuron_ui(ui, pos, radius, weights) 
}
