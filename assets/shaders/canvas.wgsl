
// #import bevy_sprite::mesh2d_types
// #import bevy_sprite::mesh2d_view_types

#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_render::view::View

// struct Vertex {
//     @location(0) position: vec3<f32>,
//     @location(1) normal: vec3<f32>,
//     @location(2) uv: vec2<f32>,
// #ifdef VERTEX_TANGENTS
//     @location(3) tangent: vec4<f32>,
// #endif
// };

// struct VertexOutput {
//     @builtin(position) clip_position: vec4<f32>,
//     @location(0) world_position: vec4<f32>,
//     @location(1) world_normal: vec3<f32>,
//     @location(2) uv: vec2<f32>,
// #ifdef VERTEX_TANGENTS
//     @location(3) world_tangent: vec4<f32>,
// #endif
// };


struct GraphEditShader {
    mouse_pos: vec2<f32>,
    tick_period: vec2<f32>,
    bound_up: vec2<f32>, // TODO
    bound_lo: vec2<f32>,
    time: f32,
    zoom: f32,
    size: vec2<f32>,
    outer_border: vec2<f32>,
    // position: vec2<f32>,
    show_target: f32,
    hide_contour: f32,
    target_pos: vec2<f32>,
    background_color1: vec4<f32>,
    background_color2: vec4<f32>,
    target_color: vec4<f32>,
    show_grid: f32,
    show_axes: f32,
};

struct GraphPosition {
    position: vec2<f32>,
};


@group(0) @binding(0)
var<uniform> view: View;

@group(2) @binding(0)
var<uniform> mate: GraphEditShader;

@group(2) @binding(1) var<uniform> graph_position: GraphPosition;

// @group(2) @binding(0)
// var<uniform> mesh: Mesh2d;

// struct VertexOutput {
//     @builtin(position) clip_position: vec4<f32>,
//     @location(0) uv: vec2<f32>,
// };

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
#ifdef VERTEX_TANGENTS
    @location(3) tangent: vec4<f32>,
#endif
};


struct Segment {
    start: vec2<f32>,
    end: vec2<f32>,
};

struct Interval {
    start: vec2<f32>,
    end: vec2<f32>,
    control: vec4<f32>,
};

struct GraphSize {
    size: vec2<f32>,
    outer_border: vec2<f32>,
};



var<private>  solid: f32 = 0.001;  
var<private>  smooth_dist2: f32 = 0.003;
var<private>  point2_radius: f32 = 0.03;
var<private>  out_of_bounds: f32 = 100000.0;
var<private>  bluish : vec4<f32> = vec4<f32>(0.13, 0.28, 0.86, 1.0);
var<private>  num_segments: i32 = 256;



struct Globals {
    time: f32,
    zoom: f32,
    dum1: f32,
    dum2: f32,
};




@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    let world_position = vec4<f32>(vertex.position, 1.0);
    let view_proj = view.clip_from_view * view.view_from_world;
    out.position = view_proj * world_position;
    out.world_position = world_position;
    out.world_normal = vertex.normal;
    out.uv = vertex.uv;
    
#ifdef VERTEX_TANGENTS
    out.world_tangent = vec4<f32>(vertex.tangent.xyz, vertex.tangent.w);
#endif

    return out;
}

// struct FragmentInput {
//     @builtin(front_facing) is_front: bool,
//     @location(0) world_position: vec4<f32>,
//     @location(1) world_normal: vec3<f32>,
//     @location(2) uv: vec2<f32>,
// #ifdef VERTEX_TANGENTS
//     @location(3) world_tangent: vec4<f32>,
// #endif
// };

fn from_pix_to_local(uv_orig: vec2<f32>) -> vec2<f32> {

    var uv = (uv_orig - graph_position.position) ;

    let x_max = mate.bound_up.x;
    let y_max = mate.bound_up.y;

    let x_min = mate.bound_lo.x;
    let y_min = mate.bound_lo.y;

    let x_range = x_max - x_min;
    let y_range = y_max - y_min;

    uv.x = uv.x * (1.0 + mate.outer_border.x) / mate.size.x ;
    uv.x = uv.x * x_range ;

    uv.y = uv.y * (1.0 + mate.outer_border.y) / mate.size.y;
    uv.y = uv.y * y_range;

    let current_zero_pos = vec2<f32>(x_range / 2.0 + x_min, y_range / 2.0 + y_min);
    let uv_local = uv + current_zero_pos;

    return uv_local;
}

