use crate::{
  backend::{GegBackend, GraphicsContext},
  events::GegEvent,
  io::{to_geg_keycode, to_geg_mousebtn, ModifiersState},
  layer::Layer,
};

use glam::DVec2;

use std::sync::Arc;
use std::time::Instant;

use winit::{
  event::{DeviceEvent, Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  platform::run_return::EventLoopExtRunReturn,
  window::{Window, WindowBuilder},
};

pub struct GegAppOptions {
  pub name: String,
  pub backend: GegBackend,
}

impl Default for GegAppOptions {
  fn default() -> Self {
    Self {
      name: "Geg App".to_string(),
      backend: GegBackend::Vulkano,
    }
  }
}

pub struct GegApp {
  name: String,
  window: Arc<Window>,
  event_loop: Option<EventLoop<()>>,
  layers: Vec<Box<dyn Layer>>,
  last_frame_time: Instant,
  modifier_state: ModifiersState,
  graphics_context: GraphicsContext,
}

impl GegApp {
  pub fn new(opts: GegAppOptions) -> Self {
    let event_loop = EventLoop::new();
    let window = Arc::new(
      WindowBuilder::new()
        .with_title(opts.name.clone())
        .build(&event_loop)
        .unwrap(),
    );

    GegApp {
      name: opts.name,
      window: window.clone(),
      event_loop: Some(event_loop),
      layers: Vec::new(),
      last_frame_time: Instant::now(),
      modifier_state: ModifiersState::default(),
      graphics_context: GraphicsContext::new(opts.backend, window),
    }
  }

  pub fn run(&mut self) {
    self
      .event_loop
      .take()
      .unwrap()
      .run_return(move |event, _, control_flow| {
        let dt = self.last_frame_time.elapsed().as_secs_f32();
        self.last_frame_time = Instant::now();

        *control_flow = ControlFlow::Poll;

        match event {
          Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => {
              *control_flow = ControlFlow::Exit;
            }

            WindowEvent::Resized(size) => {
              // sel
            }

            WindowEvent::ModifiersChanged(modifiers) => {
              self.modifier_state.ctrl = modifiers.ctrl();
              self.modifier_state.shift = modifiers.shift();
              self.modifier_state.alt = modifiers.alt();
              self.modifier_state.logo = modifiers.logo();
            }

            WindowEvent::KeyboardInput { input, .. } => {
              let mut key = crate::io::Key::Unknown;
              // checking for some key that is not included in winit::event::VirtualKeyCode
              // like globe in macos
              if input.virtual_keycode.is_some() {
                key = to_geg_keycode(input.virtual_keycode.unwrap())
              };

              let is_down = input.state == winit::event::ElementState::Pressed;

              if is_down {
                for layer in &mut self.layers {
                  if layer.on_event(GegEvent::KeyDown(key), self.modifier_state) {
                    break;
                  }
                }
              } else {
                for layer in &mut self.layers {
                  if layer.on_event(GegEvent::KeyUp(key), self.modifier_state) {
                    break;
                  };
                }
              }
            }

            WindowEvent::MouseInput { state, button, .. } => {
              let is_down = state == winit::event::ElementState::Pressed;

              if is_down {
                for layer in &mut self.layers {
                  if layer.on_event(
                    GegEvent::MouseButtonDown(to_geg_mousebtn(button)),
                    self.modifier_state,
                  ) {
                    break;
                  }
                }
              } else {
                for layer in &mut self.layers {
                  if layer.on_event(
                    GegEvent::MouseButtonUp(to_geg_mousebtn(button)),
                    self.modifier_state,
                  ) {
                    break;
                  }
                }
              }
            }

            WindowEvent::CursorMoved { position, .. } => {
              let pos = DVec2::new(position.x, position.y);

              for layer in &mut self.layers {
                if layer.on_event(GegEvent::MouseMoved(pos), self.modifier_state) {
                  break;
                }
              }
            }

            _ => (),
          },

          Event::DeviceEvent { event, .. } => match event {
            DeviceEvent::MouseMotion { delta } => {
              let delta = DVec2::new(delta.0 as f64, delta.1 as f64);
              for layer in &mut self.layers {
                if layer.on_event(GegEvent::MouseRaw(delta), self.modifier_state) {
                  break;
                }
              }
            }
            _ => (),
          },

          Event::MainEventsCleared => {
            let layers = &mut self.layers;
            for layer in layers.iter_mut() {
              layer.on_update(dt);
            }
          }

          _ => (),
        }
      });
  }

  pub fn add_layer(&mut self, layer: Box<dyn Layer>) {
    self.layers.push(layer);
  }

  pub fn name(&self) -> &str {
    &self.name
  }
}
