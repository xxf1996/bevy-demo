use bevy::prelude::*;
use bevy_obj::*;
use bevy_egui::{ egui, EguiContext, EguiPlugin };

pub struct SetupPlugin;

#[derive(Clone, Debug)]
pub struct ObjMaterial {
  color: Color,
}

impl Material for ObjMaterial {
  fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
    Some(asset_server.load("shader/obj_custom.wgsl"))
  }
}

impl Plugin for SetupPlugin {
  fn build(&self, app: &mut App) {
    app.insert_resource(AmbientLight {
      color: Color::WHITE,
      brightness: 0.2f32
    });
  }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<StandardMaterial>>) {
  commands.spawn_bundle(MaterialMeshBundle {
    mesh: asset_server.load("model/bunny/bunny.obj"),
    transform: Transform::from_xyz(0., 0., 0.),
    material: materials.add(StandardMaterial {
      base_color: Color::Rgba { red: 0.8, green: 0.2, blue: 0.2, alpha: 1.0 },
      ..default()
    }),
    ..default()
  });
  commands.spawn_bundle(PerspectiveCameraBundle {
    transform: Transform::from_xyz(1., 1., 1.).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
    ..default()
  });
  commands.spawn_bundle(DirectionalLightBundle {
    directional_light: DirectionalLight {
      color: Color::WHITE,
      ..default()
    },
    ..default()
  });
}

fn rotate_obj(
  time: Res<Time>,
  mut query: Query<&mut Transform, With<Handle<Mesh>>>
) {
  for mut transform in query.iter_mut() {
    transform.rotation = Quat::from_euler(
      EulerRot::ZYX,
      0.0,
      time.seconds_since_startup() as f32 * std::f32::consts::TAU / 10.0,
      -std::f32::consts::FRAC_PI_4,
    );
  }
}

fn use_ui(mut egui_ctx: ResMut<EguiContext>) {
  egui::Window::new("参数配置")
    .show(egui_ctx.ctx_mut(), |ui| {
      ui.
      ui.label("Hello");
    });
}

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(ObjPlugin)
    .add_plugin(EguiPlugin)
    .add_plugin(SetupPlugin)
    .add_startup_system(setup)
    .add_system(rotate_obj)
    .add_system(use_ui)
    .run();
}
