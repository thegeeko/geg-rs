use std::sync::Arc;
use vulkano::device::physical::{PhysicalDevice, PhysicalDeviceType};
use vulkano::device::{
  Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo, QueueFlags,
};
use vulkano::instance::{Instance, InstanceCreateInfo, Version};
use vulkano::swapchain::Surface;
use vulkano::VulkanLibrary;

use spdlog::prelude::*;
use winit::window::Window;

pub(crate) struct GegVkDevice {
  win: Arc<Window>,
  instance: Arc<Instance>,
  physical_device: Arc<PhysicalDevice>,
  surface: Arc<Surface>,
  device: Arc<Device>,
  queue: Arc<Queue>,
}

impl GegVkDevice {
  pub fn new(win: Arc<Window>) -> Self {
    let lib = VulkanLibrary::new().expect("failed to load Vulkan library");

    let required_extensions = vulkano_win::required_extensions(&lib);
    let instance_cration_info = InstanceCreateInfo {
      // for moltenVK
      max_api_version: Some(Version::V1_2),
      enumerate_portability: true,
      enabled_extensions: required_extensions,
      ..Default::default()
    };

    let instance =
      Instance::new(lib, instance_cration_info).expect("failed to create Vulkan instance");

    let surface = vulkano_win::create_surface_from_winit(win.clone(), instance.clone())
      .expect("failed to create surface");

    // required extensions
    let device_extensions = DeviceExtensions {
      khr_swapchain: true,
      ..DeviceExtensions::empty()
    };

    let (physical_device, queue_family_index) = instance
      .enumerate_physical_devices()
      .unwrap()
      // filter for devices that support swapchain
      .filter(|p| p.supported_extensions().contains(&device_extensions))
      .filter_map(|p| {
        // filter for devices that support graphics and presentation
        p.queue_family_properties()
          .iter()
          .enumerate()
          .position(|(i, q)| {
            q.queue_flags.intersects(&QueueFlags {
              graphics: true,
              ..Default::default()
            }) && p.surface_support(i as u32, &surface).unwrap_or(false)
          })
          // convert to (device, index_q_family)
          .map(|i| (p, i as u32))
      })
      .min_by_key(|(p, _)| {
        // We assign a lower score to device types that are likely to be faster/better.
        match p.properties().device_type {
          PhysicalDeviceType::DiscreteGpu => 0,
          PhysicalDeviceType::IntegratedGpu => 1,
          PhysicalDeviceType::VirtualGpu => 2,
          PhysicalDeviceType::Cpu => 3,
          PhysicalDeviceType::Other => 4,
          _ => 5,
        }
      })
      .expect("No suitable physical device found");

    info!(
      "Vulkan physical device: {}({:#?})",
      physical_device.properties().device_name,
      physical_device.properties().device_type
    );
    info!("API Version: {}", physical_device.properties().api_version);

    let (device, mut queues) = Device::new(
      physical_device.clone(),
      DeviceCreateInfo {
        queue_create_infos: vec![QueueCreateInfo {
          queue_family_index,
          ..Default::default()
        }],
        enabled_extensions: device_extensions,
        ..Default::default()
      },
    )
    .unwrap();
    info!("created Vulkan device");

    Self {
      instance,
      physical_device,
      surface,
      device,
      queue: queues.next().unwrap(),
      win,
    }
  }
}
