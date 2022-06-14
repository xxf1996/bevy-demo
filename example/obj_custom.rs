use bevy::{
  prelude::*,
  render::{
    render_asset::RenderAsset, renderer::RenderDevice, render_resource::{Buffer, BindGroup, BufferInitDescriptor, std140::{AsStd140, Std140}, BufferUsages, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, ShaderStages, BindingType, BufferBindingType, BufferSize },
  }, ecs::system::lifetimeless::SRes, pbr::MaterialPipeline, reflect::TypeUuid
};
use bevy_obj::*;
use bevy_egui::{ egui, EguiContext, EguiPlugin };

pub struct SetupPlugin;

#[derive(Clone, Debug, TypeUuid)]
#[uuid = "031c3ce6-8962-404b-86aa-c5c9f8be99b4"]
pub struct ObjMaterial {
  pub color: Color,
}

pub struct ObjMaterialData {
  _buffer: Buffer,
  bind_group: BindGroup,
}

impl RenderAsset for ObjMaterial {
  type ExtractedAsset = ObjMaterial;
  type Param = (SRes<RenderDevice>, SRes<MaterialPipeline<Self>>);
  type PreparedAsset = ObjMaterialData;
  fn extract_asset(&self) -> Self::ExtractedAsset {
    self.clone()
  }

  fn prepare_asset(
    extracted_asset: Self::ExtractedAsset,
    (render_device, material_pipeline): &mut bevy::ecs::system::SystemParamItem<Self::Param>,
  ) -> Result<Self::PreparedAsset, bevy::render::render_asset::PrepareAssetError<Self::ExtractedAsset>> {
    let color = Vec4::from_slice(&extracted_asset.color.as_linear_rgba_f32());
    let buffer = (render_device as &mut Res<'_, RenderDevice>).create_buffer_with_data(&BufferInitDescriptor {
      contents: color.as_std140().as_bytes(),
      label: Some("obj_buffer"),
      usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
    });
    let bind_group = (render_device as &mut Res<'_, RenderDevice>).create_bind_group(&BindGroupDescriptor {
      entries: &[
        BindGroupEntry {
          binding: 0,
          resource: buffer.as_entire_binding(),
        }
      ],
      label: Some("obj_bind_group"),
      layout: &(material_pipeline as &mut Res<'_, MaterialPipeline<Self>>).material_layout,
    });

    Ok(ObjMaterialData {
      _buffer: buffer,
      bind_group,
    })
  }
}

impl Material for ObjMaterial {
  fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
    Some(asset_server.load("shader/obj_custom.wgsl"))
  }

  fn bind_group(material: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
    &material.bind_group
  }

  fn bind_group_layout(render_device: &RenderDevice) -> bevy::render::render_resource::BindGroupLayout {
    render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
      entries: &[
        BindGroupLayoutEntry {
          binding: 0,
          visibility: ShaderStages::FRAGMENT,
          ty: BindingType::Buffer {
            ty: BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: BufferSize::new(Vec4::std140_size_static() as u64)
          },
          count: None
        }
      ],
      label: Some("obj_bind_group_layout")
    })
  }

  fn specialize(
    _pipeline: &MaterialPipeline<Self>,
    descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
    _layout: &bevy::render::mesh::MeshVertexBufferLayout,
  ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
    descriptor.primitive.cull_mode = None; // 开启双面渲染
    Ok(())
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ObjMaterial>>) {
  commands.spawn_bundle(MaterialMeshBundle {
    mesh: asset_server.load("model/bunny/bunny.obj"),
    transform: Transform::from_xyz(0., 0., 0.),
    material: materials.add(ObjMaterial {
      color: Color::Rgba { red: 0.4, green: 0.2, blue: 0.8, alpha: 1.0 },
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

/// 加载中文字体，egui默认不支持显示中文
///
/// 参考：https://github.com/emilk/egui/blob/master/examples/custom_font/src/main.rs
fn load_custom_font(ctx: &egui::Context) {
  let mut fonts = egui::FontDefinitions::default();
  fonts
    .font_data
    .insert("apple".to_owned(), egui::FontData::from_static(include_bytes!("../assets/fonts/苹方黑体-中黑-简.ttf")));
  fonts
    .families
    .entry(egui::FontFamily::Proportional)
    .or_default()
    .insert(0, "apple".to_owned());
  fonts
    .families
    .entry(egui::FontFamily::Monospace)
    .or_default()
    .push("apple".to_owned());

  ctx.set_fonts(fonts);
}

fn use_ui(mut egui_ctx: ResMut<EguiContext>) {
  load_custom_font(egui_ctx.ctx_mut());
  egui::Window::new("参数配置")
    .show(egui_ctx.ctx_mut(), |ui| {
      ui.label("Hello");
    });
}

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(ObjPlugin)
    .add_plugin(EguiPlugin)
    .add_plugin(SetupPlugin)
    .add_plugin(MaterialPlugin::<ObjMaterial>::default())
    .add_startup_system(setup)
    .add_system(rotate_obj)
    .add_system(use_ui)
    .run();
}
