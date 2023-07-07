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

pub struct LayerMenuState {
    index : usize,
    input_shape: usize,
    last_layer_index: Option<usize>,
    layer_to_delete: Option<usize>,
    layer_to_add: Option<(usize, rust_mlp::Layer<f64>)>
}

pub fn model<'a>(model: &'a mut Model<f64>, layer_state: &'a mut LayerState) -> impl egui::Widget + 'a {
    move |ui : &mut egui::Ui| model_ui(ui, model, layer_state) 
}

pub fn model_ui(ui: &mut egui::Ui, model: &mut Model<f64>, layer_state : &mut LayerState) -> egui::Response {
    let radius = 30.0;
    let mut layer_menu_state = LayerMenuState {
        index: 0,
        input_shape: 0,
        last_layer_index : model.layers.len().checked_sub(1),
        layer_to_delete: None,
        layer_to_add: None
    };


    let return_response = ui.horizontal_centered(|ui| create_layers_ui(ui, model, radius, layer_state, &mut layer_menu_state)).response;

    match (layer_menu_state.layer_to_delete, layer_menu_state.last_layer_index) {
        (Some(num), Some(last)) if num != last => {model.layers.remove(num); ()},
        _ => ()
    } 

    match layer_menu_state.layer_to_add {
        Some((num, x)) => model.layers.insert(num + 1, x),
        _ => ()
    }
     
    return_response
}

fn create_layers_ui(ui: &mut egui::Ui, model: &mut Model<f64>, radius: f32, layer_state: &mut LayerState, layer_menu_state: &mut LayerMenuState) {
    model.layers
        .iter_mut()
        .enumerate()
        .for_each(|(num, model_layer)| {
            let layer_neurons = model_layer.weights.len() as f32;
            let layer_size = Vec2::new(radius * 2.5, radius * 4.0 * layer_neurons);
            layer_menu_state.input_shape = layer_neurons as usize;
            layer_menu_state.index = num;
            ui.add_sized(layer_size, layer(model_layer, radius))
                .context_menu(|ui| layer_context_menu(ui, layer_state, layer_menu_state));
        });
}

fn layer_context_menu(ui : &mut egui::Ui, layer_state : &mut LayerState, layer_menu_state: &mut LayerMenuState) {
    ui.label("Layer Actions:");
    ui.menu_button("New", |ui| {
        egui::TextEdit::singleline(&mut layer_state.neuron_count_string)
            .hint_text("Layer Size")
            .show(ui);

        //egui::ComboBox::from_label("Activation Function")
            //.selected_text(format!("{:?}", layer_state.activation))
        ui.menu_button("Activation", |ui| { 
            ui.selectable_value(&mut layer_state.activation, Activation::None, "None");
            ui.selectable_value(&mut layer_state.activation, Activation::Sigmoid, "Sigmoid");
            ui.selectable_value(&mut layer_state.activation, Activation::Relu, "ReLu");
            ui.selectable_value(&mut layer_state.activation, Activation::Softmax, "Softmax");
        });


        if ui.button("Add").clicked() {
            let input_shape = layer_menu_state.input_shape;
            let output_shape = layer_state.neuron_count_string.clone().parse().unwrap();
            layer_menu_state.layer_to_add = Some((layer_menu_state.index, rust_mlp::Layer::from_size(input_shape, output_shape, layer_state.activation.clone()))); 
            ui.close_menu();
        };
    });

    // Safety : Can unwrap safely because layers > 1 in this code block
    // so last_layer_num must exist.

    if layer_menu_state.index != layer_menu_state.last_layer_index.unwrap() && ui.button("Delete").clicked() {
        layer_menu_state.layer_to_delete = Some(layer_menu_state.index);
        ui.close_menu();
    }
}




