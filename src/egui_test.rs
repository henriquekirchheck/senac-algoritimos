use clap::Parser;
use eframe::egui;
use egui_extras::install_image_loaders;
use reqwest::blocking::Client;
use rustemon::model::pokemon::Pokemon;

#[derive(Debug, Parser)]
struct Args {
    pokemon: String,
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    let args = Args::parse();
    let _ = eframe::run_native(
        "Hello World!",
        native_options,
        Box::new(move |cc| Box::new(MyEguiApp::new(cc, args))),
    )
    .unwrap();
}

#[derive(Debug)]
struct MyEguiApp {
    pokemon: Pokemon,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>, args: Args) -> Self {
        let client = rustemon::client::RustemonClient::default();
        let pokemon = get_pokemon(args, &client);
        install_image_loaders(&cc.egui_ctx);
        Self { pokemon }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(self.pokemon.name.to_owned());
            let binding = self.pokemon.sprites.front_default.clone().unwrap();
            let image = egui::Image::new(&binding);
            ui.add(image)
        });
    }
}

#[tokio::main]
async fn get_pokemon(args: Args, client: &rustemon::client::RustemonClient) -> Pokemon {
    rustemon::pokemon::pokemon::get_by_name(&args.pokemon, client)
        .await
        .unwrap()
}
