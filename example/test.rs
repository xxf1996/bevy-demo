use bevy::prelude::*;

fn setup() {
  println!("test");
}

fn main() {
  App::new().add_startup_system(setup).run();
}
