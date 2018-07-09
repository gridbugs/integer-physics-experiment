use cgmath::{BaseNum, Vector2};
use num::{One, Signed, Zero};

pub trait PhysicsNum: BaseNum + ::std::ops::Neg<Output = Self> + Signed + Ord {
    fn two() -> Self {
        two()
    }
    fn clamp(self, min: Self, max: Self) -> Self {
        clamp(self, min, max)
    }
    fn clamp_zero_one(self) -> Self {
        clamp_zero_one(self)
    }
}

impl PhysicsNum for i64 {}

fn two<N: PhysicsNum>() -> N {
    <N as One>::one() + <N as One>::one()
}

pub fn magnitude2<N: PhysicsNum>(v: Vector2<N>) -> N {
    v.x * v.x + v.y * v.y
}

pub fn dot<N: PhysicsNum>(v: Vector2<N>, w: Vector2<N>) -> N {
    v.x * w.x + v.y * w.y
}

fn clamp<N: PhysicsNum>(v: N, min: N, max: N) -> N {
    v.max(min).min(max)
}

fn clamp_zero_one<N: PhysicsNum>(v: N) -> N {
    clamp(v, Zero::zero(), One::one())
}
