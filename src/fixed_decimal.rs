use core::fmt;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    iter::Sum,
    ops::{
        Add, AddAssign, Div, DivAssign, Mul, MulAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
        SubAssign,
    },
};

#[derive(Clone, Copy, Eq)]
pub struct FixedDecimal<const DECIMALS: u32>(i128);

impl<const DECIMALS: u32> Serialize for FixedDecimal<DECIMALS> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de, const DECIMALS: u32> Deserialize<'de> for FixedDecimal<DECIMALS> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FixedDecimal::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl<const DECIMALS: u32> FixedDecimal<DECIMALS> {
    const fn scale() -> i128 {
        10i128.pow(DECIMALS)
    }

    pub fn from_integer(x: i128) -> Self {
        Self(x * Self::scale())
    }

    pub fn from_raw(x: i128) -> Self {
        Self(x)
    }

    pub fn min_positive() -> Self {
        Self::from_raw(1)
    }

    pub fn from_str(x: &str) -> Result<Self, &'static str> {
        let parts: Vec<&str> = x.split('.').collect();
        let integer_part = parts[0];
        let decimal_part = parts.get(1).unwrap_or(&"0");

        // Take only up to DECIMALS characters from the decimal part
        let decimal_part = if decimal_part.len() > DECIMALS as usize {
            &decimal_part[..DECIMALS as usize]
        } else {
            decimal_part
        };

        let mut result = Self::from_integer(
            integer_part
                .parse::<i128>()
                .map_err(|_| "Invalid integer part")?,
        );
        let scale = DECIMALS as i32 - decimal_part.len() as i32;
        let mut decimal_part = decimal_part
            .parse::<i128>()
            .map_err(|_| "Invalid decimal part")?;
        if scale > 0 {
            decimal_part *= 10i128.pow(scale as u32);
        } else if scale < 0 {
            decimal_part /= 10i128.pow(-scale as u32);
        }

        result.0 += decimal_part;
        Ok(result)
    }

    pub fn to_integer(self) -> i128 {
        self.0 / Self::scale()
    }

    pub fn to_float(self) -> f64 {
        self.0 as f64 / Self::scale() as f64
    }

    pub fn to_string(self) -> String {
        format!("{}.{}", self.to_integer(), self.0 % Self::scale())
    }

    pub fn add(self, right: Self) -> Self {
        Self(self.0 + right.0)
    }

    pub fn add_i128(self, right: i128) -> Self {
        Self(self.0 + right)
    }

    pub fn sub(self, right: Self) -> Self {
        Self(self.0 - right.0)
    }

    pub fn sub_i128(self, right: i128) -> Self {
        Self(self.0 - right)
    }

    pub fn mul(self, right: Self) -> Self {
        Self((self.0 * right.0) / Self::scale())
    }

    pub fn mul_i128(self, right: i128) -> Self {
        Self(self.0 * right)
    }

    pub fn div(self, right: Self) -> Self {
        Self(self.0 * Self::scale() / right.0)
    }

    pub fn squared(self) -> Self {
        Self(self.0 * self.0 / Self::scale())
    }

    pub fn raw_value(self) -> i128 {
        self.0
    }

    pub fn scale_const() -> i128 {
        Self::scale()
    }
}

impl<const DECIMALS: u32> fmt::Display for FixedDecimal<DECIMALS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_float())
    }
}

impl<const DECIMALS: u32> fmt::Debug for FixedDecimal<DECIMALS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.to_integer(), self.0 % Self::scale())
    }
}
impl<const DECIMALS: u32> Add for FixedDecimal<DECIMALS> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.add(rhs)
    }
}

impl<const DECIMALS: u32> Sub for FixedDecimal<DECIMALS> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.sub(rhs)
    }
}

impl<const DECIMALS: u32> Mul for FixedDecimal<DECIMALS> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        self.mul(rhs)
    }
}

impl<const DECIMALS: u32> Div for FixedDecimal<DECIMALS> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self.div(rhs)
    }
}

