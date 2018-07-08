#![feature(nonzero)]
extern crate best;
extern crate cgmath;
#[macro_use]
extern crate custom_derive;
extern crate fnv;
#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;
#[macro_use]
extern crate newtype_derive;
extern crate num;

mod aabb;
mod arith;
mod axis_aligned_rect;
mod collision;
mod line_segment;
mod shape;

fn main() {
    let a = cgmath::Vector2::new(arith::SubPixelI64::new(12), arith::SubPixelI64::new(23));
    let b = cgmath::Vector2::new(arith::SubPixelI64::new(7), arith::SubPixelI64::new(17));
    let c = cgmath::Vector2::new(arith::SubPixelI64::new(2), arith::SubPixelI64::new(3));
    let d = cgmath::Vector2::new(arith::SubPixelI64::new(15), arith::SubPixelI64::new(9));
    let line = line_segment::LineSegment::new(a, b);
    let col = collision::vertex_moving_towards_edge(c, d, line);
    println!("{:?}", col);
}
