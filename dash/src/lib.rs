#![warn(clippy::all, rust_2018_idioms)]

pub mod widget;

mod app;
mod headlines;

pub use app::App;
pub use headlines::Headlines;