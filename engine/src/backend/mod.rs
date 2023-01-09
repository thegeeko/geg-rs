use std::sync::Arc;
use winit::window::Window;

use self::vulkan::device::GegVkDevice;

mod vulkan;

#[derive(Debug)]
pub enum GegBackend {
  Vulkano,
  Wgpu,
}

pub struct GraphicsContext {
  device: GegVkDevice,
  backend: GegBackend
}

impl GraphicsContext {
  pub fn new(backend: GegBackend, win: Arc<Window>) -> Self {
    Self {
      device: GegVkDevice::new(win),
      backend
    }
  }
}
