#import bevy_pbr::mesh_view_bind_group
#import bevy_pbr::mesh_struct

struct Vertex {
  [[location(0)]] position: vec3<f32>;
  [[location(1)]] normal: vec3<f32>;
  [[location(2)]] uv: vec2<f32>;
};

[[group(2), binding(0)]]
var<uniform> mesh: Mesh;

struct VertexOutput {
  [[builtin(position)]] clip_position: vec4<f32>;
  [[location(0)]] normal: vec3<f32>;
  [[location(1)]] uv: vec2<f32>;
};

[[stage(vertex)]]
fn vertex(vertex: Vertex) -> VertexOutput {
  let world_position = mesh.model * vec4<f32>(vertex.position, 1.0);

  var out: VertexOutput;
  out.clip_position = view.view_proj * world_position;
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
