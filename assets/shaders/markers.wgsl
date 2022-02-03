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

fn sdRhombus(p: vec2<f32>, b: vec2<f32>) -> f32 {
  let q = abs(p);
  let qb = dot(q, vec2<f32>(b.x, -b.y));
  let bb = dot(b, vec2<f32>(b.x, -b.y));
  let h = clamp((-2. * qb + bb) / dot(b, b), -1., 1.);
  let d = length(q - 0.5 * b * vec2<f32>(1. - h, 1. + h));
  return d * sign(q.x * b.y + q.y * b.x - b.x * b.y);
}

fn sdTriangleIsosceles(p: vec2<f32>, c: vec2<f32>) -> f32 {
  let q = vec2<f32>(abs(p.x), p.y);
  let a = q - c * clamp(dot(q, c) / dot(c, c), 0., 1.);
  let b = q - c * vec2<f32>(clamp(q.x / c.x, 0., 1.), 1.);
  let s = -sign(c.y);
  let d = min(vec2<f32>(dot(a, a), s * (q.x * c.y - q.y * c.x)), vec2<f32>(dot(b, b), s * (q.y - c.y)));
  return -sqrt(d.x) * sign(d.y);
}

fn sdStar(p: vec2<f32>, r: f32, n: u32, m: f32) ->f32 {
  let an = 3.141593 / f32(n);
  let en = 3.141593 / m;
  let acs = vec2<f32>(cos(an), sin(an));
  let ecs = vec2<f32>(cos(en), sin(en));
  let bn = (atan2(abs(p.x), p.y) % (2. * an)) - an;
  var q: vec2<f32> = length(p) * vec2<f32>(cos(bn), abs(sin(bn)));
  q = q - r * acs;
  q = q + ecs * clamp(-dot(q, ecs), 0., r * acs.y / ecs.y);
  return length(q) * sign(q.x);
}

fn sdHeart(p: vec2<f32>) -> f32 {
  let q = vec2<f32>(abs(p.x), p.y);
  let w = q - vec2<f32>(0.25, 0.75);
  if (q.x + q.y > 1.0) { return sqrt(dot(w, w)) - sqrt(2.) / 4.; }
  let u = q - vec2<f32>(0., 1.0);
  let v = q - 0.5 * max(q.x + q.y, 0.);
  return sqrt(min(dot(u, u), dot(v, v))) * sign(q.x - q.y);
}

fn sdMoon(p: vec2<f32>, d: f32, ra: f32, rb: f32) -> f32 {
  let q = vec2<f32>(p.x, abs(p.y));
  let a = (ra * ra - rb * rb + d * d) / (2. * d);
  let b = sqrt(max(ra * ra - a * a, 0.));
  if (d * (q.x * b - q.y * a) > d * d * max(b - q.y, 0.)) { return length(q-vec2<f32>(a, b)); }
  return max((length(q) - ra), -(length(q - vec2<f32>(d, 0.)) - rb));
}

fn sdCross(p: vec2<f32>, b: vec2<f32>) -> f32 {
  var q: vec2<f32> = abs(p);
  q = select(q.xy, q.yx, q.y > q.x);
  let t = q - b;
  let k = max(t.y, t.x);
  let w = select(vec2<f32>(b.y - q.x, -k), t, k > 0.);
  return sign(k) * length(max(w, vec2<f32>(0.)));
}

