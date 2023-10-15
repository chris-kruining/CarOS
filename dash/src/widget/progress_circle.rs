use egui::{Area, Pos2, Response, Sense, Shape, Stroke, Ui, Vec2, Widget};
use crate::widget::shape::{Circle, CircleOptions};

pub struct ProgressCircle {
    angle: f32,
}

impl ProgressCircle {
    pub fn new(angle: f32) -> Self {
        Self {
            angle,
        }
    }
}

impl Widget for ProgressCircle {
    fn ui(self, ui: &mut Ui) -> Response {
        let radius = 150.;
        let width = 10.;

        let (response, painter) = ui.allocate_painter(Vec2::splat(radius * 2.), Sense::hover());
        
        // paint a circle
        let center = response.rect.center();
        let visuals = &painter.ctx().style().visuals;

        let opts = CircleOptions {
            angle: self.angle,
            closed: false,
            ..Default::default()
        };

        Circle::new(radius, Stroke::new(width, visuals.extreme_bg_color), Default::default()).paint(&painter, &response);
        Circle::new(radius, Stroke::new(width, visuals.error_fg_color), opts).paint(&painter, &response);
        
        response
    }
}