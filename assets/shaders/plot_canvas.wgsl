// #import bevy::pbr::mesh_view_bind_group
// #import bevy::pbr::mesh_struct

type float4 = vec4<f32>;
type float2 = vec2<f32>;

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] uv: vec2<f32>;
};

struct Segment {
    start: float2;
    end: float2;
};

struct Interval {
    start: vec2<f32>;
    end: vec2<f32>;
    control: float4;
};

struct GraphSize {
    size: float2;
    outer_border: float2;
};

struct Bound {
    up: float2;
    lo: float2;
};

var<private>  solid: f32 = 0.001;  
var<private>  smooth_dist2: f32 = 0.003;
var<private>  point_radius: f32 = 0.03;
var<private>  out_of_bounds: f32 = 100000.0;
var<private>  bluish : float4 = float4 (0.13, 0.28, 0.86, 1.0);
var<private>  num_segments: i32 = 256;



struct Globals {
    time: f32;
    zoom: f32;
    dum1: f32;
    dum2: f32;
};

struct GraphEditShader {
    
    mouse_pos: vec2<f32>;
    tick_period: vec2<f32>;
    bounds: Bound;
    globals: Globals;
    size: float2;
    outer_border: float2;
    position: vec2<f32>;

    // vars: array<Interval, 16>;
    // lines_params: array<LineParams, 8> ;
    // data: array<array<float4, 256>, 8>;
    // data: array<float2, 64>;
    
};

[[group(1), binding(0)]]
var<uniform> mate: GraphEditShader;

fn from_pix_to_local(uv_orig: float2) -> float2 {

    var uv = (uv_orig - mate.position) ;

    let x_max = mate.bounds.up.x;
    let y_max = mate.bounds.up.y;

    let x_min = mate.bounds.lo.x;
    let y_min = mate.bounds.lo.y;

    let x_range = x_max - x_min;
    let y_range = y_max - y_min;

    uv.x = uv.x * (1.0 + mate.outer_border.x ) / mate.size.x ;
    uv.x = uv.x * x_range ; 

    uv.y = uv.y * (1.0 + mate.outer_border.y ) / mate.size.y;
    uv.y = uv.y * y_range;

    let current_zero_pos = float2( x_range / 2.0 + x_min, y_range / 2.0 + y_min);
    let uv_local = uv + current_zero_pos;

    return uv_local;
};

fn from_local_to_pixels(uv_local: float2) -> float2 {
    // unimplemented
    var uv = uv_local;

    uv.x = uv.x * mate.size.x / (1.0 + mate.outer_border.x ) ;
    uv.x = uv.x / (mate.bounds.up.x - mate.bounds.lo.x);

    uv.y = uv.y * mate.size.y / (1.0 + mate.outer_border.y ) ;
    uv.y = uv.y / (mate.bounds.up.y - mate.bounds.lo.y);

    // only scaled at the moment: no offset to the original uv

    return uv;
};

fn from_local_to_pixels2(uv_local: float2) -> float2 {
    // unimplemented
    var uv = uv_local;

    uv.x = uv.x * mate.size.x / (1.0 + mate.outer_border.x ) ;
    uv.x = uv.x / (mate.bounds.up.x - mate.bounds.lo.x);

    uv.y = uv.y * mate.size.y / (1.0 + mate.outer_border.y ) ;
    uv.y = uv.y / (mate.bounds.up.y - mate.bounds.lo.y);

    // only scaled at the moment: no offset to the original uv

    return uv;
};





fn even(uv: f32) -> f32 {
    var tempo: f32 = 0.0;
    let whatever = modf(uv + 1.0, &tempo);
    var temp2 = 0.;
    let frac = modf(tempo / 2.0, &temp2);

    if (abs(frac) < 0.001) {
        return 1.0;
    } else {
        return 0.0;
    }

}

//////////////////////// sdfs //////////////////////////////////////

fn sdRoundedBox(p: vec2<f32>, b: vec2<f32>, r: vec4<f32>) -> f32 {
  var x = r.x;
  var y = r.y;
  x = select(r.z, r.x, p.x > 0.);
  y = select(r.w, r.y, p.x > 0.);
  x  = select(y, x, p.y > 0.);
  let q = abs(p) - b + x;
  return min(max(q.x, q.y), 0.) + length(max(q, vec2<f32>(0.))) - x;
}

