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

// implement the add and sub traits for the position types
impl Add for IntPos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for IntPos {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl Add for FloatPos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for FloatPos {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

//implement the add and sub traits for the size types
impl Add for IntSize {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for IntSize {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl Add for FloatSize {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for FloatSize {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

// implement the add and sub traits for Pos + Size, because it makes sense to add a size to a position
impl Add<IntSize> for IntPos {
    type Output = Self;

    fn add(self, other: IntSize) -> Self {
        Self(self.0 + other.0 as i32, self.1 + other.1 as i32)
    }
}

impl Sub<IntSize> for IntPos {
    type Output = Self;

    fn sub(self, other: IntSize) -> Self {
        Self(self.0 - other.0 as i32, self.1 - other.1 as i32)
    }
}

impl Add<FloatSize> for FloatPos {
    type Output = Self;

    fn add(self, other: FloatSize) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub<FloatSize> for FloatPos {
    type Output = Self;

    fn sub(self, other: FloatSize) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

// implement casting between the position types and the size types
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

/// The float positions and sizes compare with a tolerance, because they are the result of
/// layout arithmetic and asking two of those to be bit-equal is asking for a flicker.
///
/// **They deliberately do not implement `Hash`.** They used to, by quantising to a
/// thousandth, and that quietly broke the contract every hash container relies on: two
/// values 0.00005 apart are equal here but land either side of a bucket boundary and hash
/// differently, so a lookup could miss a key that is in the map. Nothing ever keyed a map on
/// one, so it was never hit - and now it cannot be.
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