// fn from_pix_to_local(uv_orig: vec2<f32>) -> vec2<f32> {

//     // var uv = (uv_orig - graph_position.position) ;
//     var uv = (uv_orig - vec2<f32>(0.5, 0.5)) ;

//     let x_max = mate.bound_up.x;
//     let y_max = mate.bound_up.y;

//     let x_min = mate.bound_lo.x;
//     let y_min = mate.bound_lo.y;

//     let x_range = x_max - x_min;
//     let y_range = y_max - y_min;

//     uv.x = uv.x * (1.0 + mate.outer_border.x) ;
//     uv.x = uv.x * x_range ;

//     uv.y = uv.y * (1.0 + mate.outer_border.y);
//     uv.y = -uv.y * y_range;

//     let current_zero_pos = vec2<f32>(x_range / 2.0 + x_min, y_range / 2.0 + y_min);
//     let uv_local = uv + current_zero_pos;

//     return uv_local;
// }

fn from_local_to_pixels(uv_local: vec2<f32>) -> vec2<f32> {
    var uv = uv_local;

    uv.x = uv.x * mate.size.x / (1.0 + mate.outer_border.x) ;
    uv.x = uv.x / (mate.bound_up.x - mate.bound_lo.x);

    uv.y = uv.y * mate.size.y / (1.0 + mate.outer_border.y) ;
    uv.y = uv.y / (mate.bound_up.y - mate.bound_lo.y);



    return uv;
}

fn from_local_to_pixels3_inv(uv_local: vec2<f32>) -> vec2<f32> {
    var uv = uv_local;

    uv.x = (uv.x - 0.5) * mate.size.x ;
    // uv.x = uv.x / (mate.bound_up.x - mate.bound_lo.x);

    uv.y = (uv.y - 0.5) * mate.size.y ;
    // uv.y = uv.y / (mate.bound_up.y - mate.bound_lo.y);

    uv = uv + graph_position.position / 2.0;



    return uv;
}


fn from_local_to_pixels3(uv_local: vec2<f32>) -> vec2<f32> {
    var uv = uv_local;

    uv.x = (uv.x - 0.5) * mate.size.x ;
    // uv.x = uv.x / (mate.bound_up.x - mate.bound_lo.x);

    uv.y = -(uv.y - 0.5) * mate.size.y ;
    // uv.y = uv.y / (mate.bound_up.y - mate.bound_lo.y);

    uv = uv + graph_position.position / 2.0;



    return uv;
}

// fn from_uv_to_pixels2(uv_local: vec2<f32>) -> vec2<f32> {
//     var uv = uv_local;

//     let x_max = mate.bound_up.x;
//     let y_max = mate.bound_up.y;

//     let x_min = mate.bound_lo.x;
//     let y_min = mate.bound_lo.y;

//     let x_range = x_max - x_min;
//     let y_range = y_max - y_min;

//     let current_zero_pos = vec2<f32>(x_range / 2.0 + x_min, y_range / 2.0 + y_min);
//     uv = uv - current_zero_pos;

//     uv.y = uv.y / y_range;
//     uv.y = uv.y / (1.0 + mate.outer_border.y) * mate.size.y;

//     uv.x = uv.x / x_range;
//     uv.x = uv.x / (1.0 + mate.outer_border.x) * mate.size.x;

//     return uv;
// }

// // There are currently no function for x % 2 in wgpu
// fn even(uv: f32) -> f32 {
//     // var tempo: f32 = 0.0;
//     // let whatever = modf(uv + 1.0, &tempo);
//     // var temp2 = 0.;
//     // let frac = modf(tempo / 2.0, &temp2);

//     // if abs(frac) < 0.001 {
//     //     return 1.0;
//     // } else {
//     //     return 0.0;
//     // }

//     return 1.0;
// }

fn even(uv: f32) -> f32 {
    let whole = floor(uv + 1.0);
    return select(0.0, 1.0, abs((whole / 2.0) % 1.0) < 0.001);
}


//////////////////////// sdfs //////////////////////////////////////

fn sdRoundedBox(p: vec2<f32>, b: vec2<f32>, r: vec4<f32>) -> f32 {
    var x = r.x;
    var y = r.y;
    x = select(r.z, r.x, p.x > 0.);
    y = select(r.w, r.y, p.x > 0.);
    x = select(y, x, p.y > 0.);
    let q = abs(p) - b + x;
    return min(max(q.x, q.y), 0.) + length(max(q, vec2<f32>(0.))) - x;
}

