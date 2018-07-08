use arith::{self, PhysicsNum};
use cgmath::{vec2, Vector2};
use line_segment::LineSegment;
use num::{One, Zero};

fn vector2_cross_product<N: PhysicsNum>(v: Vector2<N>, w: Vector2<N>) -> N {
    v.x * w.y - v.y * w.x
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Collision<N> {
    StartInsideEdge,
    CollidesWithEdgeAfter(Vector2<N>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoCollision {
    ColinearNonOverlapping,
    ParallelNonColinear,
    NonParallelNonIntersecting,
}

pub fn vertex_moving_towards_edge<N: PhysicsNum>(
    vertex: Vector2<N>,
    vertex_movement: Vector2<N>,
    edge: LineSegment<N>,
) -> Result<Collision<N>, NoCollision> {
    let edge_vector = edge.vector();
    let cross = vector2_cross_product(vertex_movement, edge_vector);
    let vertex_to_edge_start = edge.start - vertex;
    if cross.is_zero() {
        if vector2_cross_product(vertex_to_edge_start, vertex_movement).is_zero() {
            let mult_a_x_movement_len2 = arith::dot(vertex_to_edge_start, vertex_movement);
            let mult_b_x_movement_len2 =
                arith::dot(vertex_to_edge_start + edge_vector, vertex_movement);
            let (mult_min_x_movement_len2, mult_max_x_movement_len2) =
                if mult_a_x_movement_len2 < mult_b_x_movement_len2 {
                    (mult_a_x_movement_len2, mult_b_x_movement_len2)
                } else {
                    (mult_b_x_movement_len2, mult_a_x_movement_len2)
                };
            let movement_len2 = arith::magnitude2(vertex_movement);
            if mult_max_x_movement_len2 < Zero::zero() || mult_min_x_movement_len2 > movement_len2 {
                return Err(NoCollision::ColinearNonOverlapping);;
            }
            if mult_min_x_movement_len2 <= Zero::zero() {
                return Ok(Collision::StartInsideEdge);
            }
            if mult_min_x_movement_len2 <= movement_len2 {
                let allowed_vertex_movement = {
                    let allowed_movement_x_movement_len2 =
                        vertex_movement * mult_min_x_movement_len2;
                    let x = (allowed_movement_x_movement_len2.x - One::one()) / movement_len2;
                    let y = (allowed_movement_x_movement_len2.y - One::one()) / movement_len2;
                    vec2(x, y)
                };
                return Ok(Collision::CollidesWithEdgeAfter(allowed_vertex_movement));
            }
        }
        Err(NoCollision::ParallelNonColinear)
    } else {
        let cross_abs = cross.abs();
        let cross_sign = cross.signum();
        let vertex_multiplier_x_cross = vector2_cross_product(vertex_to_edge_start, edge_vector);
        let vertex_multiplier_x_cross_abs = vertex_multiplier_x_cross * cross_sign;
        if vertex_multiplier_x_cross_abs < Zero::zero() {
            return Err(NoCollision::NonParallelNonIntersecting);
        }
        if vertex_multiplier_x_cross_abs > cross_abs {
            return Err(NoCollision::NonParallelNonIntersecting);
        }
        let edge_multiplier_x_cross = vector2_cross_product(vertex_to_edge_start, vertex_movement);
        let edge_multiplier_x_cross_abs = edge_multiplier_x_cross * cross_sign;
        if edge_multiplier_x_cross_abs < Zero::zero() {
            return Err(NoCollision::NonParallelNonIntersecting);
        }
        if edge_multiplier_x_cross_abs > cross_abs {
            return Err(NoCollision::NonParallelNonIntersecting);
        }
        if vertex_multiplier_x_cross.is_zero() {
            return Ok(Collision::StartInsideEdge);
        }
        let movement_to_intersection_point_x_cross = vertex_movement * vertex_multiplier_x_cross;
        let allowed_vertex_movement = {
            let one = <N as One>::one() * cross_sign;
            let x = if movement_to_intersection_point_x_cross.x.is_zero() {
                Zero::zero()
            } else {
                (movement_to_intersection_point_x_cross.x - one) / cross
            };
            let y = if movement_to_intersection_point_x_cross.y.is_zero() {
                Zero::zero()
            } else {
                (movement_to_intersection_point_x_cross.y - one) / cross
            };
            vec2(x, y)
        };
        Ok(Collision::CollidesWithEdgeAfter(allowed_vertex_movement))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use cgmath::vec2;

    fn v(x: i64, y: i64) -> Vector2<i64> {
        vec2(x, y)
    }
    fn ls(start: Vector2<i64>, end: Vector2<i64>) -> LineSegment<i64> {
        LineSegment::new(start, end)
    }

    #[test]
    fn basic() {
        assert_eq!(
            vertex_moving_towards_edge(v(0, 0), v(3, 3), ls(v(0, 4), v(4, 0))),
            Ok(Collision::CollidesWithEdgeAfter(v(1, 1)))
        );
        assert_eq!(
            vertex_moving_towards_edge(v(0, 0), v(3, 3), ls(v(0, 5), v(5, 0))),
            Ok(Collision::CollidesWithEdgeAfter(v(2, 2)))
        );
        assert_eq!(
            vertex_moving_towards_edge(v(0, 0), v(2, 2), ls(v(0, 5), v(5, 0))),
            Err(NoCollision::NonParallelNonIntersecting)
        );
    }

    #[test]
    fn parallel() {
        assert_eq!(
            vertex_moving_towards_edge(v(0, 0), v(2, 1), ls(v(1, 1), v(3, 2))),
            Err(NoCollision::ParallelNonColinear)
        );
        assert_eq!(
            vertex_moving_towards_edge(v(0, 0), v(2, 1), ls(v(4, 2), v(8, 4))),
            Err(NoCollision::ColinearNonOverlapping)
        );
        assert_eq!(
            vertex_moving_towards_edge(v(0, 0), v(2, 1), ls(v(2, 1), v(8, 4))),
            Ok(Collision::CollidesWithEdgeAfter(v(1, 0)))
        );
        assert_eq!(
            vertex_moving_towards_edge(v(2, 1), v(2, 1), ls(v(0, 0), v(8, 4))),
            Ok(Collision::StartInsideEdge)
        );
    }

    #[test]
    fn perpendicular() {
        assert_eq!(
            vertex_moving_towards_edge(v(0, 0), v(10, 0), ls(v(5, 5), v(5, -5))),
            Ok(Collision::CollidesWithEdgeAfter(v(4, 0)))
        );
    }
}
