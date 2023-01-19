use super::{device::GegVkDevice, renderpass::GegVkRenderpass, swapchain::GegVkSwapchain};

use bytemuck::{Pod, Zeroable};
use spdlog::prelude::*;
use std::collections::{hash_map, HashMap};
use std::sync::Arc;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::allocator::StandardCommandBufferAllocator;
use vulkano::command_buffer::{
  AutoCommandBufferBuilder, CommandBufferUsage, RenderPassBeginInfo,
  SecondaryCommandBufferAbstract, SubpassContents,
};
use vulkano::device::{Device, Queue};
use vulkano::format::Format;
use vulkano::memory::allocator::StandardMemoryAllocator;
use vulkano::pipeline::graphics::vertex_input::{
  BuffersDefinition, Vertex as VertexTrait, VertexInputAttributeDescription, VertexInputState,
};
use vulkano::pipeline::{
  graphics::{
    input_assembly::InputAssemblyState,
    viewport::{Viewport, ViewportState},
  },
  GraphicsPipeline,
};
use vulkano::render_pass::Subpass;
use vulkano::swapchain::{AcquireError, SwapchainPresentInfo};
use vulkano::sync::{self, GpuFuture};

#[repr(C)]
#[derive(Default, Copy, Clone, Zeroable, Pod)]
struct Vertex {
  position: [f32; 3],
  color: [f32; 3],
}
vulkano::impl_vertex!(Vertex, position, color);

pub(crate) struct GegVkRenderer {
  device: Arc<Device>,
  queue: Arc<Queue>,
  geg_swapchain: GegVkSwapchain,
  geg_renderpass: GegVkRenderpass,
  default_pipeline: Arc<GraphicsPipeline>,
  command_buffer_allocator: StandardCommandBufferAllocator,
  lastframe: Option<Box<dyn GpuFuture>>,
  vertex_buffer: Arc<CpuAccessibleBuffer<[Vertex]>>,
}

impl GegVkRenderer {
  pub fn new(geg_device: GegVkDevice) -> Self {
    let device = geg_device.device();
    let queue = geg_device.queue();
    let geg_swapchain = GegVkSwapchain::new(geg_device.clone());
    let geg_renderpass = GegVkRenderpass::new(geg_device.clone(), geg_swapchain.clone());
    let command_buffer_allocator =
      StandardCommandBufferAllocator::new(device.clone(), Default::default());

    // temporary
    mod vert_shader {
      vulkano_shaders::shader! {
        ty: "vertex",
        src: "
        #version 450

        layout(location = 0) in vec3 position;
        layout(location = 1) in vec3 color;

        layout(location = 0) out vec3 v_color;

        void main() {
          gl_Position = vec4(position, 1.0);
          v_color = color;
        }
      "
      }
    }

    mod frag_shader {
      vulkano_shaders::shader! {
        ty: "fragment",
        src: "
        #version 450

        layout(location = 0) in vec3 v_color;

        layout(location = 0) out vec4 f_color;

        void main() {
          f_color = vec4(v_color, 1.0);
        }
      "
      }
    }

    let vertices = [
      Vertex {
        position: [-0.5, 0.5, 0.0],
        color: [1.0, 0.0, 0.0],
      },
      Vertex {
        position: [0.5, 0.5, 0.0],
        color: [0.0, 1.0, 0.0],
      },
      Vertex {
        position: [0.0, -0.25, 0.0],
        color: [0.0, 0.0, 1.0],
      },
    ];

    let memory_allocator = StandardMemoryAllocator::new_default(device.clone());
    let vertex_buffer = CpuAccessibleBuffer::from_iter(
      &memory_allocator,
      BufferUsage {
        vertex_buffer: true,
        ..BufferUsage::empty()
      },
      true,
      vertices,
    )
    .unwrap();

    let vs = vert_shader::load(device.clone()).expect("failed to create shader module");
    let fs = frag_shader::load(device.clone()).expect("failed to create shader module");

    let viewport = Viewport {
      origin: [0.0, 0.0],
      dimensions: geg_device.window().inner_size().into(),
      depth_range: 0.0..1.0,
    };

    let pipeline = GraphicsPipeline::start()
      .vertex_input_state(BuffersDefinition::new().vertex::<Vertex>())
      .vertex_shader(vs.entry_point("main").unwrap(), ())
      .input_assembly_state(InputAssemblyState::new())
      .viewport_state(ViewportState::viewport_fixed_scissor_irrelevant([viewport]))
      .fragment_shader(fs.entry_point("main").unwrap(), ())
      .render_pass(Subpass::from(geg_renderpass.render_pass(), 0).unwrap())
      .build(device.clone())
      .unwrap();

    debug!("Renderer created");
    Self {
      device: device.clone(),
      queue,
      geg_swapchain,
      geg_renderpass,
      default_pipeline: pipeline,
      command_buffer_allocator,
      lastframe: Some(Box::new(sync::now(device.clone()))),
      vertex_buffer,
    }
  }

  pub fn render(&mut self) {
    self.lastframe.as_mut().unwrap().cleanup_finished();

    let mut builder = AutoCommandBufferBuilder::primary(
      &self.command_buffer_allocator,
      self.queue.queue_family_index(),
      CommandBufferUsage::MultipleSubmit,
    )
    .unwrap();

    let (image_index, suboptimal, acquire_future) =
      match vulkano::swapchain::acquire_next_image(self.geg_swapchain.swapchain(), None) {
        Ok(r) => r,
        Err(AcquireError::OutOfDate) => {
          return;
        }
        Err(e) => panic!("Failed to acquire next image: {e:?}"),
      };

    builder
      .begin_render_pass(
        RenderPassBeginInfo {
          clear_values: vec![Some([0.0, 0.0, 0.0, 1.0].into())],
          ..RenderPassBeginInfo::framebuffer(
            self.geg_renderpass.frame_buffers()[image_index as usize].clone(),
          )
        },
        SubpassContents::Inline,
      )
      .unwrap()
      // new stuff
      .bind_pipeline_graphics(self.default_pipeline.clone())
      .bind_vertex_buffers(0, self.vertex_buffer.clone())
      .draw(
        3, 1, 0, 0, // 3 is the number of vertices, 1 is the number of instances
      )
      .unwrap()
      .end_render_pass()
      .unwrap();

    let command_buffer = builder.build().unwrap();

    let future = self
      .lastframe
      .take()
      .unwrap_or_else(|| Box::new(sync::now(self.device.clone())))
      .join(acquire_future)
      .then_execute(self.queue.clone(), command_buffer)
      .unwrap()
      .then_swapchain_present(
        self.queue.clone(),
        SwapchainPresentInfo::swapchain_image_index(self.geg_swapchain.swapchain(), image_index),
      )
      .then_signal_fence_and_flush();

    if future.is_ok() {
      self.lastframe = Some(Box::new(future.unwrap()));
    } else {
      self.lastframe = Some(Box::new(sync::now(self.device.clone())));
    }
  }
}