fn sdSegment(p: vec2<f32>, a: vec2<f32>, b: vec2<f32>) -> f32 {
    let pa: vec2<f32> = p - a;
    let ba: vec2<f32> = b - a;
    let h = clamp(dot(pa, ba) / dot(ba, ba), 0.0, 1.0);
    return length(pa - ba * h);
}

// fn draw_segment(thickness: f32, rect: vec4<f32>, uv: vec2<f32>, segment: Segment, color: vec4<f32>, alpha: f32 ) -> vec4<f32> {
//     let t = thickness; 
//     let d = sdSegment(uv, segment.start, segment.end);
//     let seg_start = smoothstep(t, t + 1.0/100.0,   d);
//     let rect2 = mix(rect,  color,   alpha*abs( 1.0 -seg_start));
//     return rect2;
// }

fn draw_segment(thickness: f32, rect: vec4<f32>, uv: vec2<f32>, segment: Segment, color: vec4<f32>, alpha: f32) -> vec4<f32> {
    // let uv = from_local_to_pixels(uv_orig);
    let t = thickness; // * mate.globals.zoom;
    let d = sdSegment(uv, segment.start, segment.end);
    let seg_start = smoothstep(t, t + t * 2.0, d);
    let rect2 = mix(rect, color, alpha * abs(1.0 - seg_start));
    return rect2;
}


fn sdCircle(pos: vec2<f32>, r: f32) -> f32 {
    return length(pos) - r;
}

fn draw_circle(
    rect: vec4<f32>,
    uv_orig: vec2<f32>,
    r: f32,
    pcolor: vec4<f32>,
    annular: bool,
    point2: vec2<f32>,
) -> vec4<f32> {

    let t = solid * 100.0;
    let s = smooth_dist2 * 200.0;

    let uv_pixels = from_local_to_pixels(uv_orig - point2);
    let r_pixels_vec2 = from_local_to_pixels(vec2<f32>(r, r));

    var sd_start = sdCircle(uv_pixels, r_pixels_vec2.x);
    //
    if annular {
        sd_start = abs(sd_start);
    }
    let cerc_start = smoothstep(t, t + s * 2., sd_start);
    let rect2 = mix(rect, pcolor, 1.0 - cerc_start);
    return rect2;
}
//////////////////////// sdfs //////////////////////////////////////





