#import bevy_pbr::mesh_view_bind_group // 这里是直接从bevy_pbr包里引入相关的着色器模块
#import bevy_pbr::mesh_struct

struct Vertex {
  [[location(0)]] position: vec3<f32>;
  [[location(1)]] normal: vec3<f32>;
  [[location(2)]] uv: vec2<f32>;
};

[[group(2), binding(0)]]
var<uniform> mesh: Mesh; // Mesh结构来自上述引入代码中：https://github.com/bevyengine/bevy/blob/83c6ffb73c4a91182cda10141f824987ef3fba2f/crates/bevy_pbr/src/render/mesh_struct.wgsl#L3

struct VertexOutput {
  [[builtin(position)]] clip_position: vec4<f32>;
  [[location(0)]] normal: vec3<f32>;
  [[location(1)]] uv: vec2<f32>;
};

[[stage(vertex)]]
fn vertex(vertex: Vertex) -> VertexOutput {
  let world_position = mesh.model * vec4<f32>(vertex.position, 1.0);

  var out: VertexOutput;
  out.clip_position = view.view_proj * world_position; // view也是：https://github.com/bevyengine/bevy/blob/83c6ffb73c4a91182cda10141f824987ef3fba2f/crates/bevy_pbr/src/render/mesh_view_bind_group.wgsl#L3
  out.normal = vertex.normal;
  out.uv = vertex.uv;
  return out;
}

struct FragmentInput {
  [[location(0)]] normal: vec3<f32>;
  [[location(1)]] uv: vec2<f32>;
};

[[stage(fragment)]]
fn fragment(input: FragmentInput) -> [[location(0)]] vec4<f32> {
  return vec4<f32>(input.normal, 1.0);
}
