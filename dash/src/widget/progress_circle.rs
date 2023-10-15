use egui::{Area, Pos2, Response, Sense, Shape, Stroke, Ui, Vec2, Widget};
use crate::widget::shape::{Circle, CircleOptions};

fn create_circle_vertices(radius: f32, center: Pos2, angle: Option<f32>, offset: Option<f32>) -> Vec<Pos2> {
    let points = 1800.;
    let single = points / 360.;

    let angle = angle.unwrap_or(360.);
    let angle = if angle != 360. { angle % 360. } else { angle };
    let ratio = angle / 360.;

    let offset = offset.unwrap_or(0.).to_radians();

    (0..(points * ratio) as u32)
        .map(|p | p as f32 / single)
        .map(|p| {
            let (sin, cos) = (p.to_radians() + offset).sin_cos();

            let x = center.x + radius * sin;
            let y = center.y + radius * -cos;

            Pos2 { x, y }
        })
        .collect()
}

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

        let visuals = &ui.ctx().style().visuals;

        Area::new("progress").fixed_pos(Pos2::new(0., 0.)).show(&ui.ctx(), |ui| {
            // bg
            ui.add(Circle::new(radius, Stroke::new(width, visuals.extreme_bg_color), Default::default()));

            // fg
            ui.add(Circle::new(
                radius,
                Stroke::new(width, visuals.error_fg_color),
                CircleOptions {
                    angle: self.angle,
                    closed: false,
                    ..Default::default()
                }
            ));
        }).response

        // let (response, painter) = ui.allocate_painter(Vec2::splat(radius * 2.), Sense::hover());
        //
        // // paint a circle
        // {
        //     let center = response.rect.center();
        //     let visuals = &painter.ctx().style().visuals;
        //
        //     // paint bg
        //     let stroke = Stroke::new(width, visuals.extreme_bg_color);
        //     let bg = create_circle_vertices(radius - stroke.width - 1., center, None, None);
        //     let bg = Shape::closed_line(bg, stroke);
        //     painter.add(bg);
        //
        //     // paint fg
        //     let stroke = Stroke::new(width, visuals.error_fg_color);
        //     let fg = create_circle_vertices(radius - stroke.width - 1., center, Some(self.angle), None);
        //     let fg = Shape::line(fg, stroke);
        //     painter.add(fg);
        // }
        //
        // response
    }
}