@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {



    // ///////////////////// coordinates /////////////////////////////////
    let x_max = mate.bound_up.x;
    let y_max = mate.bound_up.y;

    let x_min = mate.bound_lo.x;
    let y_min = mate.bound_lo.y;

    let x_range = x_max - x_min;
    let y_range = y_max - y_min;
    // ///////////////////// coordinates /////////////////////////////////

    let uv_pix = from_local_to_pixels3(in.uv);
    // let uv_pix_inv = from_local_to_pixels3_inv(in.uv);

    var uv = from_pix_to_local(uv_pix) ;

    // var uv = in.uv;
    // var uv = uv_pix;
    // var uv = in.world_position.xy;

    // if uv_pix.x > 0.5 {
    //     return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    // }



    ///////////////////// colors /////////////////////////////////////
    let red = vec4<f32>(1.0, 0.0, 0.0, 1.0);
    let yellow = vec4<f32>(0.89, 0.41, 0.14, 1.0);
    let green = vec4<f32>(0.0, 1.0, 0.0, 1.0);
    let black = vec4<f32>(0.0, 0.0, 0.0, 1.0);

    let colBackground1 = mate.background_color1;
    let colBackground2 = mate.background_color2;
    ///////////////////// colors /////////////////////
    

    ///////////////////// background /////////////////
    let tile_freq_x: f32 = 1.0 / mate.tick_period.x;
    let tile_freq_y: f32 = 1.0 / mate.tick_period.y;

    let tiles = even((floor(tile_freq_x * uv.x) + floor(tile_freq_y * uv.y))) ; //+ even(uv.y * 5.);

    var rect: vec4<f32> = mix(colBackground1, colBackground2, tiles);
    ///////////////////// background /////////////////



    ////////////////////////////////// grid ////////////////////////////////
    let so = mate.size / (1.0 + mate.outer_border);
    let edges = vec2<f32>(0.5, 0.5) * so;

    var origin = (-mate.bound_lo / (mate.bound_up - mate.bound_lo) - 0.5) * so;
    // origin.y = -origin.y;

    let tick_period_pix = mate.tick_period / (mate.bound_up - mate.bound_lo) * so;
    let bar_alpha = 1.0;

    var segment: Segment;

    var sig = sign(uv);
    sig = vec2<f32>(1.0, 1.0);

    // in the tiki coordinate, 1 corresponds to one tick period
    let tiki = (uv_pix - graph_position.position - origin) / tick_period_pix - vec2<f32>(0.5, 0.5) * sig ;

    // In wgpu currently, the mod function take a reference to a dummy variable.
    // This will change in the future.
    var temp_y: f32 = 0.0;
    var temp_x: f32 = 0.0;
    // let sad_x = modf(tiki.x, &temp_x);
    // let sad_y = modf(tiki.y, &temp_y);
    let sad_x = tiki.x % 1.0;
    let sad_y = tiki.y % 1.0;


    let ggg = -vec2<f32>(sad_x, sad_y) ;


    let half = -0.5 * sig;

    let aspect_ratio = tick_period_pix.x / tick_period_pix.y;

    let bars_thickness = 0.5 / tick_period_pix ;

    if mate.show_grid > 0.5 {
        // horizontal bars
        segment.start = vec2<f32>(-edges.x, half.y) ;
        segment.end = vec2<f32>(edges.x, half.y) ;
        rect = draw_segment(bars_thickness.y, rect, ggg, segment, black, bar_alpha) ;

        // vertical bars
        segment.start = vec2<f32>(half.x, -edges.y) ;
        segment.end = vec2<f32>(half.x, edges.y) ;
        rect = draw_segment(bars_thickness.x, rect, ggg, segment, black, bar_alpha) ;
    }
    /////////////////////////////////////// grid /////////////////////////////////////



    /////////////////////////////////////// axes //////////////////////////////
    if mate.show_axes > 0.5 {
        segment.start = vec2<f32>(-edges.x, origin.y);
        segment.end = vec2<f32>(edges.x, origin.y);
        rect = draw_segment(1.0, rect, uv_pix - graph_position.position, segment, black, bar_alpha) ;


        segment.start = vec2<f32>(origin.x, -edges.y);
        segment.end = vec2<f32>(origin.x, edges.y);
        rect = draw_segment(1.0, rect, uv_pix - graph_position.position, segment, black, bar_alpha) ;
    }
    //////////////////////////////////////// axes //////////////////////////////



    /////////////////// borders /////////////////////////
    rect = mix(rect, colBackground2, step(x_max, uv.x));
    rect = mix(rect, colBackground2, step(-x_min, -uv.x));
    rect = mix(rect, colBackground2, step(-y_min, -uv.y));
    rect = mix(rect, colBackground2, step(y_max, uv.y));
    /////////////////// borders /////////////////////////


    /////////////////// mouse target /////////////////////////
    if mate.show_target > 0.5 {
        // let aspect_ratio = mate.size.y / mate.size.x;

        let target_thickness = 0.75; // mate.globals.zoom;
        let pos_edges = edges - graph_position.position;

        segment.start = vec2<f32>(mate.target_pos.x, -pos_edges.y);
        segment.end = vec2<f32>(mate.target_pos.x, pos_edges.y);
        rect = draw_segment(target_thickness, rect, uv_pix, segment, mate.target_color, bar_alpha);

        segment.start = vec2<f32>(-pos_edges.x, mate.target_pos.y);
        segment.end = vec2<f32>(pos_edges.x, mate.target_pos.y);
        rect = draw_segment(target_thickness, rect, uv_pix, segment, mate.target_color, bar_alpha);
    }
    /////////////////// mouse target /////////////////////////


    /////////////////// contours /////////////////////////
    if mate.hide_contour < 0.5 {

        let so = mate.size / (1.0 + mate.outer_border);
        let ax_thick = 0.8 ;

        let r = 0.02 * so.x;
        let d = sdRoundedBox(uv_pix - graph_position.position, so / 2.0, vec4<f32>(r, r, r, r));
        let s = smoothstep(0.0, 2.0, d);

        let colBackground3 = vec4<f32>(colBackground2.xyz, 0.0);
        rect = mix(rect, colBackground3, s);

        let r2 = 0.02 * so.x;
        let d2 = sdRoundedBox(uv_pix - graph_position.position, so / 2.0, vec4<f32>(r2, r2, r2, r2));
        let s2 = smoothstep(0.0, 2.0, abs(d2) - 1.0);

        rect = mix(rect, vec4<f32>(0.0, 0.0, 0.0, 1.0), 1.0 - s2);
    }
    /////////////////// contours /////////////////////////



    return rect;
}
