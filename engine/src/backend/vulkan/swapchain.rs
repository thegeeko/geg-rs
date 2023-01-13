use super::device::GegVkDevice;

use std::sync::Arc;
use vulkano::format::Format;
use vulkano::image::ImageUsage;
use vulkano::image::SwapchainImage;
use vulkano::swapchain::{ColorSpace, Swapchain, SwapchainCreateInfo};

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

    let avaliable_formats_colorspaces = physical_device
      .surface_formats(&surface, Default::default())
      .expect("failed to get surface formats");

    let (format, color_space) = *avaliable_formats_colorspaces
      .iter()
      .filter(|fc| {
        let (format, color_space) = fc;
        *format == Format::B8G8R8A8_SRGB && *color_space == ColorSpace::SrgbNonLinear
      })
      .next()
      .unwrap_or_else(|| {
        panic!(
          "failed to find a supported format: {:?}",
          avaliable_formats_colorspaces
        )
      });

    let dimensions = geg_device.window().inner_size();
    let composite_alpha = caps.supported_composite_alpha.iter().next().unwrap();

    let (swapchain, images) = Swapchain::new(
      geg_device.device(),
      geg_device.surface(),
      SwapchainCreateInfo {
        min_image_count: caps.min_image_count + 1, // How many buffers to use in the swapchain
        image_extent: dimensions.into(),
        image_format: Some(format),
        image_color_space: color_space,
        image_usage: ImageUsage {
          color_attachment: true, // What the images are going to be used for
          ..Default::default()
        },
        composite_alpha,
        ..Default::default()
      },
    )
    .unwrap();

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
