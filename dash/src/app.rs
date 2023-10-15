use chrono::Duration;
use eframe::epaint::FontFamily;
use egui::{ScrollArea, FontDefinitions, FontData, Separator, Ui, TopBottomPanel, Layout, Label, Align, RichText, Button};
use crate::widget::{ProgressCircle, Timer};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,

    #[serde(skip)]
    headlines: crate::Headlines,
}

impl Default for App {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Kaas is awesome!".to_owned(),
            value: 2.7,
            headlines: crate::Headlines::new(),
        }
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

const PADDING: f32 = 10.0;
impl eframe::App for App {
    fn persist_native_window(&self) -> bool { false }
    fn persist_egui_memory(&self) -> bool { false }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { label, value, headlines } = self;
        let mut timer = Timer::new(Duration::seconds(5));

        timer.resume();

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("My Side Panel");

            ui.add(timer);

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(label);
            });

            ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                *value += 1.0;
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    egui::warn_if_debug_build(ui);
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });

        menu(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            header(ui);

            self.headlines.render(ui, |a, ui| {
                ui.heading(&a.title);
                ui.hyperlink_to(&a.description, &a.url);
            });

            footer(ctx);
        });

        // egui::Window::new("Window").show(ctx, |ui| {
        //     ui.label("Windows can be moved by dragging them.");
        //     ui.label("They are automatically sized based on contents.");
        //     ui.label("You can turn on resizing and scrolling if you like.");
        //     ui.label("You would normally choose either panels OR windows.");
        // });
    }
}

fn menu(ctx: &egui::Context) {
    TopBottomPanel::top("menu").show(ctx, |ui| {
        ui.add_space(10.);

        egui::menu::bar(ui, |ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                ui.label(RichText::new("📚").text_style(egui::TextStyle::Heading));
            });

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let _close_btn = ui.button(RichText::new("❌").text_style(egui::TextStyle::Body));
                let _refresh_btn = ui.button(RichText::new("🔃").text_style(egui::TextStyle::Body));
                let _theme_btn = ui.button(RichText::new("🌙").text_style(egui::TextStyle::Body));
            });
        });

        ui.add_space(10.);
    });
}

fn header(ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("Headlines");
    });
    ui.add(Separator::default().spacing(PADDING));
}

fn footer(ctx: &egui::Context) {
    TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(PADDING);
            ui.heading("Footer");
            ui.add_space(PADDING);
        });
    });
}