impl<const DECIMALS: u32> AddAssign for FixedDecimal<DECIMALS> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const DECIMALS: u32> SubAssign for FixedDecimal<DECIMALS> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<const DECIMALS: u32> MulAssign for FixedDecimal<DECIMALS> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<const DECIMALS: u32> DivAssign for FixedDecimal<DECIMALS> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<const DECIMALS: u32> PartialOrd<FixedDecimal<DECIMALS>> for FixedDecimal<DECIMALS> {
    fn partial_cmp(&self, other: &FixedDecimal<DECIMALS>) -> Option<Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl<const DECIMALS: u32> PartialEq<FixedDecimal<DECIMALS>> for FixedDecimal<DECIMALS> {
    fn eq(&self, other: &FixedDecimal<DECIMALS>) -> bool {
        self.0 == other.0
    }
}

macro_rules! impl_fixed_ops_with_primitive {
    ($($t:ty),*) => {
        $(
            impl<const DECIMALS: u32> Add<$t> for FixedDecimal<DECIMALS> {
                type Output = Self;
                fn add(self, rhs: $t) -> Self::Output {
                    self.add(FixedDecimal::from_integer(rhs as i128))
                }
            }

            impl<const DECIMALS: u32> Add<FixedDecimal<DECIMALS>> for $t {
                type Output = FixedDecimal<DECIMALS>;
                fn add(self, rhs: FixedDecimal<DECIMALS>) -> Self::Output {
                    FixedDecimal::from_integer(self as i128).add(rhs)
                }
            }

            impl<const DECIMALS: u32> Sub<$t> for FixedDecimal<DECIMALS> {
                type Output = Self;
                fn sub(self, rhs: $t) -> Self::Output {
                    self.sub(FixedDecimal::from_integer(rhs as i128))
                }
            }

            impl<const DECIMALS: u32> Sub<FixedDecimal<DECIMALS>> for $t {
                type Output = FixedDecimal<DECIMALS>;
                fn sub(self, rhs: FixedDecimal<DECIMALS>) -> Self::Output {
                    FixedDecimal::from_integer(self as i128).sub(rhs)
                }
            }

            impl<const DECIMALS: u32> Mul<$t> for FixedDecimal<DECIMALS> {
                type Output = Self;
                fn mul(self, rhs: $t) -> Self::Output {
                    self.mul_i128(rhs as i128)
                }
            }

            impl<const DECIMALS: u32> Mul<FixedDecimal<DECIMALS>> for $t {
                type Output = FixedDecimal<DECIMALS>;
                fn mul(self, rhs: FixedDecimal<DECIMALS>) -> Self::Output {
                    rhs.mul_i128(self as i128)
                }
            }

            impl<const DECIMALS: u32> Div<$t> for FixedDecimal<DECIMALS> {
                type Output = Self;
                fn div(self, rhs: $t) -> Self::Output {
                    self.div(FixedDecimal::from_integer(rhs as i128))
                }
            }

            impl<const DECIMALS: u32> Div<FixedDecimal<DECIMALS>> for $t {
                type Output = FixedDecimal<DECIMALS>;
                fn div(self, rhs: FixedDecimal<DECIMALS>) -> Self::Output {
                    FixedDecimal::from_integer(self as i128).div(rhs)
                }
            }
        )*
    };
}

impl_fixed_ops_with_primitive!(i128, i64, i32, usize, u64, u32);

macro_rules! impl_fixed_shift_ops {
    ($($t:ty),*) => {
        $(
            impl<const DECIMALS: u32> Shl<$t> for FixedDecimal<DECIMALS> {
                type Output = Self;
                fn shl(self, rhs: $t) -> Self::Output {
                    Self::from_raw(self.0 << rhs)
                }
            }

            impl<const DECIMALS: u32> Shr<$t> for FixedDecimal<DECIMALS> {
                type Output = Self;
                fn shr(self, rhs: $t) -> Self::Output {
                    Self::from_raw(self.0 >> rhs)
                }
            }

            impl<const DECIMALS: u32> ShlAssign<$t> for FixedDecimal<DECIMALS> {
                fn shl_assign(&mut self, rhs: $t) {
                    self.0 <<= rhs;
                }
            }

            impl<const DECIMALS: u32> ShrAssign<$t> for FixedDecimal<DECIMALS> {
                fn shr_assign(&mut self, rhs: $t) {
                    self.0 >>= rhs;
                }
            }
        )*
    };
}

impl_fixed_shift_ops!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

macro_rules! impl_fixed_comparisons {
    ($($t:ty),*) => {
        $(
            impl<const DECIMALS: u32> PartialEq<$t> for FixedDecimal<DECIMALS> {
                fn eq(&self, other: &$t) -> bool {
                    *self == FixedDecimal::from_integer(*other as i128)
                }
            }

            impl<const DECIMALS: u32> PartialEq<FixedDecimal<DECIMALS>> for $t {
                fn eq(&self, other: &FixedDecimal<DECIMALS>) -> bool {
                    FixedDecimal::from_integer(*self as i128) == *other
                }
            }

            impl<const DECIMALS: u32> PartialOrd<$t> for FixedDecimal<DECIMALS> {
                fn partial_cmp(&self, other: &$t) -> Option<std::cmp::Ordering> {
                    self.partial_cmp(&FixedDecimal::from_integer(*other as i128))
                }
            }

            impl<const DECIMALS: u32> PartialOrd<FixedDecimal<DECIMALS>> for $t {
                fn partial_cmp(&self, other: &FixedDecimal<DECIMALS>) -> Option<std::cmp::Ordering> {
                    FixedDecimal::from_integer(*self as i128).partial_cmp(other)
                }
            }
        )*
    };
}

impl_fixed_comparisons!(i128, i64, i32, isize, u128, u64, u32, usize);

impl<const DECIMALS: u32> Sum for FixedDecimal<DECIMALS> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(FixedDecimal::from_raw(0), |acc, x| acc + x)
    }
}

impl<'a, const DECIMALS: u32> Sum<&'a FixedDecimal<DECIMALS>> for FixedDecimal<DECIMALS> {
    fn sum<I: Iterator<Item = &'a FixedDecimal<DECIMALS>>>(iter: I) -> Self {
        iter.fold(FixedDecimal::from_raw(0), |acc, &x| acc + x)
    }
}
