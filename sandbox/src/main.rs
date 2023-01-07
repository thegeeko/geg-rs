use geg::app::{GegApp, GegAppOptions};

fn main() {
  geg::init();
 
  let opts = GegAppOptions::default();
  let mut app = GegApp::new(opts);

  geg::info!("Starting app");
  app.run();
}
