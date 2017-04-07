extern crate splines;
use splines::{Point, ControlPoly};

/// Minimal usage example and playground.
fn main() {
    let ctrl: ControlPoly = ControlPoly {
        data: vec![Point { x: 0f32, y: 0f32 },
                   Point { x: 0f32, y: 1f32 },
                   Point { x: 1f32, y: 1f32 },
                   Point { x: 1f32, y: 0f32 }],
        closed: false, // not yet used
        line_thickness: 10, // not yet used
    };
    let draw = ctrl.eval_with_casteljau();
    println!("ctrl: {:?}\ndraw: {:?}", ctrl.data, draw.data);
}
