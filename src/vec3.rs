// portable Vec3 by GPT

use std::ops::{Add, Sub, Mul};
use num_traits::Float;

#[derive(Copy, Clone, Debug)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T>
where
    T: Float + Copy,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3 { x, y, z }
    }

    pub fn add(self, v: Self) -> Self {
        Vec3 {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }

    pub fn sub(self, v: Self) -> Self {
        Vec3 {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }

    pub fn mul(self, a: T) -> Self {
        Vec3 {
            x: self.x * a,
            y: self.y * a,
            z: self.z * a,
        }
    }

    pub fn normalize(self) -> Self {
        let sq_sum = self.x * self.x + self.y * self.y + self.z * self.z;
        if sq_sum == T::zero() {
            Vec3::new(T::zero(), T::zero(), T::zero())
        } else {
            let len = sq_sum.sqrt();
            self.mul(T::one() / len)
        }
    }
}

impl<T> Add for Vec3<T>
where
    T: Float + Copy,
{
    type Output = Self;
    fn add(self, other: Self) -> Self { self.add(other) }
}

impl<T> Sub for Vec3<T>
where
    T: Float + Copy,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self { self.sub(other) }
}

impl<T> Mul<T> for Vec3<T>
where
    T: Float + Copy,
{
    type Output = Self;
    fn mul(self, scalar: T) -> Self { self.mul(scalar) }
}
