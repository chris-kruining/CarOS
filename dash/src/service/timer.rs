use chrono::{DateTime, Duration, Utc};

#[derive(PartialEq, Eq, Debug)]
pub enum Status {
    Active,
    Paused,
    Completed,
}

#[derive(Debug)]
pub struct Timer {
    pub duration: Duration,
    pub finish_at: DateTime<Utc>,
    pub paused_at: DateTime<Utc>,
    pub status: Status,
    paused: bool
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        let now = Utc::now();

        Self {
            duration,
            finish_at: now + duration,
            paused_at: now,
            status: Status::Paused,
            paused: true,
        }
    }

    pub fn tick(&mut self) {
        if self.status != Status::Active {
            return;
        }

        if Utc::now() > self.finish_at {
            self.paused_at = Utc::now();
            self.status = Status::Completed;
        }
    }

    pub fn resume(&mut self) {
        if self.paused == false {
            return;
        }

        self.paused = false;
        self.finish_at = Utc::now() + (self.finish_at - self.paused_at);
        self.status = Status::Active;
    }

    pub fn pause(&mut self) {
        if self.paused == true {
            return;
        }

        self.paused = true;
        self.paused_at = Utc::now();
        self.status = Status::Paused;
    }

    pub fn reset(&mut self) {
        let now = Utc::now();

        self.paused_at = now;
        self.finish_at = now + self.duration;
        self.status = match self.paused {
            true => Status::Paused,
            false => Status::Active,
        };
    }

    pub fn is_running(&self) -> bool {
        self.status == Status::Active
    }

    pub fn progress(&self) -> f32 {
        match self.status {
            Status::Completed => 1.,
            Status::Active => self.calc_percentage(Utc::now()),
            Status::Paused => self.calc_percentage(self.paused_at),
        }
    }

    fn calc_percentage(&self, goal: DateTime<Utc>) -> f32 {
        let difference = self.finish_at - goal;
        let percentage = difference.num_milliseconds() as f32 / self.duration.num_milliseconds() as f32;

        1. - percentage
    }
}