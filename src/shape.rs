use aabb::Aabb;
use physics_num::{self, PhysicsNum};
use axis_aligned_rect::AxisAlignedRect;
use best::BestMap;
use cgmath::Vector2;
use collision::{self, Collision};
use line_segment::LineSegment;
use num::Zero;

fn for_each_single_direction_intersection<A, B, F, N>(
    shape: &A,
    position: Vector2<N>,
    other_shape: &B,
    other_position: Vector2<N>,
    movement: Vector2<N>,
    reverse_movement: Vector2<N>,
    f: &mut F,
) where
    N: PhysicsNum,
    A: Collide<N>,
    B: Collide<N>,
    F: FnMut(Collision<N>, LineSegment<N>),
{
    shape.for_each_vertex_facing(movement, |rel_vertex| {
        let abs_vertex = rel_vertex + position;
        other_shape.for_each_edge_facing(reverse_movement, |rel_edge| {
            let abs_edge = rel_edge.add_vector(other_position);
            match collision::vertex_moving_towards_edge(abs_vertex, movement, abs_edge) {
                Ok(collision) => f(collision, abs_edge),
                Err(_) => (),
            }
        });
    });
}

pub trait Collide<N: PhysicsNum> {
    fn aabb(&self, top_left: Vector2<N>) -> Aabb<N>;
    fn for_each_edge_facing<F: FnMut(LineSegment<N>)>(&self, direction: Vector2<N>, f: F);
    fn for_each_vertex_facing<F: FnMut(Vector2<N>)>(&self, direction: Vector2<N>, f: F);
    fn for_each_movement_intersection<StationaryShape, F>(
        &self,
        position: Vector2<N>,
        stationary_shape: &StationaryShape,
        stationary_position: Vector2<N>,
        movement: Vector2<N>,
        mut f: F,
    ) where
        Self: Sized,
        StationaryShape: Collide<N>,
        F: FnMut(Collision<N>, LineSegment<N>),
    {
        let reverse_movement = -movement;
        for_each_single_direction_intersection(
            self,
            position,
            stationary_shape,
            stationary_position,
            movement,
            reverse_movement,
            &mut f,
        );
        for_each_single_direction_intersection(
            stationary_shape,
            stationary_position,
            self,
            position,
            reverse_movement,
            movement,
            &mut f,
        );
    }
    fn movement_collision_test<StationaryShape>(
        &self,
        position: Vector2<N>,
        stationary_shape: &StationaryShape,
        stationary_position: Vector2<N>,
        movement: Vector2<N>,
    ) -> Option<CollisionInfo<N>>
    where
        Self: Sized,
        StationaryShape: Collide<N>,
    {
        let mut best_collision: BestMap<N, LineSegment<N>> = BestMap::new();
        self.for_each_movement_intersection(
            position,
            stationary_shape,
            stationary_position,
            movement,
            |collision, abs_edge| {
                let magnitude2 = match collision {
                    Collision::StartInsideEdge => Zero::zero(),
                    Collision::CollidesWithEdgeAfter(movement) => {
                        physics_num::magnitude2(movement)
                    }
                };
                best_collision.insert_le(magnitude2, abs_edge);
            },
        );
        if let Some((magnitude2, line_segment)) = best_collision.into_key_and_value() {
            Some(CollisionInfo {
                magnitude2,
                line_segment,
            })
        } else {
            None
        }
    }
}

pub struct CollisionInfo<N> {
    magnitude2: N,
    line_segment: LineSegment<N>,
}

#[derive(Debug, Clone)]
pub enum Shape<N: PhysicsNum> {
    AxisAlignedRect(AxisAlignedRect<N>),
    LineSegment(LineSegment<N>),
}

impl<N: PhysicsNum> Shape<N> {
    pub fn aabb(&self, top_left: Vector2<N>) -> Aabb<N> {
        match self {
            &Shape::AxisAlignedRect(ref rect) => rect.aabb(top_left),
            &Shape::LineSegment(ref line_segment) => line_segment.aabb(top_left),
        }
    }
    pub fn movement_collision_test(
        &self,
        position: Vector2<N>,
        stationary: &Self,
        stationary_position: Vector2<N>,
        movement_vector: Vector2<N>,
    ) -> Option<CollisionInfo<N>> {
        match self {
            &Shape::AxisAlignedRect(ref moving) => match stationary {
                &Shape::AxisAlignedRect(ref stationary) => moving
                    .movement_collision_test(
                        position,
                        stationary,
                        stationary_position,
                        movement_vector,
                    ),
                &Shape::LineSegment(ref stationary) => moving.movement_collision_test(
                    position,
                    stationary,
                    stationary_position,
                    movement_vector,
                ),
            },
            &Shape::LineSegment(_) => panic!(),
        }
    }
}
