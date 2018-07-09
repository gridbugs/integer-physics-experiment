use aabb::Aabb;
use physics_num::PhysicsNum;
use cgmath::{Vector2, vec2};
use shape::Collide;

#[derive(Debug, Clone, Copy)]
pub struct LineSegment<N> {
    pub start: Vector2<N>,
    pub end: Vector2<N>,
}

impl<N: PhysicsNum> LineSegment<N> {
    pub fn new(start: Vector2<N>, end: Vector2<N>) -> Self {
        Self { start, end }
    }
    pub fn add_vector(&self, vector: Vector2<N>) -> Self {
        Self {
            start: self.start + vector,
            end: self.end + vector,
        }
    }
    pub fn vector(&self) -> Vector2<N> {
        self.end - self.start
    }
}

impl<N: PhysicsNum> Collide<N> for LineSegment<N> {
    fn aabb(&self, top_left: Vector2<N>) -> Aabb<N> {
        let start = self.start + top_left;
        let end = self.end + top_left;
        let x_min = start.x.min(end.x);
        let x_max = start.x.max(end.x);
        let y_min = start.y.min(end.y);
        let y_max = start.y.max(end.y);
        let top_left = vec2(x_min, y_min);
        let bottom_right = vec2(x_max, y_max);
        Aabb::new(top_left, bottom_right - top_left)
    }
    fn for_each_edge_facing<F: FnMut(LineSegment<N>)>(
        &self,
        _direction: Vector2<N>,
        mut f: F,
    ) {
        f(*self);
    }
    fn for_each_vertex_facing<F: FnMut(Vector2<N>)>(
        &self,
        _direction: Vector2<N>,
        mut f: F,
    ) {
        f(self.start);
        f(self.end);
    }
}
