#import bevy_sprite::mesh2d_view_bind_group
[[group(0), binding(0)]]
var<uniform> view: View;

#import bevy_sprite::mesh2d_struct
[[group(1), binding(0)]]
var<uniform> mesh: Mesh2d;

type float4 = vec4<f32>;
type float2 = vec2<f32>;

struct BezierCurveUniform {
    mech: f32;
    zoom: f32;
    inner_canvas_size_in_pixels: float2;
    canvas_position_in_pixels: float2;
    color: float4;
    size: f32;
    dummy: f32;
    style: i32;
};

[[group(2), binding(0)]]
var<uniform> bez_uni: BezierCurveUniform;

// The structure of the vertex buffer is as specified in `specialize()`
struct Vertex {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] ends: vec4<f32>;
    [[location(2)]] uv: vec2<f32>;
    [[location(3)]] control: vec4<f32>;
};
struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] ends: vec4<f32>;
    [[location(1)]] uv: vec2<f32>;
    [[location(2)]] control: vec4<f32>;
};
/// Entry point for the vertex shader
[[stage(vertex)]]
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    // Project the world position of the mesh into screen position
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

// Converts a color from sRGB gamma to linear light gamma (unused?)
fn toLinear(sRGB: float4) -> float4
{
    let cutoff = vec4<f32>(sRGB < float4(0.04045));
    let higher = pow((sRGB + float4(0.055))/float4(1.055), float4(2.4));
    let lower = sRGB/float4(12.92);

    return mix(higher, lower, cutoff);
}


struct FragmentInput {
    [[location(0)]] ends: vec4<f32>;
    [[location(1)]] uv: vec2<f32>;
    [[location(2)]] control: vec4<f32>;
};





fn sdBezier(p: vec2<f32>, A: vec2<f32>, B: vec2<f32>, C: vec2<f32>) -> vec2<f32> {
  let a = B - A;
  let b = A - 2. * B + C;
  let c = a * 2.;
  let d = A - p;
  let kk = 1. / dot(b, b);
  let kx = kk * dot(a, b);
  let ky = kk * (2. * dot(a, a) + dot(d, b)) / 3.;
  let kz = kk * dot(d, a);

  let p1 = ky - kx * kx;
  let p3 = p1 * p1 * p1;
  let q = kx * (2.0 * kx * kx - 3.0 * ky) + kz;
  var h: f32 = q * q + 4. * p3;

  var res: vec2<f32>;
  if (h >= 0.) {
    h = sqrt(h);
    let x = (vec2<f32>(h, -h) - q) / 2.;
    let uv = sign(x) * pow(abs(x), vec2<f32>(1. / 3.));
    let t = clamp(uv.x + uv.y - kx, 0., 1.);
    let f = d + (c + b * t) * t;
    res = vec2<f32>(dot(f, f), t);
  } else {
    let z = sqrt(-p1);
    let v = acos(q / (p1 * z * 2.)) / 3.;
    let m = cos(v);
    let n = sin(v) * 1.732050808;
    let t = clamp(vec2<f32>(m + m, -n - m) * z - kx, vec2<f32>(0.0), vec2<f32>(1.0));
    let f = d + (c + b * t.x) * t.x;
    var dis: f32 = dot(f, f);
    res = vec2<f32>(dis, t.x);

    let g = d + (c + b * t.y) * t.y;
    dis = dot(g, g);
    res = select(res, vec2<f32>(dis, t.y), dis < res.x);
  }
  res.x = sqrt(res.x);
  return res;
}

fn dot2( v: float2 ) -> f32 { return dot(v,v); }
fn cro(  a: float2, b: float2 ) -> f32 { return a.x*b.y - a.y*b.x; }

