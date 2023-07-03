use egui::{Widget, Area, Rect, Sense, Pos2, Vec2, RichText};
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


    ui.add_space(15.0);
    let layer_info = format!("Layer: Neurons: {}, Dimensions: {} x {}, Activation {:?}", layer.weights.len(), layer.weights[0].len(), layer.weights.len(), layer.activation);
    ui
        .interact(ui.min_rect(), ui.next_auto_id(), Sense::hover())
        .on_hover_ui_at_pointer(|ui| {
            ui.label(RichText::new(layer_info).size(16.0));
        });
    
    // Return click response to handle right click context menu in model widget
    ui.interact(ui.min_rect(), ui.next_auto_id(), Sense::click())



    //layer_resp.on_hover_text_at_pointer(layer_info)
}

pub fn layer(layer: &mut rust_mlp::Layer<f64>, radius: f32) -> impl Widget +'_{
    move |ui: &mut egui::Ui| layer_ui(ui, layer, radius)
}
