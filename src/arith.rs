use cgmath::{BaseNum, Vector2};
use num::{Num, NumCast, One, Signed, ToPrimitive, Zero};

pub trait PhysicsNum: BaseNum + ::std::ops::Neg<Output = Self> + Signed + Ord {}

impl PhysicsNum for i64 {}

macro_rules! make_i64_wrapper {
    ($name:ident) => {
        custom_derive! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
                     NewtypeFrom, NewtypeAdd, NewtypeMul(i64), NewtypeMul,
                     NewtypeSub,  NewtypeRem, NewtypeDiv, NewtypeNeg,
                     NewtypeAddAssign, NewtypeSubAssign, NewtypeMulAssign,
                     NewtypeDivAssign, NewtypeRemAssign)]
            pub struct $name(i64);
        }

        impl Zero for $name {
            fn zero() -> Self {
                $name(0)
            }
            fn is_zero(&self) -> bool {
                self.0.is_zero()
            }
        }

        impl One for $name {
            fn one() -> Self {
                $name(1)
            }
            fn is_one(&self) -> bool {
                self.0.is_one()
            }
        }

        impl ToPrimitive for $name {
            fn to_i64(&self) -> Option<i64> {
                self.0.to_i64()
            }
            fn to_u64(&self) -> Option<u64> {
                self.0.to_u64()
            }
        }

        impl NumCast for $name {
            fn from<T>(n: T) -> Option<Self>
            where
                T: ToPrimitive,
            {
                n.to_i64().map($name)
            }
        }

        impl Num for $name {
            type FromStrRadixErr = <i64 as Num>::FromStrRadixErr;
            fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
                <i64 as Num>::from_str_radix(str, radix).map($name)
            }
        }

        impl Signed for $name {
            fn abs(&self) -> Self {
                $name(self.0.abs())
            }
            fn abs_sub(&self, other: &Self) -> Self {
                $name(self.0.abs_sub(&other.0))
            }
            fn signum(&self) -> Self {
                $name(self.0.signum())
            }
            fn is_positive(&self) -> bool {
                self.0.is_positive()
            }
            fn is_negative(&self) -> bool {
                self.0.is_negative()
            }
        }

        impl PhysicsNum for $name {}

        impl $name {
            pub fn new(value: i64) -> Self {
                $name(value)
            }
        }
    };
}

make_i64_wrapper!(PixelI64);
make_i64_wrapper!(SubPixelI64);

pub fn two<N: PhysicsNum>() -> N {
    <N as One>::one() + <N as One>::one()
}

pub fn magnitude2<N: PhysicsNum>(v: Vector2<N>) -> N {
    v.x * v.x + v.y * v.y
}

pub fn dot<N: PhysicsNum>(v: Vector2<N>, w: Vector2<N>) -> N {
    v.x * w.x + v.y * w.y
}
