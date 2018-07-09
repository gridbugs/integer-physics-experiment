use aabb::Aabb;
use physics_num::PhysicsNum;
use cgmath::{Vector2, vec2};
use line_segment::LineSegment;
use num::Zero;
use shape::Collide;

#[derive(Debug, Clone)]
pub struct AxisAlignedRect<N: PhysicsNum> {
    dimensions: Vector2<N>,
}

impl<N: PhysicsNum> AxisAlignedRect<N> {
    pub fn new(dimensions: Vector2<N>) -> Self {
        Self { dimensions }
    }
    fn top_left(&self) -> Vector2<N> {
        vec2(Zero::zero(), Zero::zero())
    }
    fn top_right(&self) -> Vector2<N> {
        vec2(self.dimensions.x, Zero::zero())
    }
    fn bottom_left(&self) -> Vector2<N> {
        vec2(Zero::zero(), self.dimensions.y)
    }
    fn bottom_right(&self) -> Vector2<N> {
        self.dimensions
    }
    fn top(&self) -> LineSegment<N> {
        LineSegment::new(self.top_left(), self.top_right())
    }
    fn right(&self) -> LineSegment<N> {
        LineSegment::new(self.top_right(), self.bottom_right())
    }
    fn bottom(&self) -> LineSegment<N> {
        LineSegment::new(self.bottom_right(), self.bottom_left())
    }
    fn left(&self) -> LineSegment<N> {
        LineSegment::new(self.bottom_left(), self.top_left())
    }
    pub fn dimensions(&self) -> Vector2<N> {
        self.dimensions
    }
}

impl<N: PhysicsNum> Collide<N> for AxisAlignedRect<N> {
    fn aabb(&self, top_left: Vector2<N>) -> Aabb<N> {
        Aabb::new(top_left, self.dimensions)
    }
    fn for_each_vertex_facing<F>(&self, direction: Vector2<N>, mut f: F)
    where
        F: FnMut(Vector2<N>),
    {
        if direction.y >= Zero::zero() {
            f(self.bottom_left());
            f(self.bottom_right());
            if direction.x >= Zero::zero() {
                f(self.top_right());
            }
            if direction.x <= Zero::zero() {
                f(self.top_left());
            }
        }
        if direction.y <= Zero::zero() {
            f(self.top_left());
            f(self.top_right());
            if direction.x >= Zero::zero() {
                f(self.bottom_right());
            }
            if direction.x <= Zero::zero() {
                f(self.bottom_left());
            }
        }
    }
    fn for_each_edge_facing<F>(&self, direction: Vector2<N>, mut f: F)
    where
        F: FnMut(LineSegment<N>),
    {
        if direction.y >= Zero::zero() {
            f(self.bottom())
        }
        if direction.y <= Zero::zero() {
            f(self.top())
        }
        if direction.x >= Zero::zero() {
            f(self.right())
        }
        if direction.x <= Zero::zero() {
            f(self.left())
        }
    }
}
