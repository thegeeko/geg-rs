use crate::{
  events::GegEvent,
  io::{to_geg_keycode, to_geg_mousebtn, ModifiersState},
  layer::Layer,
};

use glam::DVec2;

use std::time::Instant;

use winit::{
  event::{DeviceEvent, Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  platform::run_return::EventLoopExtRunReturn,
  window::{Window, WindowBuilder},
};

pub struct GegAppOptions {
  pub name: String,
}

impl Default for GegAppOptions {
  fn default() -> Self {
    Self {
      name: "Geg App".to_string(),
    }
  }
}

pub struct GegApp {
  name: String,
  window: Window,
  event_loop: Option<EventLoop<()>>,
  layers: Vec<Box<dyn Layer>>,
  last_frame_time: Instant,
  modifier_state: ModifiersState,
}

impl GegApp {
  pub fn new(opts: GegAppOptions) -> Self {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
      .with_title(opts.name.clone())
      .build(&event_loop)
      .unwrap();

    GegApp {
      name: opts.name,
      window,
      event_loop: Some(event_loop),
      layers: Vec::new(),
      last_frame_time: Instant::now(),
      modifier_state: ModifiersState::default(),
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

            WindowEvent::ModifiersChanged(modifiers) => {
              self.modifier_state.ctrl = modifiers.ctrl();
              self.modifier_state.shift = modifiers.shift();
              self.modifier_state.alt = modifiers.alt();
              self.modifier_state.logo = modifiers.logo();
            }

            WindowEvent::KeyboardInput { input, .. } => {
              let key = to_geg_keycode(input.virtual_keycode.unwrap());
              let is_down = input.state == winit::event::ElementState::Pressed;

              if is_down {
                for layer in &mut self.layers {
                  layer.on_event(GegEvent::KeyDown(key), self.modifier_state);
                }
              } else {
                for layer in &mut self.layers {
                  layer.on_event(GegEvent::KeyUp(key), self.modifier_state);
                }
              }
            }

            WindowEvent::MouseInput { state, button, .. } => {
              let is_down = state == winit::event::ElementState::Pressed;

              if is_down {
                for layer in &mut self.layers {
                  layer.on_event(
                    GegEvent::MouseButtonDown(to_geg_mousebtn(button)),
                    self.modifier_state,
                  );
                }
              } else {
                for layer in &mut self.layers {
                  layer.on_event(
                    GegEvent::MouseButtonUp(to_geg_mousebtn(button)),
                    self.modifier_state,
                  );
                }
              }
            }

            WindowEvent::CursorMoved { position, .. } => {
              let pos = DVec2::new(position.x, position.y);

              for layer in &mut self.layers {
                layer.on_event(GegEvent::MouseMoved(pos), self.modifier_state);
              }
            }

            _ => (),
          },

          Event::DeviceEvent { event, .. } => match event {
            DeviceEvent::MouseMotion { delta } => {
              let delta = DVec2::new(delta.0 as f64, delta.1 as f64);
              for layer in &mut self.layers {
                layer.on_event(GegEvent::MouseRaw(delta), self.modifier_state);
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
