extern crate splines;
extern crate renderer;
use splines::ControlPoly;
use renderer::{AABB, Renderer};
use renderer::coordinates::{Coord2D, Pixel16};

/// Minimal usage example and playground.
fn main() {
    let mut aabb = AABB {
        min: Pixel16 { x: 10, y: 10 },
        max: Pixel16 { x: 15, y: 20 },
    };
    // control polygon
    let mut ctrl: ControlPoly = ControlPoly {
        data: vec![Coord2D { x: 0f32, y: 1f32 },
                   Coord2D { x: 1f32, y: 0f32 },
                   Coord2D { x: 1.5f32, y: 0f32 },
                   Coord2D { x: 2.5f32, y: 1f32 }],
        closed: false, // not yet used
        line_thickness: 10, // not yet used
    };
    //ctrl.data = aabb.fit_in_aabb(ctrl.data.as_slice());
    // ControlPoly -> spline approximation
    let spline = ctrl.eval_with_casteljau();
    println!("ctrl_inAABB: {:?}\nspline: {:?}", ctrl.data, spline.data);
    // set up renderer
    let func = |x, y, color| print!("({}, {})", x, y);
    let mut r = Renderer { pixel_col_fn: func };
    // plot data into given AABB.
    r.draw_lines(aabb, spline.data);
}