fn sdRoundedX(p: vec2<f32>, w: f32, r: f32) -> f32 {
  let q = abs(p);
  return length(q - min(q.x + q.y, w) * 0.5) - r;
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

fn sdBox(p: vec2<f32>, b: vec2<f32>) -> f32 {
  let d = (abs(p) - b) ;
  return length(max(d, vec2<f32>(0.))) + min(max(d.x, d.y), 0.);
}


struct MarkersUniform {
    marker_size: f32;
    hole_size: f32;
    zoom: f32;
    point_type: u32;
};

[[group(2), binding(0)]]
var<uniform> custom_uniform: MarkersUniform;


[[stage(fragment)]]
fn fragment(in: FragmentInput) -> [[location(0)]] vec4<f32> {
    // let width = 0.003 ;
    let width = 0.041 ;
    let zoom = custom_uniform.zoom;
    //  let zoom = 1.0;
    var w = width * zoom  ;
    var solid = width * zoom * 1.0 ;

    // var out_col = float4(0.5, 0.5, 0.0, 1.0);
    var out_col = in.color;

    var uv = in.uv - float2(0.5,0.5);

    let marker_size = custom_uniform.marker_size;

    let point_type = i32(custom_uniform.point_type);
    // let point_type = 8;

    // change the aliasing as a function of the zoom
    var circ_zoom = zoom;

    if (zoom  >.0) {
      circ_zoom =  pow(zoom, 0.05);
    }

    if (zoom < 1.0) {
      circ_zoom =  sqrt(sqrt(zoom));
    }

    // square -> 0
    // heart -> 1
    // rhombus -> 2
    // triangle -> 3
    // star -> 4
    // moon -> 5
    // cross -> 6
    // x -> 7
    // circle -> 8


    // square -> 0
    if (point_type == 0) {

      let r = cla(0.01, 0.3, 0.2 * custom_uniform.marker_size);
      let side_size = cla(0.1, 0.45, 0.4 * custom_uniform.marker_size);

      let d = sdRoundedBox(uv, float2(side_size, side_size), float4(r, r, r, r));  
      let s = smoothStep(solid*0.0 , solid * 0.0 + w , d    );

      out_col.a = out_col.a * (1.0 - s);


    // heart -> 1
    } else if (point_type == 1) {

        let heart_size = cla(0.2, 0.6, 0.15 * custom_uniform.marker_size); 
        let w_heart = w / heart_size;

        let d = sdHeart((uv - float2(0.0, -heart_size * 0.9 + 0.15)) / heart_size + float2(0.0, 0.2));  
        let s = smoothStep(0.0 , w_heart , d    );

        out_col.a = out_col.a * (1.0 - s);

    // rhombus -> 2
    } else if (point_type == 2) {

        let size = cla(0.1, 0.4, 0.3 * custom_uniform.marker_size); 

        let d = sdRhombus(uv, float2(size * 1.2, size * 0.8));
        let s = smoothStep(0.0  , w / circ_zoom, d   );

        out_col.a = out_col.a *(1.0 - s);

    // triangle -> 3
    } else if (point_type == 3) {

        uv.y = -uv.y;

        let size = cla(0.13, 0.5, 0.3 * custom_uniform.marker_size); 
        
        let d = sdTriangleIsosceles(uv - float2(0.0, -size * 0.5) , float2(size * 0.7, size));
        let s = smoothStep(0.0  , 0.0  + w / circ_zoom, d   );

        out_col.a = out_col.a * (1.0 - s);
    
    // star -> 4
    } else if (point_type == 4) {

        let star_size = cla(0.05, 0.2, 0.1 * custom_uniform.marker_size); 

        let d = sdStar(uv, star_size, u32(5), 0.35);
        let s = smoothStep(0.0  , 0.0  + w / circ_zoom, d   );

        out_col.a = out_col.a * (1.0 - s);

        let sb = smoothStep(1.0  , 0.0  + w / circ_zoom, d   );



    // moon -> 5
    } else if (point_type == 5) {

        let moon_size = cla(0.3, 1.3,  custom_uniform.marker_size); 

        let d = sdMoon(uv - float2(0.05 * (1.0 + moon_size * 0.7 ), 0.0), 0.3 * moon_size, 0.35 * moon_size, 0.35 * moon_size);
        let s = smoothStep(0.0  , 0.0  + w / circ_zoom, d   );
       
        out_col.a = out_col.a *  (1.0 - s);

    // cross -> 6
    } else if (point_type == 6) {

        let cross_size = cla(0.1, 0.4,  0.25  *custom_uniform.marker_size); 

        let d = sdCross(uv, float2(cross_size, cross_size / 3.0));
        let s = smoothStep(0.0  , 0.0  + w / circ_zoom, d   );


        out_col.a = out_col.a * (1.0 - s);
        

    // x -> 7
    } else if (point_type == 7) {
         let ex_size = cla(0.15, 0.6,  0.3 * custom_uniform.marker_size); 

        let start_size = 0.1;
        let d = sdRoundedX(uv, ex_size, ex_size / 6.0);
        let s = smoothStep(0.0  ,  w / circ_zoom, d   );

        out_col.a = out_col.a * (1.0 - s);

    // circles -> 8
    } else if (point_type == 8) {

      let circle_size = cla(0.04, 0.45,  0.25 * custom_uniform.marker_size); 

      let r =  circle_size;
      let d = sdCircle(uv, float2(0.0,0.0), circle_size);  
      let s = smoothStep(0.0 ,  w , d    );

      out_col.a = out_col.a * (1.0 - s) ;
    }


    // tiny circle at exact location of data point
    let dc = sdCircle(uv  , float2(0.0, 0.0 ), 0.025 * custom_uniform.hole_size );
    let sc = smoothStep(0.0, w / circ_zoom * custom_uniform.hole_size , dc  );
    out_col = mix(out_col, float4(0.0,0.3,0.3,1.0) , 1.0 - sc) ;

    return out_col;

}