fn sdBezier2( p: float2,  v0q: float2,  v1q: float2,  v2q: float2 ) -> float2
{
	let i = v0q - v2q;
    let j = v2q - v1q;
    let k = v1q - v0q;
    let w = j-k;

    var v0 = v0q;
    var v1 = v1q;
    var v2 = v2q;

	v0 = v0 - p; 
    v1 = v1 - p; 
    v2 = v2 - p;
    
	let x = cro(v0, v2);
    let y = cro(v1, v0);
    let z = cro(v2, v1);

	let s = 2.0*(y*j+z*k)-x*i;

    let r =  (y*z-x*x*0.25)/dot2(s);
    let t = clamp( (0.5*x+y+r*dot(s,w))/(x+y+z),0.0,1.0);
    
	return float2(length( v0+t*(k+k+t*w) ), 0.0) ;
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

fn sdPie(p: vec2<f32>, sc: vec2<f32>, r: f32) -> f32 {
  let q = vec2<f32>(abs(p.x), p.y);
  let l = length(q) - r;
  let m = length(q - sc * clamp(dot(q, sc), 0., r));
  return max(l, m * sign(sc.y * q.x - sc.x * q.y));
}

fn tips(uv: float2, m_in: float4, dy: float2, solid: f32, w: f32 ) -> float4 {
    var m = m_in;
    let theta = atan2(dy.y, dy.x) + 3.1415 / 2.0;
    let ma = mat2x2<f32>(float2(cos(theta), -sin(theta)), float2(sin(theta), cos(theta)) );
    let uvrot = ma * uv ;
    let angle = 3.1415 / 2.0;
    let pies = sdPie(uvrot , vec2<f32>(sin(angle), cos(angle)) , solid / 0.9);
    let sp = smoothStep(0.0 , w  , pies);
    // let mp = mix(m, vec4<f32>(0.0, 1.0, 0.0, 1.0),   1.0-sp);
    m.a = m.a * (sp);
    return m;
}

[[stage(fragment)]]
fn fragment(in: FragmentInput) -> [[location(0)]] vec4<f32> {
    let width = bez_uni.size / 1.0;
    // let w = 1.0 + width * bez_uni.zoom  * 1.0;
    // let solid = width * bez_uni.zoom ;
    let w = 1.0 + width  * 1.0;
    let solid = width ;

    // var out_col = float4(0.2, 0.3, 0.8, 1.00);
    var out_col = bez_uni.color;

    var uv = in.uv;

    var p0 = in.ends.xy;
    var p1 = in.ends.zw;

    var control = in.control.xy;
    let is_last = in.control.w;

    control.x = clamp(p0.x, p1.x, control.x);

    if (control.x < min(p1.x, p0.x)) {
        control.x = min(p1.x, p0.x);
    }

    if (control.x > max(p1.x, p0.x)) {
        control.x = max(p1.x, p0.x);
    }
    

    let d = sdBezier(uv, p0, control  , p1);
    let s = smoothStep(0.0 + solid, w  + solid , d.x);

    // mechanical look
    if (bez_uni.mech > 0.5) {
    // if (false) {
        let c0 = sdCircle(in.uv, p0, w);
        let sc0 = smoothStep(0.0 + solid, w + solid , c0);

        let solid_c = solid / 3.0;
        let w_c = w / 2.0;

        let c1 = sdCircle(in.uv, p1, 0.2 );
        let sc1 = smoothStep(0.0 + solid_c , (w_c + solid_c)  , abs(c1));
        
        out_col.a = out_col.a * (1.0 -s )   * ( sc1) * sc0;

    } else {
        // smooth look
        out_col.a = out_col.a * (1.0 -s );

        // correcting for artifacts at the intersection of two bezier end-points
        // by displacing a circle in the direction of the derivative.
        // Thw artifact variable was chosen experimentally for sizes of 0.5, 1.0 and 2.0;

        if (is_last < 0.5) { 
            var artifact = 1.0;
            if (bez_uni.size > 1.9) {
                artifact = 0.78;
            }
            if (bez_uni.size < 0.6) {
                artifact = 1.75;
            }
            let dy = normalize(p1 - control);
            let dc = sdCircle(in.uv, p1 + dy *  width * 2.12 * artifact , w);
            // let dc = sdCircle(in.uv, p1 + dy *  width * bez_uni.dummy * 2.12 * artifact , w);

           let sc = smoothStep(solid, w * 0.9 *0.5 + solid , dc);
           
            // let dy = normalize(p1 - control);
            // let dc = sdCircle(in.uv, p1 + dy * 4.0 , 0.0);
            // let sc = smoothStep(solid, solid + w * 1.0 , dc);


            out_col.a = out_col.a  - ( 1.0 - sc ) ;
            
            let div = 5.0;
            let dy1 = normalize(p1 - control);
            let dy0 = normalize(p0 - control);
            let mu = 0.6;

        // out_col = tips(in.uv- p0 - dy0 * mu, out_col, control - p0, solid*2.5, w / div  );
        // out_col = tips(in.uv- p1 - dy1 * mu, out_col, control - p1, solid*2.5, w / div  );
        }

    }

    // let dc = sdCircle(in.uv, p1 + float2(0.0, 6.0)  , 1.2);
    // let sc = smoothStep(solid, solid + w  , dc);
    // out_col = mix(out_col, vec4<f32>(0.5, 0.0, 0.0, 1.0), 1.0 -  sc);

    // let dc = sdCircle(in.uv, control.xy  , 1.2);
    // let sc = smoothStep(solid, solid + w  , dc);
    // out_col = mix(out_col, vec4<f32>(0.5, 0.0, 0.5, 1.0), 1.0 -  sc);


    // mask with the canvas
    let r = 0.02 * bez_uni.inner_canvas_size_in_pixels.x;
    let d = sdRoundedBox(
        // in.uv - bez_uni.canvas_position_in_pixels , 
        in.uv,
        bez_uni.inner_canvas_size_in_pixels / 2.0 - 1.0, float4(r,r,r,r)
    );

    let s = smoothStep(-2.0, 0.0, d );
    out_col = mix(out_col, float4(0.0,0.3,0.3,0.0) ,  s) ;

    // out_col =  float4(0.0,0.3,0.3,1.0);


    return out_col;
}