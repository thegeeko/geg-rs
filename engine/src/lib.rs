pub mod app;
pub mod backend;
pub mod layer;
pub mod events;
pub mod io;

pub use spdlog::prelude::*;

pub fn init() {
    spdlog::init_env_level().expect("Failed to initialize logging");
    info!("Initializing Geg");
}
