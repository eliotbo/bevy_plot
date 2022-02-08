// Import the standard 2d mesh uniforms and set their bind groups
#import bevy_sprite::mesh2d_view_bind_group
[[group(0), binding(0)]]
var<uniform> view: View;


#import bevy_sprite::mesh2d_struct

[[group(1), binding(0)]]
var<uniform> mesh: Mesh2d;

type float4 = vec4<f32>;
type float2 = vec2<f32>;

// The structure of the vertex buffer is as specified in `specialize()`
struct Vertex {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
    // [[location(3)]] tangent: vec2<f32>;

    // instanced
    [[location(3)]] i_pos_scale: vec4<f32>;
    [[location(4)]] i_color: vec4<f32>;
};

struct VertexOutput {
    // The vertex shader must set the on-screen position of the vertex
    [[builtin(position)]] clip_position: vec4<f32>;

    [[location(0)]] uv: vec2<f32>;
    [[location(1)]] pos_scale: vec4<f32>;
    [[location(2)]] color: vec4<f32>;
};

[[stage(vertex)]]
fn vertex(vertex: Vertex) -> VertexOutput {

    let position = vertex.position * vertex.i_pos_scale.w + vertex.i_pos_scale.xyz  ;
    let world_position = mesh.model * vec4<f32>(position, 1.0);

    var out: VertexOutput;

    out.clip_position = view.view_proj * world_position;
    out.color = vertex.i_color;
    out.uv = vertex.uv;
    out.pos_scale = vertex.i_pos_scale;

    return out;
}

fn fromLinear(linearRGB: float4) -> float4
{
    let cutoff: vec4<f32> = vec4<f32>(linearRGB < float4(0.0031308));
    let higher: vec4<f32> = float4(1.055)*pow(linearRGB, float4(1.0/2.4)) - float4(0.055);
    let lower: vec4<f32> = linearRGB * float4(12.92);

    return mix(higher, lower, cutoff);
}

// Converts a color from sRGB gamma to linear light gamma
fn toLinear(sRGB: float4) -> float4
{
    let cutoff = vec4<f32>(sRGB < float4(0.04045));
    let higher = pow((sRGB + float4(0.055))/float4(1.055), float4(2.4));
    let lower = sRGB/float4(12.92);

    return mix(higher, lower, cutoff);
}


struct FragmentInput {
    [[location(0)]] uv: vec2<f32>;
    [[location(1)]] pos_scale: vec4<f32>;
    [[location(2)]] color: vec4<f32>;
};

fn cla(mi: f32, ma: f32, x: f32) -> f32 {
  if (x<mi) {
    return mi;
  }
  if (x>ma) {
    return ma;
  }
  return x;
}

fn sdSegment(p: vec2<f32>, a: vec2<f32>, b: vec2<f32>) -> f32 {
  let pa = p - a;
  let ba = b - a;
  let h = clamp(dot(pa, ba) / dot(ba, ba), 0., 1.);
  return length(pa - ba * h);
}


fn sdCircle(p: vec2<f32>, c: vec2<f32>, r: f32) -> f32 {
  let d = length(p - c);
  return d - r;
}


fn sdRoundedBox(p: vec2<f32>, b: vec2<f32>, r: vec4<f32>) -> f32 {
  var x = r.x;
  var y = r.y;
  x = select(r.z, r.x, p.x > 0.);
  y = select(r.w, r.y, p.x > 0.);
  x  = select(y, x, p.y > 0.);
  let q = abs(p) - b + x;
  return min(max(q.x, q.y), 0.) + length(max(q, vec2<f32>(0.))) - x;
}

// fn sdBox(p: vec2<f32>, b: vec2<f32>) -> f32 {
//   let d = (abs(p) - b) ;
//   return length(max(d, vec2<f32>(0.))) + min(max(d.x, d.y), 0.);
// }


struct SegmentUniform {
    segmemt_size: f32;
    hole_size: f32;
    zoom: f32;
    // point_type: i32;
    quad_size: f32;
    contour: f32;
    inner_canvas_size_in_pixels: float2;
    canvas_position_in_pixels: float2;
    color: float4;
    segment_point_color: float4;
    
};

[[group(2), binding(0)]]
var<uniform> uni: SegmentUniform;


[[stage(fragment)]]
fn fragment(in: FragmentInput) -> [[location(0)]] vec4<f32> {
 
    let width = 0.041 ;
    let zoom = uni.zoom;

    var w = width * zoom  ;
    var solid = width * zoom  ;


    var out_col = uni.color;

    var uv = in.uv - float2(0.5,0.5);

    var uv_in_pixels = float2(-uv.x, uv.y) * uni.quad_size - in.pos_scale.xy;

    let segmemt_size = uni.segmemt_size;

    // let point_type = i32(uni.point_type);
    // let point_type = 6;

    // change the aliasing as a function of the zoom
    var circ_zoom = zoom;

    if (zoom  >.0) {
      circ_zoom =  pow(zoom, 0.05);
    }

    if (zoom < 1.0) {
      circ_zoom =  sqrt(sqrt(zoom));
    }



    let black = float4(0.0, 0.0, 0.0, 1.0);



    // mask with the canvas
    let r = 0.02 * uni.inner_canvas_size_in_pixels.x;
    let d = sdRoundedBox(
        // in.uv - bez_uni.canvas_position_in_pixels , 
        in.uv + uni.canvas_position_in_pixels,
        uni.inner_canvas_size_in_pixels / 2.0 - 1.0, float4(r,r,r,r)
    );

    let s = smoothStep(-2.0, 0.0, d );
    out_col = mix(out_col, float4(0.0,0.3,0.3,0.0) ,  s) ;


    return out_col;

}