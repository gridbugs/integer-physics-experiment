macro_rules! make_i64_wrapper {
    ($name:ident) => {

        use num;

        custom_derive! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default,
                     NewtypeFrom, NewtypeAdd, NewtypeMul(i64), NewtypeMul,
                     NewtypeSub,  NewtypeRem, NewtypeDiv, NewtypeNeg,
                     NewtypeAddAssign, NewtypeSubAssign, NewtypeMulAssign,
                     NewtypeDivAssign, NewtypeRemAssign)]
            pub struct $name(i64);
        }

        impl num::Zero for $name {
            fn zero() -> Self {
                $name(0)
            }
            fn is_zero(&self) -> bool {
                self.0.is_zero()
            }
        }

        impl num::One for $name {
            fn one() -> Self {
                $name(1)
            }
            fn is_one(&self) -> bool {
                self.0.is_one()
            }
        }

        impl num::ToPrimitive for $name {
            fn to_i64(&self) -> Option<i64> {
                self.0.to_i64()
            }
            fn to_u64(&self) -> Option<u64> {
                self.0.to_u64()
            }
        }

        impl num::NumCast for $name {
            fn from<T>(n: T) -> Option<Self>
            where
                T: num::ToPrimitive,
            {
                n.to_i64().map($name)
            }
        }

        impl num::Num for $name {
            type FromStrRadixErr = <i64 as num::Num>::FromStrRadixErr;
            fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
                <i64 as num::Num>::from_str_radix(str, radix).map($name)
            }
        }

        impl num::Signed for $name {
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

pub mod sub_pixel_i64 {

    use physics_num::PhysicsNum;
    use super::*;
    use self::pixel_i64::PixelI64;
    use cgmath::{Vector2, vec2};
    make_i64_wrapper!(SubPixelI64);
    const SUB_PIXELS_PER_PIXEL: i64 = 256;
    const SUB_PIXELS_PER_PIXEL_SQUARED: i64 = 65536;
    impl SubPixelI64 {
        pub fn new_pixels_f32(pixels: f32) -> Self {
            SubPixelI64(pixels as i64 * SUB_PIXELS_PER_PIXEL)
        }
        pub fn approx_pixel(self) -> PixelI64 {
            PixelI64::new(self.0 / SUB_PIXELS_PER_PIXEL)
        }
        pub fn clamp_zero_one_pixel(self) -> Self {
            SubPixelI64(self.0.clamp(0, SUB_PIXELS_PER_PIXEL))
        }
    }
    pub fn normalize_vector_if_longer_than_one(
        v: Vector2<SubPixelI64>,
    ) -> Vector2<SubPixelI64> {
        let x2 = v.x.0 * v.x.0;
        let y2 = v.y.0 * v.y.0;
        let mag2 = x2 + y2;
        if mag2 > SUB_PIXELS_PER_PIXEL_SQUARED {
            let mag = (mag2 as f32).sqrt();
            let x = (v.x.0 * SUB_PIXELS_PER_PIXEL) as f32 / mag;
            let y = (v.y.0 * SUB_PIXELS_PER_PIXEL) as f32 / mag;
            vec2(SubPixelI64(x as i64), SubPixelI64(y as i64))
        } else {
            v
        }
    }
    pub fn vector_to_f32_pixel(v: Vector2<SubPixelI64>) -> Vector2<f32> {
        vec2(
            (v.x.0 / SUB_PIXELS_PER_PIXEL) as f32,
            (v.y.0 / SUB_PIXELS_PER_PIXEL) as f32,
        )
    }
}

pub mod pixel_i64 {

    use physics_num::PhysicsNum;
    make_i64_wrapper!(PixelI64);
}

pub use self::sub_pixel_i64::SubPixelI64;
pub use self::pixel_i64::PixelI64;
