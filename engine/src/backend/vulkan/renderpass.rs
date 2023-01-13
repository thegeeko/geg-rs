use std::sync::Arc;
use vulkano::{
  image::view::ImageView,
  render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass},
};

use super::{device::GegVkDevice, swapchain::GegVkSwapchain};

pub(super) struct GegVkRenderPass {
  render_pass: Arc<RenderPass>,
  frame_buffers: Vec<Arc<Framebuffer>>,
}

impl GegVkRenderPass {
  pub fn new(geg_swapchain: GegVkSwapchain, geg_device: GegVkDevice) -> Self {
    let render_pass = vulkano::single_pass_renderpass!(
      geg_device.device(),
      attachments: {
        color: {
          load: Clear,
          store: Store,
          format: geg_swapchain.format(),
          samples: 1,
        }
      },
      pass: {
        color: [color],
        depth_stencil: {}
      }
    )
    .unwrap();

    let frame_buffers = geg_swapchain
      .images()
      .iter()
      .map(|image| {
        let view = ImageView::new_default(image.clone()).unwrap();

        Framebuffer::new(
          render_pass.clone(),
          FramebufferCreateInfo {
            attachments: vec![view],
            ..Default::default()
          },
        )
        .unwrap()
      })
      .collect::<Vec<_>>();

    Self {
      render_pass,
      frame_buffers,
    }
  }
}
