use egui::{Widget, Area, Rect, Sense, Pos2, Vec2, RichText, LayerId, Order};
use std::fmt::Write;
use super::neuron::{neuron};

pub fn layer_ui(ui: &mut egui::Ui, layer : &mut rust_mlp::Layer<f64>, radius : f32) -> egui::Response {
    let layer_info = format!("Layer: Neurons: {}, Dimensions: {} x {}, Activation {:?}", layer.weights.len(), layer.weights[0].len(), layer.weights.len(), layer.activation);
    let neuron_size = Vec2::new(2.0 * radius, 2.0 * radius); // 4x radius for 1 neuron padding
    let top_left = ui.min_rect().min + Vec2::new(-1.25 * radius, -4.0 * radius * layer.weights.len() as f32);
    let bottom_right = ui.min_rect().min + Vec2::new(1.25 * radius + 15.0, 2.5 * radius * layer.weights.len() as f32);
    let layer_rect = Rect::from_min_max(top_left, bottom_right);
    let return_resp = ui.interact(layer_rect, ui.next_auto_id() ,Sense::click());

    ui.vertical(|ui| { 
        layer.weights
            .iter_mut()
            .enumerate()
            .for_each(
                |(num, weights)| {
                    let response = ui.add_sized(neuron_size, neuron(radius, weights));
                    handle_neuron_response(ui, response.clone(), weights.to_vec());
                    ui.add_space(2.0 * radius);
                }
            );

    });

    ui.add_space(15.0);
     
    ui
        .interact(ui.min_rect(), ui.next_auto_id(), Sense::hover())
        .on_hover_ui_at_pointer(|ui| {
            ui.label(RichText::new(layer_info).size(16.0));
        });
    
    // Return click response to handle right click context menu in model widget
    return_resp
}

#[allow(unused)]
fn handle_neuron_response(ui: &mut egui::Ui, response: egui::Response, weights: Vec<f64>) {
    if response.clicked() {
        response.request_focus();
    };

    if response.has_focus() {
        let mut weights_string = String::from("Weights: ["); 
        for weight in weights {
            write!(weights_string, "{:.3}, ", weight);
        }
        write!(weights_string, "]");

        let area = Area::new(response.id)
            .order(egui::Order::Foreground)
            .constrain(true)
            .fixed_pos(response.rect.min)
            .interactable(true);

        area.show(&response.ctx, |ui| {
            let frame = egui::Frame::menu(ui.style()).show(ui, |ui| {
                const DEFAULT_MENU_WIDTH: f32 = 150.0;
                ui.set_max_width(DEFAULT_MENU_WIDTH);
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                    ui.label(RichText::new(weights_string).size(10.0));
                }).inner
            });
        });
    }

}

pub fn layer(layer: &mut rust_mlp::Layer<f64>, radius: f32) -> impl Widget +'_{
    move |ui: &mut egui::Ui| layer_ui(ui, layer, radius)
}
