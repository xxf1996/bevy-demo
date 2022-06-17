// 这里是直接从bevy_pbr包里引入相关的着色器模块
#import bevy_pbr::mesh_view_bind_group
#import bevy_pbr::mesh_struct

let PI: f32 = 3.141592653589793;
let light_pos: vec3<f32> = vec3<f32>(0.0, 3.0, 0.0);
let light_radiance: f32 = 80.0;

// attribute
struct Vertex {
  [[location(0)]] position: vec3<f32>;
  [[location(1)]] normal: vec3<f32>;
  [[location(2)]] uv: vec2<f32>;
};

// uniform
struct Material {
  base_color: vec4<f32>;
  metallic: f32;
  roughness: f32;
};

[[group(1), binding(0)]]
var<uniform> material: Material;
[[group(1), binding(1)]]
var texture_t: texture_2d<f32>;
[[group(1), binding(2)]]
var texture_s: sampler;

[[group(2), binding(0)]]
var<uniform> mesh: Mesh; // Mesh结构来自上述引入代码中：https://github.com/bevyengine/bevy/blob/83c6ffb73c4a91182cda10141f824987ef3fba2f/crates/bevy_pbr/src/render/mesh_struct.wgsl#L3

struct VertexOutput {
  [[builtin(position)]] clip_position: vec4<f32>;
  [[location(0)]] normal: vec3<f32>;
  [[location(1)]] uv: vec2<f32>;
  [[location(2)]] world_position: vec3<f32>;
};

[[stage(vertex)]]
fn vertex(vertex: Vertex) -> VertexOutput {
  let world_position = mesh.model * vec4<f32>(vertex.position, 1.0);

  var out: VertexOutput;
  out.clip_position = view.view_proj * world_position; // view也是：https://github.com/bevyengine/bevy/blob/83c6ffb73c4a91182cda10141f824987ef3fba2f/crates/bevy_pbr/src/render/mesh_view_bind_group.wgsl#L3
  let norm = mesh.inverse_transpose_model * vec4<f32>(vertex.normal, 1.0);
  out.normal = norm.xyz / norm.w;
  out.uv = vertex.uv;
  out.world_position = world_position.xyz / world_position.w;
  return out;
}

struct FragmentInput {
  [[location(0)]] normal: vec3<f32>;
  [[location(1)]] uv: vec2<f32>;
  [[location(2)]] world_position: vec3<f32>;
};

fn DistributionGGX(N: vec3<f32>, H: vec3<f32>, roughness: f32) -> f32
{
  // To calculate GGX NDF here
  let alpha = roughness * roughness;
  let a2 = alpha * alpha;
  let NoH = dot(N, H);
  let nom   = a2;
  var denom = (NoH * NoH * (a2 - 1.0) + 1.0);
  denom = PI * denom * denom;

  return nom / max(denom, 0.0001);
}

fn GeometrySchlickGGX(NdotV: f32, roughness: f32) -> f32
{
  // To calculate Smith G1 here

  let k = (roughness + 1.0) * (roughness + 1.0) / 8.0;

  return NdotV / (NdotV * (1.0 - k) + k);
}

fn GeometrySmith(N: vec3<f32>, V: vec3<f32>, L: vec3<f32>, roughness: f32) -> f32
{
  // To calculate Smith G here

  let G1 = GeometrySchlickGGX(dot(N, V), roughness);
  let G2 = GeometrySchlickGGX(dot(N, L), roughness);

  return G1 * G2;
}

fn fresnelSchlick(F0: vec3<f32>, V: vec3<f32>, H: vec3<f32>) -> vec3<f32>
{
  // To calculate Schlick F here

  return F0 + (1.0 - F0) * pow((1.0 - dot(V, H)), 5.0);
}

[[stage(fragment)]]
fn fragment(input: FragmentInput) -> [[location(0)]] vec4<f32> {
  let albedo = (material.base_color * textureSample(texture_t, texture_s, input.uv)).xyz;
  let N = normalize(input.normal);
  let V = normalize(view.world_position - input.world_position);
  let NdotV = max(dot(N, V), 0.0);
  var F0 = vec3<f32>(0.04);
  F0 = mix(F0, albedo, material.metallic);
  var Lo = vec3<f32>(0.0);
  let L = normalize(light_pos - input.world_position);
  let H = normalize(V + L);
  let NdotL = max(dot(N, L), 0.0);
  let radiance = light_radiance;
  let NDF = DistributionGGX(N, H, material.roughness);
  let G = GeometrySmith(N, V, L, material.roughness);
  let F = fresnelSchlick(F0, V, H);
  let ks = F0;
  let kd = albedo * (1.0 - material.metallic);

  let numerator = NDF * G * F;
  let denominator = max((4.0 * NdotL * NdotV), 0.001);
  let BRDF_S = numerator / denominator; // 镜面反射BRDF
  let BRDF_L = albedo / PI; // 漫反射BRDF
  let BRDF_Micro = ks * BRDF_S + kd * BRDF_L; // 微表面模型反射

  Lo = Lo + BRDF_Micro * radiance * NdotL; // 目前还不支持 += ？
  var color: vec3<f32> = Lo;
  color = color / (color + vec3<f32>(1.0));
  color = pow(color, vec3<f32>(1.0 / 2.2));

  return vec4<f32>(color, 1.0);
}
