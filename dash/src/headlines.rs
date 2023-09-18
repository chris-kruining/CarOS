use egui::{Response, ScrollArea, Ui, Widget, Label, Pos2, Sense, WidgetText, Separator};
use egui::widget_text::WidgetTextGalley;

pub struct Headlines {
    pub articles: Vec<NewsCardData>
}

impl Headlines {
    pub fn new() -> Self {
        Self {
            articles: (0..20)
                .map(|i| NewsCardData {
                    title: format!("Title {}", i),
                    description: format!("Description {}", i),
                    url: format!("https://example.com/slug-for-article-{}", i),
                })
                .collect::<Vec<_>>()
        }
    }

    pub fn render(&self, ui: &mut Ui, render_item: impl Fn(&NewsCardData, &mut Ui)) {
        ScrollArea::both().auto_shrink([ false, true ]).show(ui, |ui| {
            for a in &self.articles {
                render_item(a, ui);
                ui.add(Separator::default().spacing(20.));
            }
        });
    }
}

pub struct NewsCardData {
    pub title: String,
    pub description: String,
    pub url: String,
}

