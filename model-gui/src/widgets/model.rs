use egui::{Area, Vec2, ScrollArea, widgets::plot::{Plot, PlotUi, Text, MarkerShape, Points, self}, Rect, Sense, Layout};
use rust_mlp::{Model, Activation, self};

use super::layer::layer;

pub struct LayerState {
    neuron_count_string : String,
    activation : Activation
}

impl LayerState {
    pub fn new() -> Self {
        LayerState { neuron_count_string: String::from(""), activation: Activation::None }
    }
}


pub fn model_ui(ui: &mut egui::Ui, model: &mut Model<f64>, layer_state : &mut LayerState) -> egui::Response {
    let radius = 30.0;
    let mut layer_to_delete = None;
    let mut layer_to_add = None;
    let last_layer_num = model.layers.len().checked_sub(1);
    let return_response = ui.horizontal_centered( 
        |ui| {
            let mut current_pos = ui.next_widget_position();

            model.layers
                .iter_mut()
                .enumerate()
                .for_each(
                    |(num, model_layer)| {
                        let layer_neurons = model_layer.weights.len() as f32;
                        let layer_size = Vec2::new(radius * 2.5, radius * 4.0 * layer_neurons);
                        let layer_rect = Rect::from_min_size(current_pos, layer_size);
                        ui.add_sized(layer_size, layer(model_layer, radius))
                            .context_menu(|ui| {
                                ui.menu_button("New", |ui| {
                                    ui.text_edit_singleline(&mut layer_state.neuron_count_string).on_hover_text("Enter the number of neurons");
                                    egui::ComboBox::from_label("Activation Function")
                                        .selected_text(format!("{:?}", layer_state.activation))
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(&mut layer_state.activation, Activation::Sigmoid, "Sigmoid");
                                            ui.selectable_value(&mut layer_state.activation, Activation::None, "None");
                                        });

                                    if ui.button("Add").clicked() {
                                        let input_shape = layer_neurons as usize;
                                        let output_shape = layer_state.neuron_count_string.clone().parse().unwrap();
                                        layer_to_add = Some((num, rust_mlp::Layer::from_size(input_shape, output_shape, layer_state.activation.clone())));                                        
                                        ui.close_menu();
                                    };
                                });

                                // Safety : Can unwrap safely because layers > 1 in this code block
                                // so last_layer_num must exist.

                                if num != last_layer_num.unwrap() && ui.button("Delete").clicked() {
                                    layer_to_delete = Some(num);
                                    ui.close_menu();
                                }
                            });
                    }
                );
        }
    ).response;

    match (layer_to_delete, last_layer_num) {
        (Some(num), Some(last)) if num != last => {model.layers.remove(num); ()},
        _ => ()
    } 

    match layer_to_add {
        Some((num, x)) => model.layers.insert(num + 1, x),
        _ => ()
    }
     
    return_response
}

pub fn model<'a>(model: &'a mut Model<f64>, layer_state: &'a mut LayerState) -> impl egui::Widget + 'a {
    move |ui : &mut egui::Ui| model_ui(ui, model, layer_state) 
}


