use chrono::{DateTime, Duration, Utc};

struct Active;

struct Paused {
    at: DateTime<Utc>,
}

impl Paused {
    fn new() -> Self {
        Self { at: Utc::now() }
    }
}

struct Completed;


pub struct Timer<S> {
    duration: Duration,
    state: S,
}

impl Timer<Active> {
    fn pause(mut self) -> Timer<Paused> {
        self.into()
    }
}

impl Timer<Paused> {
    fn new(duration: Duration) -> Self {
        Self {
            duration,
            state: Paused::new(),
        }
    }

    fn resume(mut self) -> Timer<Active> {
        self.into()
    }
}

impl From<Timer<Active>> for Timer<Paused> {
    fn from(value: Timer<Active>) -> Self {
        Self {
            duration: value.duration,
            state: Paused::new(),
        }
    }
}

impl From<Timer<Paused>> for Timer<Active> {
    fn from(value: Timer<Paused>) -> Self {
        Self {
            duration: value.duration,
            state: Active,
        }
    }
}

fn kaas() {
    let timer = Timer::new(Duration::seconds(5));
    let timer = timer.resume();
}