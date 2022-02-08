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
    [[location(1)]] ends: vec4<f32>;
    [[location(2)]] uv: vec2<f32>;
    [[location(3)]] control: vec4<f32>;
};

struct VertexOutput {
    // The vertex shader must set the on-screen position of the vertex
    [[builtin(position)]] clip_position: vec4<f32>;
    // We pass the vertex color to the framgent shader in location 0
    [[location(0)]] ends: vec4<f32>;
    [[location(1)]] uv: vec2<f32>;
    [[location(2)]] control: vec4<f32>;
};

[[stage(vertex)]]
fn vertex(vertex: Vertex) -> VertexOutput {

    var out: VertexOutput;

    out.clip_position = view.view_proj * mesh.model * vec4<f32>(vertex.position, 1.0);

    out.ends = vertex.ends;
    out.uv = vertex.uv;
    out.control = vertex.control;



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
    // [[location(0)]] uv: vec2<f32>;
    // [[location(1)]] pos_scale: vec4<f32>;
    // [[location(2)]] color: vec4<f32>;
    [[location(0)]] ends: vec4<f32>;
    [[location(1)]] uv: vec2<f32>;
    [[location(2)]] control: vec4<f32>;
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


fn sdLine(uv: vec2<f32>, p0: vec2<f32>, p1: vec2<f32>) -> f32 {
  let m =  (p1.y - p0.y) / (p1.x - p0.x);
  let b = p0.y - m * p0.x;
  let biga = m;
  let bigc = b;

  let d = abs(-m * uv.x + uv.y - b) / sqrt(m * m + 1.0);
  return d;
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
    mech: f32;
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
  
    let width = 1.0 ;
    // let segmemt_size = uni.segmemt_size * width;

    let zoom = uni.zoom;
    var w = width * zoom * sqrt(uni.segmemt_size * 4.5) ;
  
    var solid = width * zoom * uni.segmemt_size ;


    var out_col = uni.color;



    let y0 = in.ends.xy;
    let y1 = in.ends.zw;

    let dy = normalize(y1 - y0);
    let q0 = y0 - dy  * 10.0;
    let q1 = y1 + dy  * 10.0;



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

    // let d = sdSegment(in.uv, q0, q1) ;
    let d = sdSegment(in.uv, y0, y1) ;

    
    let s = smoothStep(solid, solid + w, d);
    out_col = out_col * (1.0 - s);


  // let black = float4(0.0, 0.0, 0.0, 1.0);



    // let t0 = y0 - dy  *  uni.segmemt_size;
    // let t1 = y1 + dy  *  uni.segmemt_size;

    // let d = sdSegment(in.uv, t0, t1) ;

    // let s = smoothStep(solid *0.95, solid *0.98, d);
    // out_col = mix(out_col, uni.color, (1.0 - s));


    

      // mechanical look
    if (uni.mech > 0.5) {
        let c0 = sdCircle(in.uv, y0, w);
        let sc0 = smoothStep(0.0 + solid, w + solid , c0);

        let solid_c = solid / 3.0;
        let w_c = w / 2.0;

        let c1 = sdCircle(in.uv, y1, 0.2 );
        let sc1 = smoothStep(0.0 + solid_c , (w_c + solid_c)  , abs(c1));
        
        out_col.a = out_col.a * (1.0 -s )   * ( sc1) * sc0;
    }



    // mask with the canvas
    let r = 0.02 * uni.inner_canvas_size_in_pixels.x;
    let d = sdRoundedBox(
        // in.uv - bez_uni.canvas_position_in_pixels , 
        in.uv + uni.canvas_position_in_pixels * 0.0,
        uni.inner_canvas_size_in_pixels / 2.0 - 1.0, float4(r,r,r,r)
    );

    let s = smoothStep(-2.0, 0.0, d );
    out_col = mix(out_col, float4(out_col.x,out_col.y,out_col.z,0.0) ,  s) ;


    return out_col;

    // return float4(1.0,0.0,0.0,1.0);

}