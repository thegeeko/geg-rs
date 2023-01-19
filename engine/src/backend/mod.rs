use std::sync::Arc;
use winit::window::Window;

use self::vulkan::{device::GegVkDevice, renderer::GegVkRenderer};

mod vulkan;

#[derive(Debug)]
pub enum GegBackend {
  Vulkano,
  Wgpu,
}

pub struct GraphicsContext {
  device: GegVkDevice,
  backend_type: GegBackend,
  renderer: GegVkRenderer,
}

impl GraphicsContext {
  pub fn new(backend_type: GegBackend, win: Arc<Window>) -> Self {
    let device = GegVkDevice::new(win);
    Self {
      device: device.clone(),
      renderer: GegVkRenderer::new(device),
      backend_type,
    }
  }

  pub fn update(&mut self) {
    self.renderer.render();
  }

  pub fn resize(&mut self, width: u32, height: u32) {
    // self.device.resize(width, height);
  }
}
