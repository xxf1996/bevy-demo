use bevy::prelude::*;
use smooth_bevy_cameras::{
  controllers::orbit::{
    OrbitCameraBundle,
    OrbitCameraController,
    OrbitCameraPlugin
  },
  LookTransformPlugin
};

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn_scene(asset_server.load("model/realistic interior/realistic interior.glb#Scene0")); // 需要指定模型名称（#后面的名称），不然加载为空
  commands.spawn_bundle(OrbitCameraBundle::new(
    OrbitCameraController::default(),
    PerspectiveCameraBundle {
      transform: Transform::from_xyz(1., 1., 1.).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
      ..default()
    },
    Vec3::new(1., 1., 1.),
    Vec3::new(0., 0., 0.)
  ));
  commands.spawn_bundle(PointLightBundle {
    point_light: PointLight {
      color: Color::WHITE,
      intensity: 800.,
      radius: 4.,
      ..default()
    },
    ..default()
  });
}

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(LookTransformPlugin)
    .add_plugin(OrbitCameraPlugin::default())
    .add_startup_system(setup)
    .run();
}
