use egui::{Color32, Response, Ui, Visuals, Widget};
use crate::app::widget::ProgressCircle;
use crate::service::timer::{Timer, Status};

#[derive(Debug)]
pub struct CircularTimer<'a> {
    timer: &'a mut Timer
}

impl<'a> CircularTimer<'a> {
    pub fn new(timer: &'a mut Timer) -> Self {
        Self {
            timer,
        }
    }

    fn angle(&self) -> f32 {
        self.timer.progress() * 360.
    }
}

impl<'a> Widget for CircularTimer<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let Self { mut timer } = self;

        if timer.is_running() {
            timer.tick();

            ui.ctx().request_repaint();
        }

        let visuals = &ui.painter().ctx().style().visuals;

        ui.add(ProgressCircle::new(timer.progress() * 360., match timer.status {
            Status::Paused => visuals.error_fg_color,
            Status::Active => visuals.warn_fg_color,
            Status::Completed => visuals.hyperlink_color,
        }))
    }
}