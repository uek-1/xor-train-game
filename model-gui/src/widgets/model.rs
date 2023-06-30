use egui::{Area, Vec2, ScrollArea, widgets::plot::{Plot, PlotUi, Text, MarkerShape, Points, self}};
use rust_mlp::Model;

use super::neuron::{neuron, self};

pub fn model_ui(ui: &mut egui::Ui, model: &mut Model<f64>) -> egui::Response {
    let max_size = ui.available_size();

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
    
    /*
    egui::ScrollArea::vertical().show_viewport(ui, |ui, viewport| {
    ui.rect(
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
    )
    }).inner.response
    */
}

pub fn model(model: &mut Model<f64>) -> impl egui::Widget + '_ {
    move |ui : &mut egui::Ui| model_ui(ui, model) 
}
