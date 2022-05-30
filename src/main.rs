use bevy::prelude::*;

fn say_hello() {
    println!("Hello, world!");
}

fn main() {
    App::new().add_system(say_hello).run();
}
