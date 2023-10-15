use egui::{Color32, Response, Sense, Stroke, Ui, Vec2, Widget};
use crate::app::widget::shape::{Circle, CircleOptions};

pub struct ProgressCircle {
    angle: f32,
    color: Color32,
}

impl ProgressCircle {
    pub fn new(angle: f32, color: impl Into<Color32>) -> Self {
        Self {
            angle,
            color: color.into(),
        }
    }
}

impl Widget for ProgressCircle {
    fn ui(self, ui: &mut Ui) -> Response {
        let Self { angle, color } = self;

        let radius = 150.;
        let width = 10.;

        let (response, painter) = ui.allocate_painter(Vec2::splat(radius * 2.), Sense::hover());
        
        // paint a circle
        let visuals = &painter.ctx().style().visuals;

        let circle_options = CircleOptions {
            angle,
            closed: false,
            ..Default::default()
        };

        Circle::new(radius, Stroke::new(width, visuals.extreme_bg_color), Default::default()).paint(&painter, &response);
        Circle::new(radius, Stroke::new(width, color), circle_options).paint(&painter, &response);
        
        response
    }
}