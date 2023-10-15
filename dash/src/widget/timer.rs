use chrono::{Utc, Duration, DateTime};
use egui::{Response, Ui, Widget};
use crate::widget::ProgressCircle;

#[derive(PartialEq, Eq, Debug)]
enum Status {
    Active,
    Paused,
    Completed,
}

#[derive(Debug)]
pub struct Timer {
    duration: Duration,
    finish_at: DateTime<Utc>,
    paused_at: DateTime<Utc>,
    status: Status,
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        let now = Utc::now();

        Self {
            duration,
            finish_at: now + duration,
            paused_at: now,
            status: Status::Paused,
        }
    }

    fn tick(&mut self) {
        if self.status == Status::Paused {
            return;
        }

        if Utc::now() > self.finish_at {
            self.status = Status::Completed;
        }


    }

    pub fn resume(&mut self) {
        if self.status != Status::Paused {
            return;
        }

        self.finish_at = Utc::now() + (self.finish_at - self.paused_at);
        self.status = Status::Active;
    }

    pub fn pause(&mut self) {
        if self.status != Status::Active {
            return;
        }

        self.paused_at = Utc::now();
        self.status = Status::Paused;
    }

    fn is_running(&self) -> bool {
        self.status == Status::Active
    }

    fn angle(&self) -> f32 {
        match self.status {
            Status::Completed => 360.,
            _ => {
                let difference = self.finish_at - self.paused_at;
                let percentage = difference.num_milliseconds() as f32 / self.duration.num_milliseconds() as f32;

                dbg!(self);
                dbg!(difference.num_milliseconds());
                dbg!(self.duration.num_milliseconds());
                dbg!(percentage);

                (1. - percentage) * 360.
            },
        }
    }
}

impl Widget for Timer {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let circle = ProgressCircle::new(self.angle());

        if self.is_running() {
            self.tick();

            ui.ctx().request_repaint();
        }

        ui.add(circle)
    }
}