fn sdSegment(p: vec2<f32>, a: vec2<f32>, b: vec2<f32>) -> f32 {
    let pa: vec2<f32> = p-a;
    let ba: vec2<f32>  = b-a;
    let h = clamp( dot(pa,ba)/dot(ba,ba), 0.0, 1.0 );
    return length( pa - ba*h );
}

fn draw_segment(thickness: f32, rect: vec4<f32>, uv: vec2<f32>, segment: Segment, color: float4, alpha: f32 ) -> vec4<f32> {
    // let uv = from_local_to_pixels(uv_orig);
    let t = thickness; // * mate.globals.zoom;
    let d = sdSegment(uv, segment.start, segment.end);
    let seg_start = smoothStep(t, t + t * 2.0,   d);
    let rect2 = mix(rect,  color,   alpha*abs( 1.0 -seg_start));
    return rect2;
}

fn draw_segment_orig(
    thickness: f32, 
    rect: vec4<f32>,
    uv_orig: vec2<f32>, 
    segment: Segment, 
    color: float4, 
    alpha: f32 
    ) -> vec4<f32> {

    let uv = from_local_to_pixels(uv_orig);
    let start = from_local_to_pixels(segment.start);
    let end = from_local_to_pixels(segment.end);

    let t = thickness;
    let d = sdSegment(uv, start, end);
    let seg_start = smoothStep(t, t + t * 5.0,   d);
    let rect2 = mix(rect,  color,   alpha*abs( 1.0 -seg_start));
    return rect2;
}

// fn draw_segment(
//     thickness: f32, 
//     rect: vec4<f32>, 
//     uv: vec2<f32>, 
//     segment: Segment,
//     color: float4, 
//     alpha: f32 
//     ) -> vec4<f32> {

//     let d = sdSegment(uv, segment.start, segment.end);
//     let seg_start = smoothStep(thickness, thickness + thickness * 10.0,   d);
//     let rect2 = mix(rect,  color,   alpha*abs( 1.0 -seg_start));
//     return rect2;
// }

fn sdCircle(pos: vec2<f32>, r: f32) -> f32 {
    return length(pos)-r;
}

fn draw_circle(
    rect: vec4<f32>,
    uv_orig: vec2<f32>,
    r: f32,
    pcolor: vec4<f32>,
    annular: bool,
    point: vec2<f32>,
) -> vec4<f32> {

    let t = solid * 100.0;
    let s = smooth_dist2 * 200.0;

    //     let t = mate.globals.zoom * solid * 100.0;
    // let s = smooth_dist2 * mate.globals.zoom * 100.0;

    // let m = mate.size.x / 600.0 * float2( 1.0, (mate.size.y / mate.size.x) * (mate.bounds.up.x - mate.bounds.lo.x) / (mate.bounds.up.y - mate.bounds.lo.y)  );

    let uv_pixels = from_local_to_pixels(uv_orig - point);
    let r_pixels_vec2 = from_local_to_pixels(float2(r,r));
    // var sd_start = sdCircle((uv_orig - point) * m, r * mate.globals.zoom);
    // var sd_start = sdCircle( uv_pixels, r_pixels_vec2.x );
    var sd_start = sdCircle( uv_pixels, r_pixels_vec2.x  );
    //
    if (annular) {
        sd_start = abs(sd_start);
    }
    let cerc_start = smoothStep(t, t + s * 2., sd_start);
    let rect2 = mix(rect, pcolor, 1.0 -cerc_start);
    return rect2;
}
//////////////////////// sdfs //////////////////////////////////////





