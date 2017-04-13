#![feature(alloc,collections)]
#![feature(use_extern_macros)]
#![no_std]
#[macro_use]
extern crate alloc;
extern crate collections;
pub extern crate renderer;
use collections::vec::Vec;
use core::ops::{Add, Sub, AddAssign, SubAssign, Mul};
use renderer::coordinates::{Pixel16, Coord2D};

#[derive(Debug, PartialEq)]
pub struct ControlPoly {
    pub data: Vec<Coord2D>,
    pub closed: bool,
    pub line_thickness: u8,
}

impl ControlPoly {
    pub fn new(points: Vec<Coord2D>) -> ControlPoly {
        ControlPoly {
            data: points,
            closed: false,
            line_thickness: 3,
        }
    }

    pub fn eval_spline_uniform(&self, iterations: u16) -> Vec<Coord2D> {
        //! Will give you around `iterations` points that are on the spline,
        //! with a uniform distance. Render Lines from point to point to draw the spline.
        let mut result: Vec<Coord2D> = Vec::new();
        let mut t: f32 = 0.0;
        while t < 1.0 {
            // result.push(self.casteljau_step().1);
            result.push(self.get_casteljau_point(self.data.len() - 1, 0, t));
            t += 1f32 / (iterations as f32);
        }
        result
    }


    fn get_casteljau_point(&self, r: usize, i: usize, t: f32) -> Coord2D {
        //! Recursive function to get the casteljau point at position `t`.
        //! Note that `t` should be between 0 and 1 to make sense,
        //! Although there is no assert that disallows different values
        //! for creative purposes ;D
        //! usage: `self.get_casteljau_point(self.data.len() - 1, 0, t)`
        if r == 0 {
            return self.data[i];
        }

        let p1: Coord2D = self.get_casteljau_point(r - 1, i, t);
        let p2: Coord2D = self.get_casteljau_point(r - 1, i + 1, t);

        Coord2D {
            x: (1.0 - t) * p1.x + t * p2.x,
            y: (1.0 - t) * p1.y + t * p2.y,
        }
    }

    fn get_casteljau_point_mid_sub(&self) -> (ControlPoly, Coord2D, ControlPoly) {
        //! Returns the casteljau point for t = 0.5
        //! and the resulting `ControlPoly`s to the left and right.
        //! This is the way the algorithm is teached in theoretical lectures.
        let mut left: Vec<Coord2D> = Vec::new();
        let mut right: Vec<Coord2D> = Vec::new();
        let mut vec = self.data.clone();
        let n = vec.len();
        left.push(vec[0]);
        right.push(vec[n - 1]);
        for k in 1..(n + 1) {
            let mut result: Vec<Coord2D> = Vec::new();
            for i in 0..n - k {
                let p = ControlPoly::casteljau_divisor_half(vec[i], vec[i + 1]);
                result.push(p);
                if i == 0 {
                    left.push(p);
                } else if n - 1 == i + k {
                    right.insert(0, p);
                }
            }
            vec = result.clone();
        }
        assert!(right.len() > 0);
        let p = right[0];
        let ctrl_l = ControlPoly {
            data: left,
            closed: self.closed,
            line_thickness: self.line_thickness,
        };
        let ctrl_r = ControlPoly {
            data: right,
            closed: self.closed,
            line_thickness: self.line_thickness,
        };
        (ctrl_l, p, ctrl_r)
    }

    fn casteljau_divisor(left: Coord2D, right: Coord2D, factor: f32) -> Coord2D {
        debug_assert!(factor < 1_f32 && factor > 0_f32);
        left + (right - left) * factor
    }

    fn casteljau_divisor_half(left: Coord2D, right: Coord2D) -> Coord2D {
        left + (right - left) * 0.5_f32
    }

    fn a_frame_divisor(left: Coord2D, right: Coord2D) -> Coord2D {
        left + (right - left) * (1_f32 / 3_f32)
    }
}

#[test]
fn test_casteljau() {
    //! This tests the casteljau implementation,
    //! assuming one casteljau iteration of the given numbers.
    use collections::vec;
    let ctrl: ControlPoly = ControlPoly {
        data: vec![Coord2D { x: 0f32, y: 0f32 },
                   Coord2D { x: 0f32, y: 1f32 },
                   Coord2D { x: 1f32, y: 1f32 },
                   Coord2D { x: 1f32, y: 0f32 }],
        closed: false, // not yet used
        line_thickness: 10, // not yet used
    };
    let draw = ctrl.eval_with_casteljau();
    let correct_draw = ControlPoly {
        data: vec![Coord2D { x: 0f32, y: 0f32 },
                   Coord2D { x: 0f32, y: 0.5 },
                   Coord2D { x: 0.25, y: 0.75 },
                   Coord2D { x: 0.5, y: 0.75 },
                   Coord2D { x: 1f32, y: 0f32 }],
        closed: false,
        line_thickness: 10,
    };
    assert_eq!(draw, correct_draw);
}

#[test]
fn test_divisor_half() {
    let p00 = Coord2D { x: 0f32, y: 0f32 };
    let p01 = Coord2D { x: 0f32, y: 1f32 };
    let p10 = Coord2D { x: 1f32, y: 1f32 };
    let p11 = Coord2D { x: 1f32, y: 0f32 };

    let p00_p01 = Coord2D { x: 0f32, y: 0.5f32 };
    let p01_p10 = Coord2D { x: 0.5f32, y: 1f32 };
    let p10_p11 = Coord2D { x: 1f32, y: 0.5f32 };
    let mid1 = Coord2D {
        x: 0.25f32,
        y: 0.75f32,
    };
    let mid3 = Coord2D {
        x: 0.75f32,
        y: 0.75f32,
    };
    let mid2 = Coord2D {
        x: 0.5f32,
        y: 0.75f32,
    };

    assert_eq!(ControlPoly::casteljau_divisor_half(p00, p01), p00_p01);
    assert_eq!(ControlPoly::casteljau_divisor_half(p01, p10), p01_p10);
    assert_eq!(ControlPoly::casteljau_divisor_half(p10, p11), p10_p11);

    assert_eq!(ControlPoly::casteljau_divisor_half(p00_p01, p01_p10), mid1);
    assert_eq!(ControlPoly::casteljau_divisor_half(p01_p10, p10_p11), mid3);
    assert_eq!(ControlPoly::casteljau_divisor_half(mid1, mid3), mid2);
}
