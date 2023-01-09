pub mod app;
pub mod backend;
pub mod layer;
pub mod events;
pub mod io;

pub use spdlog::prelude::*;

pub fn init() {
    info!("Initializing Geg");
}
