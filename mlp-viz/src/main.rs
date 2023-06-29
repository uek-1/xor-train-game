use rust_mlp::{Model, Loss, Activation, Layer};
use instant::{Duration, Instant};
use egui::{vec2, Color32, Frame, Rect, RichText, Vec2, Sense, Stroke};
use std::fmt::Write;

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


pub struct App {
    time: Instant,
    model: Model<f64>,
}

impl App {
    pub fn new(cc : &eframe::CreationContext<'_>)  -> Self {
        let visuals = egui::Visuals::dark();
        cc.egui_ctx.set_visuals(visuals);

        App {
            time: Instant::now(),
            model: Model::new(Loss::MeanSquaredError)
        } 
    }

    pub fn initialize_model(&mut self) {
        let model : Model<f64> = Model::from_layers(
            vec![
                Layer::from_size(2, 8, Activation::Sigmoid),
                Layer::from_size(8, 16, Activation::Sigmoid),
                Layer::from_size(16, 1, Activation::None)
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

    pub fn train_model(&mut self) {
        let (train, validate) = Self::get_xor_data();
     
        self.model.fit(train, validate, 1, 1.2);
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

    pub fn paint_model(&mut self, ui: &mut egui::Ui) {
        let white_col32 = Color32::from_rgb(255, 255, 255);
        let black_col32 = Color32::from_rgb(0, 0, 0);
        let white_stroke = Stroke::new(3.0, white_col32);
        let black_stroke = Stroke::new(3.0, black_col32);

        ui.heading(RichText::new("MODEL:").size(30.0));

        let (response, painter) = ui.allocate_painter(ui.available_size() * 0.9, Sense::hover());

        let screen = response.rect;
        let origin = screen.min;
        let circle_radius = 30.0;
        
        // 1 neuron width between layers
        let x_step = circle_radius * 4.0;
        // 0.5 neuron height between neurons
        let y_step = circle_radius * 3.0;

        for (layer_num, layer) in self.model.layers.iter().enumerate() {
            let neuron_count = layer.weights.len() as f32;
            let centered_height = (screen.height() / 2.0) - ((neuron_count - 1.0) * circle_radius);
            let mut current_pos = origin + Vec2::new(circle_radius + x_step * (layer_num as f32), centered_height);

            for neuron in &layer.weights {
                painter.circle_stroke(current_pos, circle_radius, white_stroke);
                current_pos += Vec2::new(0.0, y_step);
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default() 
            .frame(
                Frame::default()
            )
            .show(
                ctx, |ui| {
                    if ui.button(RichText::new("CREATE MODEL!").size(24.0)).clicked() {
                        self.initialize_model(); 
                    }

                    ui.collapsing(
                        RichText::new("DEBUG").size(16.0), |ui| {
                            ui.label(RichText::new(self.get_model_string()).size(12.0));
                        }
                    );

                    ui.add_space(5.0);

                    if ui.button(RichText::new("TRAIN MODEL!").size(24.0)).clicked() {
                        self.train_model();
                    }

                    ui.collapsing(
                        RichText::new("Predictions").size(20.0), |ui| { 
                            ui.label(RichText::new(self.get_evaluate_string()).size(12.0));
                        }
                    );

                    ui.add_space(5.0);
                    
                    egui::ScrollArea::both().show(ui, |ui| self.paint_model(ui)); 
                }
            );

        // Done with CentralPanel
        
        ctx.request_repaint_after(Duration::from_millis(50))
    }
}






