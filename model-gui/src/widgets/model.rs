use egui::{Area, Vec2, ScrollArea, widgets::plot::{Plot, PlotUi, Text, MarkerShape, Points, self}, Rect, Sense, Layout};
use rust_mlp::Model;

use super::layer::layer;

pub fn model_ui(ui: &mut egui::Ui, model: &mut Model<f64>) -> egui::Response {
    let radius = 30.0;
    ui.horizontal_centered( 
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
                        ui.add_sized(layer_size, layer(model_layer, radius));
                    }
                );
        }
    ).response
}

pub fn model_ui_plot(ui : &mut egui::Ui, model : &mut Model<f64>) -> egui::Response {
    Plot::new("model-plot")
        .show_x(false)
        .show_y(false)
        .show_axes([false; 2])
        .center_x_axis(true)
        .center_y_axis(true)
        .allow_scroll(false)
        .allow_drag(false)
        .data_aspect(2.0)
        .view_aspect(2.0)
        .auto_bounds_y()
        .auto_bounds_x()
        .show_background(false)
        .height(ui.available_height() * 0.8)
        .show(ui, |plotui| {
            let radius = 20.0;    
            let mut points = vec![];
            let layer_count = model.layers.len() as f32;
            let origin = Vec2::new(-5.0 * radius * (layer_count - 1.0), 0.0);
            
            for (i, layer) in model.layers.iter_mut().enumerate(){
                let neuron_count = layer.weights.len() as f32;
                let mut loc = origin + Vec2::new(i as f32 * radius * 10.0, -2.0 * radius * (neuron_count-1.0));

                for weights in &mut layer.weights { 
                    //ui.add(neuron(loc, 20.0, weights));
                    points.push([loc.x as f64, loc.y as f64]);
                    loc += Vec2::new(0.0, radius * 4.0);
                }
            }

            plotui.points(
                Points::new(points)
                    .radius(radius)
                    .shape(MarkerShape::Circle)
                    .color(egui::Color32::from_rgb(255, 255, 255))
                    .filled(false)
            )
    }).response

}

pub fn model(model: &mut Model<f64>) -> impl egui::Widget + '_ {
    move |ui : &mut egui::Ui| model_ui(ui, model) 
}
