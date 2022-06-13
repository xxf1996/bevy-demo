mod obj_custom;

use rand::Rng;
use bevy::{prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, gltf::{Gltf, GltfMesh, GltfNode}};
use smooth_bevy_cameras::{
  controllers::orbit::{
    OrbitCameraBundle,
    OrbitCameraController,
    OrbitCameraPlugin
  },
  LookTransformPlugin
};
use obj_custom::{
  ObjMaterial,
};


struct GltfAssetPack(Handle<Gltf>);
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct GltfLoaded(bool); // 自定义状态，参考：https://bevy-cheatbook.github.io/programming/states.html


fn load_gltf_node(
  node: &GltfNode,
  builder: &mut WorldChildBuilder,
  gltf_mesh: &Res<Assets<GltfMesh>>,
  materials: &mut ResMut<Assets<ObjMaterial>>
) {
  let transform = node.transform;
  let mut parent_bundle = builder.spawn_bundle(TransformBundle::from_transform(transform.clone()));
  let mut rng = rand::thread_rng();
  if let Some(mesh_id) = &node.mesh {
    let mesh = gltf_mesh.get(mesh_id).unwrap();
    println!("primitive num: {}", mesh.primitives.len());
    parent_bundle.with_children(|parent|  {
      for obj in mesh.primitives.iter() {
        parent.spawn_bundle(MaterialMeshBundle {
          mesh: obj.mesh.clone(),
          material: materials.add(ObjMaterial {
            color: Color::Rgba { red: rng.gen_range(0.1..0.45), green: 0.2, blue: 0.8, alpha: 1.0 },
          }), // TODO：怎么开启双面渲染？
          // transform: node.transform.clone(),
          ..default()
        });
      }
      for child in &node.children {
        load_gltf_node(child, parent, gltf_mesh, materials);
      }
    });
  }
}

/// 参考：https://bevy-cheatbook.github.io/3d/gltf.html
///
/// 自定义gltf场景节点
///
/// FIXME：目前有部分物体比例和位置不对
fn custom_gltf(
  mut commands: Commands,
  gltf_pack: Res<GltfAssetPack>,
  gltf_assets: Res<Assets<Gltf>>,
  gltf_mesh: Res<Assets<GltfMesh>>,
  gltf_nodes: Res<Assets<GltfNode>>,
  mut gltf_loaded: ResMut<State<GltfLoaded>>,
  mut scenes: ResMut<Assets<Scene>>,
  mut materials: ResMut<Assets<ObjMaterial>>
) {
  if let GltfLoaded(true) = gltf_loaded.current() {
    return;
  }
  if let Some(gltf) = gltf_assets.get(&gltf_pack.0)  {
    let mut world = World::default();
    // 参考：https://github.com/bevyengine/bevy/blob/83c6ffb73c4a91182cda10141f824987ef3fba2f/crates/bevy_gltf/src/loader.rs#L691
    world
      .spawn()
      .insert_bundle(TransformBundle::identity())
      .with_children(|parent| {
        for node_id in gltf.nodes.iter() {
          let node = gltf_nodes.get(node_id).unwrap();
          load_gltf_node(node, parent, &gltf_mesh, &mut materials);
        }
      });
    let scene_id = scenes.add(Scene::new(world));
    commands.spawn_scene(scene_id);
    gltf_loaded.set(GltfLoaded(true)).unwrap();
  }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  // commands.spawn_scene(asset_server.load("model/realistic interior/realistic interior.glb#Scene0")); // 需要指定模型名称（#后面的名称），不然加载为空
  let gltf = asset_server.load("model/realistic interior/realistic interior.glb");
  commands.insert_resource(GltfAssetPack(gltf)); // 仅把gltf作为资源插入
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
    transform: Transform::from_xyz(0., 3., 0.),
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
    .add_plugin(MaterialPlugin::<ObjMaterial>::default()) // 添加材质
    .add_state(GltfLoaded(false))
    .add_startup_system(setup)
    .add_system(custom_gltf)
    .run();
}