[[stage(fragment)]]
fn fragment(in: VertexOutput) -> [[location(0)]] vec4<f32> {

    // ///////////////////// coordinates /////////////////////////////////
    // var uv = (in.uv - mate.position) ;

    let x_max = mate.bounds.up.x;
    let y_max = mate.bounds.up.y;

    let x_min = mate.bounds.lo.x;
    let y_min = mate.bounds.lo.y;

    let x_range = x_max - x_min;
    let y_range = y_max - y_min;

    // uv.x = uv.x * (1.0 + mate.outer_border.x ) / mate.size.x ;
    // uv.x = uv.x * x_range ; 

    // uv.y = uv.y * (1.0 + mate.outer_border.y ) / mate.size.y;
    // uv.y = uv.y * y_range;

    // let current_zero_pos = float2( x_range / 2.0 + x_min, y_range / 2.0 + y_min);
    // uv = uv + current_zero_pos;
    // ///////////////////// coordinates /////////////////////////////////

    var uv = from_pix_to_local(in.uv) ;


    ///////////////////// colors /////////////////////////////////////
    let red  = vec4<f32>(1.0, 0.0, 0.0, 1.0);
    let yellow = vec4<f32> (0.89, 0.41, 0.14, 1.0);
    let green = vec4<f32> (0.0, 1.0, 0.0, 1.0);
    let black = vec4<f32> (0.0, 0.0, 0.0, 1.0);

    let bm = 0.4; 
    let  colBackground1 = vec4<f32> ( vec3<f32>(62./255., 6./255., 95./255.)/4., 1.0 / bm) * bm;
    let  colBackground2 = vec4<f32>( vec3<f32>(112./255., 11./255., 151./255.)/4., 1.0 / bm ) *bm;
    ///////////////////// colors /////////////////////
    

    ///////////////////// background /////////////////
    let tile_freq_x: f32 = 1.0 / mate.tick_period.x;
    let tile_freq_y: f32 = 1.0 / mate.tick_period.y;

    // var temp: f32 = 0.5;
    // let dum = modf(( floor(tile_freq*uv.x) + floor(tile_freq*uv.y) ) , &temp);
   
    let what = even( (floor(tile_freq_x*uv.x) + floor(tile_freq_y*uv.y) ) ) ; //+ even(uv.y * 5.);

    var rect: vec4<f32> = mix(colBackground1, colBackground2, what );
    ///////////////////// background /////////////////



    ////////////////////////////////// bars ////////////////////////////////
    

    let bah = mate.size / (1.0 + mate.outer_border);
    let edges = float2(0.5 , 0.5) * bah;

    // let dv = from_local_to_pixels(floor(mate.bounds.lo / mate.tick_period) *  mate.tick_period) ;
    let origin = (-mate.bounds.lo / (mate.bounds.up - mate.bounds.lo) - 0.5) * bah;

    let tick_period_pix = mate.tick_period / (mate.bounds.up - mate.bounds.lo) * bah;
    let bar_alpha = 1.0;

    var segment: Segment;

    var sig = sign(uv);

    // in the tiki coordinate, 1 corresponds to one tick period
    let tiki = (in.uv - mate.position - origin) / tick_period_pix   - float2(0.5, 0.5) * sig ;

    // mod function take a reference to a dummy variable
    var temp_y: f32 = 0.0;
    var temp_x: f32 = 0.0;
    let sad_x = modf(tiki.x , &temp_x);
    let sad_y = modf(tiki.y, &temp_y);


    let ggg = -float2(sad_x, sad_y ) ;


    let half = -0.5 * sig;

    let aspect_ratio = tick_period_pix.x / tick_period_pix.y;

    let bars_thickness  = 0.5 / tick_period_pix;

    // horizontal bars
    segment.start = float2(-edges.x,  half.y) ;
    segment.end = float2( edges.x,  half.y) ;
    rect = draw_segment(bars_thickness.y, rect, ggg  , segment, black, bar_alpha) ;

    // vertical bars
    segment.start = float2(half.x, -edges.y) ;
    segment.end = float2( half.x, edges.y) ;
    rect = draw_segment(bars_thickness.x , rect, ggg  , segment, black, bar_alpha) ;
    /////////////////////////////////////// bars /////////////////////////////////////



    /////////////////////////////////////// axes //////////////////////////////
    segment.start = float2(-edges.x,  origin.y) ;
    segment.end = float2( edges.x,  origin.y) ;
    rect = draw_segment(1.0, rect, in.uv - mate.position, segment, black, bar_alpha) ;


    segment.start = float2(origin.x , -edges.y) ;
    segment.end = float2(origin.x , edges.y) ;
    rect = draw_segment(1.0, rect, in.uv - mate.position, segment, black, bar_alpha) ;
    //////////////////////////////////////// axes //////////////////////////////



    /////////////////// borders /////////////////////////
    rect = mix(rect, colBackground2, step(x_max, uv.x));
    rect = mix(rect, colBackground2, step(-x_min, -uv.x));
    rect = mix(rect, colBackground2, step(-y_min, -uv.y));
    rect = mix(rect, colBackground2, step(y_max, uv.y));
    /////////////////// borders /////////////////////////


    /////////////////// mouse target /////////////////////////
    // rect = draw_circle(rect, uv, rad*2. , black, ann, mate.mouse_pos);
    let gray = vec4<f32>(1.0, 1.0, 1.0, 1.0) * 0.5;
    // let aspect_ratio = mate.size.y / mate.size.x;
    // let aspect_ratio = 1.0;
    let target_thickness = 0.5 * mate.globals.zoom;
    let pos_edges = edges - mate.position;

    segment.start = float2( mate.mouse_pos.x, -pos_edges.y ) ;
    segment.end = float2(  mate.mouse_pos.x, pos_edges.y ) ;
    rect = draw_segment(target_thickness , rect, in.uv  , segment, gray, bar_alpha) ;

    segment.start = float2(-pos_edges.x,  mate.mouse_pos.y);
    segment.end = float2( pos_edges.x,  mate.mouse_pos.y);
    rect = draw_segment(target_thickness , rect, in.uv  , segment, gray, bar_alpha) ;
    /////////////////// mouse target /////////////////////////


    /////////////////// contours /////////////////////////
    // segment.start = float2(x_min,  y_min);
    // segment.end = float2(x_max,  y_min) ;

    // segment.start = float2(0.02,  0.02);
    // segment.end = float2(0.5 * 0.975,  0.0) * mate.size.x / 1.0 ;

    let bah = mate.size / (1.0 + mate.outer_border );
    let ax_thick = 0.8 ;

    let r = 0.02 * bah.x;
    let d = sdRoundedBox(in.uv - mate.position,  bah / 2.0, float4(r,r,r,r));
    let s = smoothStep(0.0, 2.0, d );

    let colBackground3 = float4(colBackground2.xyz, 0.0);
    rect = mix(rect, colBackground3, s);

    let r = 0.02 * bah.x;
    let d = sdRoundedBox(in.uv - mate.position,  bah / 2.0, float4(r,r,r,r));
    let s = smoothStep(0.0, 2.0, abs(d) - 1.0 );

    rect = mix(rect, float4(0.0, 0.0, 0.0, 1.0), 1.0 - s);

    /////////////////// contours /////////////////////////

    // ////////////////////////// circles points //////////////////////////////////
    let ann: bool = false;
    let rad = 0.006 * mate.globals.zoom * 3.0;
    // let rad2 = 0.002 * mate.globals.zoom *3.0;
    // rect = draw_circle(rect, uv, rad, black, ann, float2(x_min, y_min));
    // rect = draw_circle(rect, uv,  rad, black, ann, float2(x_max, y_min));
    // rect = draw_circle(rect, uv,  rad, black, ann, float2(x_min, y_max));
    // rect = draw_circle(rect, uv,  rad, black, ann, float2(x_max, y_max));

    // let bm = 0.75;
    // bluish = bluish * bm;
    // bluish.w = 1.0;
    // rect = draw_circle(rect, uv, rad2  , bluish, ann, float2(x_min, y_min));
    // rect = draw_circle(rect, uv,  rad2 , bluish, ann, float2(x_max, y_min));
    // rect = draw_circle(rect, uv,  rad2 , bluish, ann, float2(x_min, y_max));
    // rect = draw_circle(rect, uv,  rad2 , bluish, ann, float2(x_max, y_max));
    // bluish = bluish / bm;

        // // test
    
    // ////////////////////////// Black points ///////////////////////////////////


    return rect;
}
