use crate::io::{Key, MouseButton};
use glam::{DVec2};

/// A event that occured.
#[derive(Debug)]
pub enum GegEvent {
  KeyDown(Key),
  KeyUp(Key),
  MouseButtonDown(MouseButton),
  MouseButtonUp(MouseButton),
  MouseMoved(DVec2),
  MouseRaw(DVec2),
}
