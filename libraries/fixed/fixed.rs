use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use serde_derive::{Deserialize, Serialize};

/// How many of the 32 bits are fraction.
pub const FRACTIONAL_BITS: u32 = 16;
/// The raw value of 1.0. Also the number of representable steps per whole unit.
pub const ONE_RAW: i64 = 1 << FRACTIONAL_BITS;

/// Narrow to `i32` by saturating. Wrapping here would put a runaway value at the opposite end
/// of the range, which in a simulation reads as a teleport rather than as the overflow it is.
const fn narrow(value: i64) -> i32 {
    if value > i32::MAX as i64 {
        i32::MAX
    } else if value < i32::MIN as i64 {
        i32::MIN
    } else {
        value as i32
    }
}

/// A signed 16.16 fixed-point number. See the module docs for why this exists.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct Fixed(i32);

impl Fixed {
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(ONE_RAW as i32);
    pub const MIN: Self = Self(i32::MIN);
    pub const MAX: Self = Self(i32::MAX);
    /// The smallest representable step. Anything smaller truncates to zero, which is what
    /// lets a decaying value reach rest instead of approaching it forever.
    pub const EPSILON: Self = Self(1);

    /// Wraps a raw 16.16 value. For deserialization and for tests that pin exact bits.
    #[must_use]
    pub const fn from_raw(raw: i32) -> Self {
        Self(raw)
    }

    #[must_use]
    pub const fn raw(self) -> i32 {
        self.0
    }

    #[must_use]
    pub const fn from_int(value: i32) -> Self {
        Self(narrow(value as i64 * ONE_RAW))
    }

    /// An exact ratio, for constants. `from_num(1, 200)` is the nearest representable 0.005
    /// without a float ever being involved, so the value is the same one on every platform
    /// and can be written in a `const`.
    #[must_use]
    pub const fn from_num(numerator: i32, denominator: i32) -> Self {
        if denominator == 0 {
            return Self::ZERO;
        }
        Self(narrow(numerator as i64 * ONE_RAW / denominator as i64))
    }

    /// Rounds toward zero, like `as i32` on a float.
    #[must_use]
    pub const fn to_int(self) -> i32 {
        (self.0 as i64 / ONE_RAW) as i32
    }

    /// Rounds toward negative infinity. This is the one to index a grid with: cell -1 starts
    /// at -1.0, so truncating toward zero would put everything in (-1, 0) in cell 0.
    #[must_use]
    pub const fn floor_to_int(self) -> i32 {
        self.0 >> FRACTIONAL_BITS
    }

    /// Rounds toward positive infinity.
    #[must_use]
    pub const fn ceil_to_int(self) -> i32 {
        -((-self.0) >> FRACTIONAL_BITS)
    }

    #[must_use]
    pub const fn abs(self) -> Self {
        if self.0 < 0 {
            Self(self.0.saturating_neg())
        } else {
            self
        }
    }

    #[must_use]
    pub const fn is_negative(self) -> bool {
        self.0 < 0
    }

    /// Square root, computed in integers so it is exact everywhere. Negative inputs give zero
    /// rather than a NaN, there being no such value here to propagate.
    #[must_use]
    pub const fn sqrt(self) -> Self {
        if self.0 <= 0 {
            return Self::ZERO;
        }
        // sqrt(raw / ONE) * ONE == sqrt(raw * ONE), and raw * ONE is at most 2^47.
        Self(narrow(((self.0 as u64) << FRACTIONAL_BITS).isqrt() as i64))
    }

    /// For rendering and for handing a number to lua. Never call this on the way *in* to the
    /// simulation - the point of the type is that the simulation holds no floats.
    #[must_use]
    pub fn to_f32(self) -> f32 {
        self.0 as f32 / ONE_RAW as f32
    }

    /// For values arriving from lua or from a settings file, converted once at the boundary.
    /// Deterministic given the same input, since the conversion is a multiply and a truncation.
    #[must_use]
    pub fn from_f32(value: f32) -> Self {
        if value.is_nan() {
            return Self::ZERO;
        }
        Self(narrow((value * ONE_RAW as f32) as i64))
    }
}

impl Add for Fixed {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0.saturating_add(rhs.0))
    }
}

impl Sub for Fixed {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0.saturating_sub(rhs.0))
    }
}

impl Neg for Fixed {
    type Output = Self;
    fn neg(self) -> Self {
        Self(self.0.saturating_neg())
    }
}

impl Mul for Fixed {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        // `/` and not `>>`: the shift would floor, biasing every product towards negative
        // infinity, so a leftward velocity would decay faster than the same rightward one.
        Self(narrow(self.0 as i64 * rhs.0 as i64 / ONE_RAW))
    }
}

impl Div for Fixed {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        if rhs.0 == 0 {
            // Saturating rather than panicking: a division by zero in a simulation step is a
            // bug in the caller, but taking the process down mid-frame is worse than pinning.
            return match self.0.signum() {
                1 => Self::MAX,
                -1 => Self::MIN,
                _ => Self::ZERO,
            };
        }
        Self(narrow((self.0 as i64 * ONE_RAW) / rhs.0 as i64))
    }
}

/// Scaling by a whole number is exact and much the commonest case - a per-tick step is a
/// velocity divided by the number of ticks in a second - so it does not go through `Fixed`.
impl Mul<i32> for Fixed {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self {
        Self(narrow(self.0 as i64 * rhs as i64))
    }
}

impl Div<i32> for Fixed {
    type Output = Self;
    fn div(self, rhs: i32) -> Self {
        if rhs == 0 {
            return match self.0.signum() {
                1 => Self::MAX,
                -1 => Self::MIN,
                _ => Self::ZERO,
            };
        }
        Self(self.0 / rhs)
    }
}

impl AddAssign for Fixed {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for Fixed {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign for Fixed {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl DivAssign for Fixed {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl fmt::Display for Fixed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.4}", self.to_f32())
    }
}

impl fmt::Debug for Fixed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fixed({})", self.to_f32())
    }
}
