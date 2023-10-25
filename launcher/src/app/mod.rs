use eframe::epaint::FontFamily;
use egui::{FontData, FontDefinitions};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    label: String,
}

impl Default for App {
    fn default() -> Self {
        Self { label: "My label".to_string() }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        configure_font(&cc.egui_ctx);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
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

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { label } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(label.to_owned());
        });
    }
}