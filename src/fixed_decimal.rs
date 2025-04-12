use core::fmt;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    iter::Sum,
    ops::{
        Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Shl, ShlAssign, Shr, ShrAssign, Sub,
        SubAssign,
    },
};

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct FixedDecimal<const DECIMALS: u32>(i128);

const fn scale_raw(raw: i128, scale_index: i32) -> i128 {
    if scale_index > 0 {
        raw * 10i128.pow(scale_index as u32)
    } else if scale_index < 0 {
        raw / 10i128.pow(-scale_index as u32)
    } else {
        raw
    }
}

impl<const DECIMALS: u32> FixedDecimal<DECIMALS> {
    const fn scale() -> i128 {
        10i128.pow(DECIMALS)
    }

    pub const fn zero() -> Self {
        Self(0)
    }

    pub const fn one() -> Self {
        Self(Self::scale())
    }

    pub const fn ln2() -> Self {
        let ln2_raw = 693147180559945309417232121458;
        let ln2_raw_length = 30;
        let scale_decimals = DECIMALS as i32 - ln2_raw_length;
        Self(scale_raw(ln2_raw, scale_decimals))
    }

    pub const fn e() -> Self {
        let e_raw = 2718281828459045235360287471352;
        let e_raw_length = 30;
        let scale_decimals = DECIMALS as i32 - e_raw_length;
        Self(scale_raw(e_raw, scale_decimals))
    }

    pub const fn pi() -> Self {
        let pi_raw = 3141592653589793238462643383279;
        let pi_raw_length = 30;
        let scale_decimals = DECIMALS as i32 - pi_raw_length;
        Self(scale_raw(pi_raw, scale_decimals))
    }

    pub fn two_pow_k(k: i32) -> Self {
        if k > 0 {
            FixedDecimal::one() << k
        } else if k < 0 {
            FixedDecimal::one() >> -k
        } else {
            FixedDecimal::one()
        }
    }

    pub fn floor(self) -> Self {
        Self(self.0 / Self::scale() * Self::scale())
    }

    pub fn floor_i128(self) -> i128 {
        self.0 / Self::scale()
    }

    pub fn from_i128(x: i128) -> Self {
        Self(x * Self::scale())
    }

    pub fn from_raw(x: i128) -> Self {
        Self(x)
    }

    pub fn min_positive() -> Self {
        Self::from_raw(1)
    }

    pub fn from_str(x: &str) -> Result<Self, &'static str> {
        let is_negative = x.starts_with('-');
        let x = if is_negative { &x[1..] } else { x };

        let parts: Vec<&str> = x.split('.').collect();
        let integer_part = parts[0];
        let decimal_part = parts.get(1).unwrap_or(&"0");

        let decimal_part = if decimal_part.len() > DECIMALS as usize {
            &decimal_part[..DECIMALS as usize]
        } else {
            decimal_part
        };

        let mut result = Self::from_i128(
            integer_part
                .parse::<i128>()
                .map_err(|_| "Invalid integer part")?,
        );

        let scale = DECIMALS as i32 - decimal_part.len() as i32;
        let mut decimal_value = decimal_part
            .parse::<i128>()
            .map_err(|_| "Invalid decimal part")?;
        if scale > 0 {
            decimal_value *= 10i128.pow(scale as u32);
        } else if scale < 0 {
            decimal_value /= 10i128.pow(-scale as u32);
        }

        result.0 += decimal_value;

        if is_negative {
            result.0 = -result.0;
        }

