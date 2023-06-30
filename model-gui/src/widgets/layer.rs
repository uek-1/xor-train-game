use egui::{Widget, Area, Rect, Sense, Pos2, Vec2};
use super::neuron::{neuron};

pub fn layer_ui(ui: &mut egui::Ui, layer : &mut rust_mlp::Layer<f64>, radius : f32) -> egui::Response {
    let layer_resp = ui.vertical( |ui| { 
        let neuron_size = Vec2::new(2.5 * radius, 4.0 * radius); // 4x radius for 1 neuron padding

        layer.weights
            .iter_mut()
            .enumerate()
            .for_each(
                |(num, weights)| {
                    ui.add_sized(neuron_size, neuron(radius, weights));
                }
            );
    }).response;



    ui.add_space(3.0);
    let layer_info = format!("Layer: Size: {}, Dimensions: {} x {}, Activation {:?}", layer.weights.len(), layer.weights[0].len(), layer.weights.len(), layer.activation);
    layer_resp.on_hover_text_at_pointer(layer_info)
}

pub fn layer(layer: &mut rust_mlp::Layer<f64>, radius: f32) -> impl Widget +'_{
    move |ui: &mut egui::Ui| layer_ui(ui, layer, radius)
}
