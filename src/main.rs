use bevy::prelude::*;

#[derive(Component)]
struct Position {
    x: f32,
    y: f32
}

#[derive(Component)]
struct Name(String);

pub struct DemoPlugin;
struct DemoTimer(Timer);

impl Plugin for DemoPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DemoTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(setup)
            .add_system(view_postions);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn().insert(Position {
        x: 1.2,
        y: 2.2
    }).insert(Name("P1".to_string()));
    commands.spawn().insert(Position {
        x: -22.3,
        y: 0.45
    });
    commands.spawn().insert(Position {
        x: -1.3,
        y: 34.45
    }).insert(Name("P3".to_string()));
}

fn view_postions(time: Res<Time>, mut timer: ResMut<DemoTimer>, query: Query<&Position, With<Name>>) {
    if timer.0.tick(time.delta()).just_finished() {
        say_hello();
        for pos in query.iter() {
            println!("position is ({}, {})!", pos.x, pos.y);
        }
    }
}

fn say_hello() {
    println!("Hello, world!");
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(DemoPlugin)
        .run();
}