        Ok(result)
    }

    pub fn to_i128(self) -> i128 {
        self.0 / Self::scale()
    }

    pub fn to_float(self) -> f64 {
        self.0 as f64 / Self::scale() as f64
    }

    pub fn to_string(self) -> String {
        let decimal = self.0.abs() % Self::scale();
        let decimal_string = decimal.to_string();
        let decimal_str = decimal_string.trim_end_matches('0');

        if decimal_str.is_empty() {
            format!("{}", self.to_i128())
        } else {
            format!("{}.{}", self.to_i128(), decimal_str)
        }
    }

    pub fn neg(self) -> Self {
        Self(-self.0)
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

    pub fn cubed(self) -> Self {
        Self(self.0 * self.0 / Self::scale() * self.0 / Self::scale())
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
        write!(f, "FixedDecimal<{}>({})", DECIMALS, self.to_string())
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

impl<const DECIMALS: u32> Neg for FixedDecimal<DECIMALS> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.neg()
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

impl<const DECIMALS: u32> Ord for FixedDecimal<DECIMALS> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

macro_rules! impl_fixed_ops_with_primitive {
    ($($t:ty),*) => {
        $(
            impl<const DECIMALS: u32> Add<$t> for FixedDecimal<DECIMALS> {
                type Output = Self;
                fn add(self, rhs: $t) -> Self::Output {
                    self.add(FixedDecimal::from_i128(rhs as i128))
                }
            }

            impl<const DECIMALS: u32> AddAssign<$t> for FixedDecimal<DECIMALS> {
                fn add_assign(&mut self, rhs: $t) {
                    *self = *self + rhs;
                }
            }

            impl<const DECIMALS: u32> Add<FixedDecimal<DECIMALS>> for $t {
                type Output = FixedDecimal<DECIMALS>;
                fn add(self, rhs: FixedDecimal<DECIMALS>) -> Self::Output {
                    FixedDecimal::from_i128(self as i128).add(rhs)
                }
            }

            impl<const DECIMALS: u32> Sub<$t> for FixedDecimal<DECIMALS> {
                type Output = Self;
                fn sub(self, rhs: $t) -> Self::Output {
                    self.sub(FixedDecimal::from_i128(rhs as i128))
                }
            }

            impl<const DECIMALS: u32> SubAssign<$t> for FixedDecimal<DECIMALS> {
                fn sub_assign(&mut self, rhs: $t) {
                    *self = *self - rhs;
                }
            }

            impl<const DECIMALS: u32> Sub<FixedDecimal<DECIMALS>> for $t {
                type Output = FixedDecimal<DECIMALS>;
                fn sub(self, rhs: FixedDecimal<DECIMALS>) -> Self::Output {
                    FixedDecimal::from_i128(self as i128).sub(rhs)
                }
            }

            impl<const DECIMALS: u32> Mul<$t> for FixedDecimal<DECIMALS> {
                type Output = Self;
                fn mul(self, rhs: $t) -> Self::Output {
                    self.mul_i128(rhs as i128)
                }
            }

            impl<const DECIMALS: u32> MulAssign<$t> for FixedDecimal<DECIMALS> {
                fn mul_assign(&mut self, rhs: $t) {
                    *self = *self * rhs;
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
                    self.div(FixedDecimal::from_i128(rhs as i128))
                }
            }

            impl<const DECIMALS: u32> DivAssign<$t> for FixedDecimal<DECIMALS> {
                fn div_assign(&mut self, rhs: $t) {
                    *self = *self / rhs;
                }
            }

            impl<const DECIMALS: u32> Div<FixedDecimal<DECIMALS>> for $t {
                type Output = FixedDecimal<DECIMALS>;
                fn div(self, rhs: FixedDecimal<DECIMALS>) -> Self::Output {
                    FixedDecimal::from_i128(self as i128).div(rhs)
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
                    *self == FixedDecimal::from_i128(*other as i128)
                }
            }

            impl<const DECIMALS: u32> PartialEq<FixedDecimal<DECIMALS>> for $t {
                fn eq(&self, other: &FixedDecimal<DECIMALS>) -> bool {
                    FixedDecimal::from_i128(*self as i128) == *other
                }
            }

            impl<const DECIMALS: u32> PartialOrd<$t> for FixedDecimal<DECIMALS> {
                fn partial_cmp(&self, other: &$t) -> Option<std::cmp::Ordering> {
                    self.partial_cmp(&FixedDecimal::from_i128(*other as i128))
                }
            }

            impl<const DECIMALS: u32> PartialOrd<FixedDecimal<DECIMALS>> for $t {
                fn partial_cmp(&self, other: &FixedDecimal<DECIMALS>) -> Option<std::cmp::Ordering> {
                    FixedDecimal::from_i128(*self as i128).partial_cmp(other)
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
