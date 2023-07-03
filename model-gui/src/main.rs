use rust_mlp::{Model, Loss, Activation, Layer};
use instant::{Duration, Instant};
use egui::{vec2, Color32, Frame, Rect, RichText, Vec2, Sense, Stroke, Pos2, Painter};
use std::fmt::Write;
mod widgets;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("Hello, world!");
    let model : Model<f64> = Model::from_layers(
        vec![
            Layer::from_size(2, 2, Activation::Sigmoid)
        ],
        Loss::MeanSquaredError
    );

    print!("MODEL : {:?}", model);
}



#[cfg(target_arch = "wasm32")]
fn main(){
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "the_canvas_id",
            web_options,
            Box::new(|cc| Box::new(App::new(cc)))
        )
        .await
        .expect("Couldn't start eframe!");
    });
}

pub struct ModelData {
    pub learning_rate : String,
    pub layer_state : crate::widgets::model::LayerState,
}

impl Default for ModelData {
    fn default() -> Self {
        ModelData { 
            learning_rate:  String::from(""),
            layer_state : crate::widgets::model::LayerState::new()
        }
    }
}


pub struct App {
    time: Instant,
    model: Model<f64>,
    data: ModelData,
}

impl App {
    pub fn new(cc : &eframe::CreationContext<'_>)  -> Self {
        let visuals = egui::Visuals::dark();
        cc.egui_ctx.set_visuals(visuals);
        let mut debug_opt = egui::style::DebugOptions::default();
        debug_opt.debug_on_hover = true;
        debug_opt.show_interactive_widgets = true;
        let mut style = (*cc.egui_ctx.style()).clone();
        style.debug = debug_opt;
        cc.egui_ctx.set_style(style);

        App {
            time: Instant::now(),
            model: Model::new(Loss::MeanSquaredError),
            data: ModelData::default()
        } 
    }

    pub fn initialize_model(&mut self) {
        let model : Model<f64> = Model::from_layers(
            vec![
                Layer::from_size(2, 2, Activation::Sigmoid),
                Layer::from_size(2, 1, Activation::None)
            ],
            Loss::MeanSquaredError
        );

        self.model = model;
    }

    fn get_xor_data() -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
        let train = vec![
            vec![0.0, 0.0],
            vec![0.0, 1.0],
            vec![1.0, 0.0],
            vec![1.0, 1.0],
        ];

        let validate = vec![
            vec![0.0],
            vec![1.0],
            vec![1.0],
            vec![0.0]
        ];

        (train, validate)
    }

    pub fn train_model(&mut self, rate: f64) {
        let (train, validate) = Self::get_xor_data();
     
        self.model.fit(train, validate, 1, rate);
    }
    
    #[allow(unused)]
    pub fn get_model_string(&self) -> String {
        let mut out = String::from("");

        for (num, layer) in self.model.layers.iter().enumerate() {
            writeln!(out, "\nLAYER {} :", num); 

            writeln!(out, "WEIGHTS: ");
            for neuron in &layer.weights {
                for weight in neuron {
                    write!(out, "{:.4} ", weight);
                }
                writeln!(out, "");
            }
            writeln!(out, "ACTIVATION {:?}", layer.activation);
        }

        writeln!(out, "\n Model Loss function: {:?}", self.model.loss);

        out
    }

    #[allow(unused)]
    pub fn get_evaluate_string(&self) -> String {
        let mut out = String::from("");
        let (train, validate) = Self::get_xor_data();

        for epoch in 0..train.len() {
            writeln!(out, "Prediction: XOR ({:?}) = {:?}", train[epoch], self.model.evaluate(&train[epoch]));
        }

        out
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::SidePanel::right("Options Bar")
            .frame(
                Frame::default()
            )
            .show(
                ctx, |ui| {
                    ui.add(widgets::control_panel(self))
                }
            );
        egui::CentralPanel::default() 
            .frame(
                Frame::default()
            )
            .show(
                ctx, |ui| { 
                    egui::ScrollArea::vertical()
                        .vscroll(true)
                        .auto_shrink([false; 2])
                        .show(ui, |ui| {
                            ui.add(widgets::model(&mut self.model, &mut self.data.layer_state))
                    });
                }
            );

        // Done with CentralPanel        
        ctx.request_repaint_after(Duration::from_millis(50))
    }
}


