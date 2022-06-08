use bevy::{prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, gltf::{Gltf, GltfMesh}};
use smooth_bevy_cameras::{
  controllers::orbit::{
    OrbitCameraBundle,
    OrbitCameraController,
    OrbitCameraPlugin
  },
  LookTransformPlugin
};

struct GltfAssetPack(Handle<Gltf>);
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct GltfLoaded(bool); // 自定义状态，参考：https://bevy-cheatbook.github.io/programming/states.html

/// 参考：https://bevy-cheatbook.github.io/3d/gltf.html
fn custom_gltf(mut commands: Commands, gltf_pack: Res<GltfAssetPack>, gltf_assets: Res<Assets<Gltf>>, gltf_mesh: Res<Assets<GltfMesh>>, mut gltf_loaded: ResMut<State<GltfLoaded>>) {
  if let GltfLoaded(true) = gltf_loaded.current() {
    return;
  }
  if let Some(gltf) = gltf_assets.get(&gltf_pack.0)  {
    for mesh_id in gltf.meshes.iter() { // 遍历场景的物体
      let mesh = gltf_mesh.get(&*mesh_id).unwrap();
      commands.spawn_bundle(PbrBundle { // 重新组合mesh和material
        mesh: mesh.primitives[0].mesh.clone(),
        material: mesh.primitives[0].material.clone().unwrap(),
        ..default()
      });
    }
    gltf_loaded.set(GltfLoaded(true)).unwrap();
  }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  // commands.spawn_scene(asset_server.load("model/realistic interior/realistic interior.glb#Scene0")); // 需要指定模型名称（#后面的名称），不然加载为空
  let gltf = asset_server.load("model/realistic interior/realistic interior.glb");
  commands.insert_resource(GltfAssetPack(gltf));
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
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_plugin(LogDiagnosticsPlugin::default())
    .add_plugin(LookTransformPlugin)
    .add_plugin(OrbitCameraPlugin::default())
    .add_state(GltfLoaded(false))
    .add_startup_system(setup)
    .add_system(custom_gltf)
    .run();
}
