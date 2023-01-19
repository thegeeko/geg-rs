use super::device::GegVkDevice;

use std::sync::Arc;
use vulkano::format::Format;
use vulkano::image::ImageUsage;
use vulkano::image::SwapchainImage;
use vulkano::swapchain::{ColorSpace, Swapchain, SwapchainCreateInfo, PresentMode};

use spdlog::prelude::*;

#[derive(Clone)]
pub(super) struct GegVkSwapchain {
  swapchain: Arc<Swapchain>,
  images: Vec<Arc<SwapchainImage>>,
  format: Format,
  color_space: ColorSpace,
}

impl GegVkSwapchain {
  pub fn new(geg_device: GegVkDevice) -> Self {
    let surface = geg_device.surface();
    let physical_device = geg_device.physical_device();

    let caps = physical_device
      .surface_capabilities(&surface, Default::default())
      .expect("failed to get surface capabilities");

    let composite_alpha = caps.supported_composite_alpha.iter().next().unwrap();

    let avaliable_formats_colorspaces = physical_device
      .surface_formats(&surface, Default::default())
      .expect("failed to get surface formats");

    let (format, color_space) = *avaliable_formats_colorspaces
      .iter()
      .find(|fc| {
        let (format, color_space) = fc;
        *format == Format::B8G8R8A8_SRGB && *color_space == ColorSpace::SrgbNonLinear
      })
      .unwrap_or_else(|| {
        panic!(
          "failed to find a supported format: {:?}",
          avaliable_formats_colorspaces
        )
      });

    let dimensions = geg_device.window().inner_size();

    let present_mode = physical_device
      .surface_present_modes(&surface)
      .expect("failed to get surface present modes")
      .find(|&pm| pm == PresentMode::Mailbox)
      .unwrap_or(PresentMode::Fifo);

    info!("Present mode: {:?}", present_mode);


    let (swapchain, images) = Swapchain::new(
      geg_device.device(),
      geg_device.surface(),
      SwapchainCreateInfo {
        min_image_count: caps.min_image_count + 1, // How many buffers to use in the swapchain
        image_extent: dimensions.into(),
        image_format: Some(format),
        image_color_space: color_space,
        present_mode,
        image_usage: ImageUsage {
          color_attachment: true, // What the images are going to be used for
          ..Default::default()
        },
        composite_alpha,
        ..Default::default()
      },
    )
    .unwrap();

    debug!("Swapchain created");

    Self {
      swapchain,
      images,
      format,
      color_space,
    }
  }

  // getters
  pub fn swapchain(&self) -> Arc<Swapchain> {
    self.swapchain.clone()
  }

  pub fn images(&self) -> &Vec<Arc<SwapchainImage>> {
    &self.images
  }

  pub fn format(&self) -> Format {
    self.format
  }

  pub fn color_space(&self) -> ColorSpace {
    self.color_space
  }
}
