use arith::{self, PhysicsNum};
use cgmath::{vec2, Vector2};
use num::One;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb<N> {
    top_left: Vector2<N>,
    size: Vector2<N>,
}

pub struct AabbSplitFour<N> {
    pub top_left: Aabb<N>,
    pub top_right: Aabb<N>,
    pub bottom_left: Aabb<N>,
    pub bottom_right: Aabb<N>,
}

impl<N: PhysicsNum> Aabb<N> {
    pub fn new(top_left: Vector2<N>, size: Vector2<N>) -> Self {
        Self { top_left, size }
    }
    pub fn from_centre_and_half_size(centre: Vector2<N>, half_size: Vector2<N>) -> Self {
        let top_left = centre - half_size;
        let size = half_size * arith::two();
        Self::new(top_left, size)
    }
    fn bottom_right_coord(&self) -> Vector2<N> {
        self.top_left + self.size
    }
    pub fn from_union(a: &Aabb<N>, b: &Aabb<N>) -> Self {
        let top_left = vec2(
            a.top_left.x.min(b.top_left.x),
            a.top_left.y.min(b.top_left.y),
        );
        let a_bottom_right_coord = a.bottom_right_coord();
        let b_bottom_right_coord = b.bottom_right_coord();
        let bottom_right_coord = vec2(
            a_bottom_right_coord.x.max(b_bottom_right_coord.x),
            a_bottom_right_coord.y.max(b_bottom_right_coord.y),
        );
        let size = bottom_right_coord - top_left;
        Self::new(top_left, size)
    }
    pub fn union(&self, other: &Self) -> Self {
        Self::from_union(self, other)
    }
    pub fn size(&self) -> Vector2<N> {
        self.size
    }
    pub fn is_intersecting(&self, other: &Aabb<N>) -> bool {
        self.top_left.x + self.size.x >= other.top_left.x
            && other.top_left.x + other.size.x >= self.top_left.x
            && self.top_left.y + self.size.y >= other.top_left.y
            && other.top_left.y + other.size.y >= self.top_left.y
    }
    pub fn centre(&self) -> Vector2<N> {
        self.top_left + self.size / arith::two()
    }
    pub fn split_four(&self) -> AabbSplitFour<N> {
        let size = self.size / arith::two();
        AabbSplitFour {
            top_left: Self::new(self.top_left, size),
            top_right: Self::new(vec2(self.top_left.x + size.x, self.top_left.y), size),
            bottom_left: Self::new(vec2(self.top_left.x, self.top_left.y + size.y), size),
            bottom_right: Self::new(self.top_left + size, size),
        }
    }
    pub fn double_about_centre(&self) -> Self {
        Self::from_centre_and_half_size(self.centre(), self.size)
    }
}
