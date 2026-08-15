use std::ops::{Add, Sub};

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct IntPos(pub i32, pub i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct IntSize(pub u32, pub u32);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FloatPos(pub f32, pub f32);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FloatSize(pub f32, pub f32);

/// Component-wise `+` and `-` between two values of the same type.
macro_rules! impl_arithmetic {
    ($type:ty) => {
        impl Add for $type {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                Self(self.0 + other.0, self.1 + other.1)
            }
        }

        impl Sub for $type {
            type Output = Self;

            fn sub(self, other: Self) -> Self {
                Self(self.0 - other.0, self.1 - other.1)
            }
        }
    };
}

/// The same, for offsetting a position by a size.
macro_rules! impl_offset_by {
    ($pos:ty, $size:ty, $cast:ty) => {
        impl Add<$size> for $pos {
            type Output = Self;

            fn add(self, other: $size) -> Self {
                Self(self.0 + other.0 as $cast, self.1 + other.1 as $cast)
            }
        }

        impl Sub<$size> for $pos {
            type Output = Self;

            fn sub(self, other: $size) -> Self {
                Self(self.0 - other.0 as $cast, self.1 - other.1 as $cast)
            }
        }
    };
}

impl_arithmetic!(IntPos);
impl_arithmetic!(IntSize);
impl_arithmetic!(FloatPos);
impl_arithmetic!(FloatSize);
impl_offset_by!(IntPos, IntSize, i32);
impl_offset_by!(FloatPos, FloatSize, f32);

impl From<IntPos> for FloatPos {
    fn from(pos: IntPos) -> Self {
        Self(pos.0 as f32, pos.1 as f32)
    }
}

impl From<FloatPos> for IntPos {
    fn from(pos: FloatPos) -> Self {
        Self(pos.0 as i32, pos.1 as i32)
    }
}

impl From<IntSize> for FloatSize {
    fn from(size: IntSize) -> Self {
        Self(size.0 as f32, size.1 as f32)
    }
}

impl From<FloatSize> for IntSize {
    fn from(size: FloatSize) -> Self {
        Self(size.0 as u32, size.1 as u32)
    }
}

/// Float positions and sizes compare with a tolerance: they come out of layout arithmetic, and
/// asking two of those to be bit-equal is asking for a flicker. **They deliberately do not
/// implement `Hash`** - two values a hair apart are equal here and would hash differently,
/// which breaks the contract. Round to integers before keying a map on one.
impl PartialEq for FloatPos {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < 0.0001 && (self.1 - other.1).abs() < 0.0001
    }
}

impl Eq for FloatPos {}

impl PartialEq for FloatSize {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < 0.0001 && (self.1 - other.1).abs() < 0.0001
    }
}

impl Eq for FloatSize {}
