use std::default;
use eframe::epaint::FontFamily;
use egui::{FontDefinitions, FontData};
// use walkers::{Tiles, Map, MapMemory, Position, providers::openstreetmap};
use default::Default;

pub struct App {
    label: String,
    // tiles: Tiles,
    // map_memory: MapMemory,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        configure_font(&cc.egui_ctx);

        Self {
            label: "Navigation".to_string(),
            // tiles: Tiles::new(openstreetmap, cc.egui_ctx.to_owned()),
            // map_memory: MapMemory::default(),
        }
    }
}

fn configure_font(ctx: &egui::Context) {
    let font_name = "MesloLGS";
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(font_name.to_owned(), FontData::from_static(include_bytes!("../../MesloLGS_NF_Regular.ttf")));

    if let Some(family) = fonts.families.get_mut(&FontFamily::Proportional) {
        family.insert(0, font_name.to_owned());
    }

    if let Some(family) = fonts.families.get_mut(&FontFamily::Monospace) {
        family.insert(0, font_name.to_owned());
    }

    ctx.set_fonts(fonts);
}

impl eframe::App for App {
    fn persist_native_window(&self) -> bool { false }
    fn persist_egui_memory(&self) -> bool { false }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let rimless = egui::Frame {
            fill: ctx.style().visuals.panel_fill,
            ..Default::default()
        };

        egui::CentralPanel::default()
            .frame(rimless)
            .show(ctx, |ui| {
                ui.label(self.label.to_owned());
                // ui.add(Map::new(
                //     Some(&mut self.tiles),
                //     &mut self.map_memory,
                //     Position::new(17.03664, 51.09916)
                // ));
            });
    }
}