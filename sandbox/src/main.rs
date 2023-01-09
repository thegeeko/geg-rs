use geg::app::{GegApp, GegAppOptions};
use geg::events::GegEvent;
use geg::io::{Key, MouseButton, ModifiersState};
use geg::layer::Layer;

struct ExampleLayer;
impl Layer for ExampleLayer {
  fn on_update(&mut self, _dt: f32) {}

  fn on_event(&mut self, event: GegEvent, modifiers: ModifiersState) -> bool {
    match event {
      GegEvent::KeyDown(key) => match key {
        Key::Escape => {
          geg::info!("Escape pressed");
        }
        Key::Space => {
          if modifiers.ctrl {
            geg::info!("Ctrl + Space pressed");
          } else {
            geg::info!("Space pressed");
          }
        }
        _ => (),
      },

      GegEvent::MouseButtonDown(btn) => match btn {
        MouseButton::Left => {
          if modifiers.ctrl {
            geg::info!("Ctrl + Left mouse button pressed");
          } else {
            geg::info!("Left mouse button pressed");
          }
        }

        MouseButton::Right => {
          geg::info!("Right mouse button pressed");
        },

        _ => (),
      },

      GegEvent::MouseMoved(pos) => {
        geg::info!("Mouse moved to: {:?}", pos);
      }

      GegEvent::MouseRaw(pos) => {
        geg::info!("Mouse raw moved to: {:?}", pos);
      }

      _ => (),
    }

    false
  }
}

fn main() {
  geg::init();

  let opts = GegAppOptions::default();
  let mut app = GegApp::new(opts);

  let layer = Box::new(ExampleLayer);
  app.add_layer(layer);

  geg::info!("Starting app");
  app.run();
}
