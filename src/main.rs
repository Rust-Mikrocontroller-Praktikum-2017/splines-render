extern crate splines;
extern crate renderer;
use splines::ControlPoly;
use renderer::{AABB, Renderer};
use renderer::coordinates::{Coord2D, Pixel16};

/// Minimal usage example and playground.
fn main() {
    // control polygon
    let ctrl: ControlPoly = ControlPoly {
        data: vec![Coord2D { x: 0f32, y: 0f32 },
                   Coord2D { x: 0f32, y: 1f32 },
                   Coord2D { x: 1f32, y: 1f32 },
                   Coord2D { x: 1f32, y: 0f32 }],
        closed: false, // not yet used
        line_thickness: 10, // not yet used
    };
    // ControlPoly -> spline approximation
    let draw = ctrl.eval_with_casteljau();
    println!("ctrl: {:?}\ndraw: {:?}", ctrl.data, draw.data);
    // set up renderer
    let func = |x, y, color| println!("({}, {})", x, y);
    let mut r = Renderer { pixel_col_fn: func };
    // set up drawing area (AABB)
    let mut aabb = AABB {
        min: Pixel16 { x: 10, y: 10 },
        max: Pixel16 { x: 15, y: 20 },
    };
    // fit data into drawing area
    let data = aabb.fit_in_aabb(draw.data.as_slice());
    println!("{:?}", draw.data.as_slice());
    println!("{:?}", data);
    // plot data into given AABB.
    r.draw_lines(aabb, data);
}
