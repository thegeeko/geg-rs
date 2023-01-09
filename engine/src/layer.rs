use crate::io::ModifiersState;
pub use crate::events::GegEvent;

/// A layer represent a part or a moudle of the application.
pub trait Layer {
  /// Called when the layer is attached to the application.
  /// usful for initializing resources used by the layer.
  fn on_attach(&mut self) -> () {}

  /// Called when the layer is detached from the application.
  /// usful for cleaning up resources used by the layer.
  fn on_detach(&mut self) -> () {}

  /// Called when the layer is updated.
  /// # Arguments
  /// * `dt` - The time in seconds since the last update(delta time).
  fn on_update(&mut self, dt: f32) -> ();

  /// Called with each event.
  /// # Arguments
  /// * `event` - The event that occured.
  ///
  /// # Returns
  /// * `true` - If the event was handled by the layer and shouldn't be passed down to other layers.
  /// * `false` - If the event wasn't handled by the layer and should be passed down to other layers.
  fn on_event(&mut self, event: GegEvent, modifiers: ModifiersState) -> bool {
    false
  }
}
