use egui::{Pos2, Response, Shape, Stroke, Painter};

pub struct CircleOptions {
    pub angle: f32,
    pub offset: f32,
    pub closed: bool,
}

impl Default for CircleOptions {
    fn default() -> Self {
        Self {
            angle: 360.,
            offset: 0.,
            closed: true,
        }
    }
}

pub struct Circle {
    radius: f32,
    stroke: Stroke,
    angle: f32,
    offset: f32,
    closed: bool
}

impl Circle {
    pub fn new(radius: f32, stroke: Stroke, options: CircleOptions) -> Self {
        let CircleOptions {
            angle,
            offset,
            closed
        } = options;

        Self { radius, stroke, angle, offset, closed }
    }
}

impl Circle {
    pub fn paint(self, painter: &Painter, response: &Response) {
        let Self { radius, stroke, angle, offset, closed } = self;

        let points = create_points(radius - stroke.width - 1., response.rect.center(), angle, offset);

        painter.add(
            match closed {
                true => Shape::closed_line(points, stroke),
                false => Shape::line(points, stroke),
            }
        );
    }
}

fn create_points(radius: f32, center: Pos2, angle: f32, offset: f32) -> Vec<Pos2> {
    let points = 1800.;
    let single = points / 360.;

    let angle = if angle != 360. { angle % 360. } else { angle };
    let ratio = angle / 360.;

    let offset = offset.to_radians();

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