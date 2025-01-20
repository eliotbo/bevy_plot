// Import the standard 2d mesh uniforms and set their bind groups
// #import bevy_sprite::mesh2d_view_bind_group
// #import bevy_sprite::mesh2d_types
// #import bevy_sprite::mesh2d_view_types

#import bevy_sprite::mesh2d_bindings
#import bevy_sprite::mesh2d_view_bindings


struct SegmentUniform {
    color: vec4<f32>,
    mech: f32,
    segment_thickness: f32,
    hole_size: f32,
    zoom: f32,
    inner_canvas_size_in_pixels: vec2<f32>,
    canvas_position_in_pixels: vec2<f32>,  
};

// @group(0) @binding(0)
// var<uniform> view: View;

@group(1) @binding(0)
var<uniform> uni: SegmentUniform;

// @group(2) @binding(0)
// var<uniform> mesh: Mesh2d;



// The structure of the vertex buffer is as specified in `specialize()`
struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1)  ends: vec4<f32>,
    @location(2)  uv: vec2<f32>,
    @location(3)  control: vec4<f32>,
};

struct VertexOutput {
    // The vertex shader must set the on-screen position of the vertex
    @builtin(position) clip_position: vec4<f32>,
    // We pass the vertex color to the framgent shader in location 0
    @location(0)  ends: vec4<f32>,
    @location(1)  uv: vec2<f32>,
    @location(2)  control: vec4<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {

    var out: VertexOutput;

    out.clip_position = view.view_proj * mesh.model * vec4<f32>(vertex.position, 1.0);

    out.ends = vertex.ends;
    out.uv = vertex.uv;
    out.control = vertex.control;

    return out;
}

fn fromLinear(linearRGB: vec4<f32>) -> vec4<f32> {
    let cutoff: vec4<f32> = vec4<f32>(linearRGB < vec4<f32>(0.0031308));
    let higher: vec4<f32> = vec4<f32>(1.055) * pow(linearRGB, vec4<f32>(1.0 / 2.4)) - vec4<f32>(0.055);
    let lower: vec4<f32> = linearRGB * vec4<f32>(12.92);

    return mix(higher, lower, cutoff);
}

// Converts a color from sRGB gamma to linear light gamma
fn toLinear(sRGB: vec4<f32>) -> vec4<f32> {
    let cutoff = vec4<f32>(sRGB < vec4<f32>(0.04045));
    let higher = pow((sRGB + vec4<f32>(0.055)) / vec4<f32>(1.055), vec4<f32>(2.4));
    let lower = sRGB / vec4<f32>(12.92);

    return mix(higher, lower, cutoff);
}


struct FragmentInput {
    @location(0)  ends: vec4<f32>,
    @location(1)  uv: vec2<f32>,
    @location(2)  control: vec4<f32>,
};

fn cla(mi: f32, ma: f32, x: f32) -> f32 {
    if x < mi {
        return mi;
    }
    if x > ma {
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
    let m = (p1.y - p0.y) / (p1.x - p0.x);
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
    x = select(y, x, p.y > 0.);
    let q = abs(p) - b + x;
    return min(max(q.x, q.y), 0.) + length(max(q, vec2<f32>(0.))) - x;
}

// fn sdBox(p: vec2<f32>, b: vec2<f32>) -> f32 {
//   let d = (abs(p) - b) ;
//   return length(max(d, vec2<f32>(0.))) + min(max(d.x, d.y), 0.);
// }




@fragment
fn fragment(in: FragmentInput) -> @location(0)  vec4<f32> {
    let width = 1.0 ;

    let zoom = uni.zoom;
    var w = width * zoom * sqrt(uni.segment_thickness * 4.5) ;

    var solid = width * zoom * uni.segment_thickness ;

    var out_col = uni.color;

    let y0 = in.ends.xy;
    let y1 = in.ends.zw;

    let dy = normalize(y1 - y0);
    let q0 = y0 - dy * 10.0;
    let q1 = y1 + dy * 10.0;


    // change the aliasing as a function of the zoom
    var circ_zoom = zoom;

    if zoom > .0 {
        circ_zoom = pow(zoom, 0.05);
    }

    if zoom < 1.0 {
        circ_zoom = sqrt(sqrt(zoom));
    }


    let d = sdSegment(in.uv, y0, y1) ;
    let s = smoothstep(solid, solid + w, d);
    out_col = out_col * (1.0 - s);


    // mechanical look
    if uni.mech > 0.5 {
        let c0 = sdCircle(in.uv, y0, w);
        let sc0 = smoothstep(0.0 + solid, w + solid, c0);

        let solid_c = solid / 3.0;
        let w_c = w / 2.0;

        let c1 = sdCircle(in.uv, y1, 0.2);
        let sc1 = smoothstep(0.0 + solid_c, (w_c + solid_c), abs(c1));

        out_col.a = out_col.a * (1.0 - s) * (sc1) * sc0;
    }


    // mask with the canvas
    let r = 0.02 * uni.inner_canvas_size_in_pixels.x;
    let d = sdRoundedBox(
        // in.uv - bez_uni.canvas_position_in_pixels , 
        in.uv + uni.canvas_position_in_pixels * 0.0,
        uni.inner_canvas_size_in_pixels / 2.0 - 1.0,
        vec4<f32>(r, r, r, r)
    );

    let s = smoothstep(-2.0, 0.0, d);
    out_col = mix(out_col, vec4<f32>(out_col.x, out_col.y, out_col.z, 0.0), s) ;


    return out_col;

    // return vec4<f32>(1.0,0.0,0.0,1.0);
}