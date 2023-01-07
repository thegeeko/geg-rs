use winit::{
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::{Window, WindowBuilder},
};

#[derive(Debug)]
pub struct GegApp {
  name: String,
  window: Window,
  event_loop: Option<EventLoop<()>>,
}

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
    }
  }

  pub fn run(&mut self) {
    self
      .event_loop
      .take()
      .unwrap()
      .run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
          Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
          } => *control_flow = ControlFlow::Exit,
          _ => (),
        }
      });
  }

  pub fn name(&self) -> &str {
    &self.name
  }
}
