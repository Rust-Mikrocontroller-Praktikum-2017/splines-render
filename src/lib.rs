#![feature(alloc,collections)]
#![feature(use_extern_macros)]
#![no_std]
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
    pub fn new(points: Vec<Coord2D>, thickness: u8, closed: bool) -> ControlPoly {
        ControlPoly {
            data: points,
            closed: closed,
            line_thickness: thickness,
        }
    }

    pub fn eval_with_casteljau(&self) -> ControlPoly {
        let mut output: Vec<Coord2D> = Vec::new();
        let mut vec = self.data.clone();
        let n = vec.len();
        output.push(vec[0]);
        output.push(vec[n - 1]);
        for k in 1..(n + 1) {
            let mut result: Vec<Coord2D> = Vec::new();
            for i in 0..n - k {
                let p = ControlPoly::casteljau_divisor_half(vec[i], vec[i + 1]);
                result.push(p);
                if i == 0 {
                    output.insert(k, p);
                } else if n - 1 == i + k {
                    let l = output.len() - k;
                    output.insert(l, p);
                }
            }
            vec = result.clone();
        }
        ControlPoly {
            data: output,
            closed: self.closed,
            line_thickness: self.line_thickness,
        }
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
