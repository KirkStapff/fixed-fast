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

pub trait FixedPrecision: Copy + Eq {
    const PRECISION: u32;
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct FixedDecimal<T: FixedPrecision>(i128, std::marker::PhantomData<T>);

const fn scale_raw(raw: i128, scale_index: i32) -> i128 {
    if scale_index > 0 {
        raw * 10i128.pow(scale_index as u32)
    } else if scale_index < 0 {
        raw / 10i128.pow(-scale_index as u32)
    } else {
        raw
    }
}

impl<T: FixedPrecision> FixedDecimal<T> {
    pub const fn scale() -> i128 {
        10i128.pow(T::PRECISION)
    }

    pub const fn zero() -> Self {
        Self(0, std::marker::PhantomData)
    }

    pub const fn one() -> Self {
        Self(Self::scale(), std::marker::PhantomData)
    }

    pub const fn ln2() -> Self {
        let ln2_raw = 693147180559945309417232121458;
        let ln2_raw_length = 30;
        let scale_decimals = T::PRECISION as i32 - ln2_raw_length;
        Self(scale_raw(ln2_raw, scale_decimals), std::marker::PhantomData)
    }

    pub const fn e() -> Self {
        let e_raw = 2718281828459045235360287471352;
        let e_raw_length = 30;
        let scale_decimals = T::PRECISION as i32 - e_raw_length;
        Self(scale_raw(e_raw, scale_decimals), std::marker::PhantomData)
    }

    pub const fn pi() -> Self {
        let pi_raw = 3141592653589793238462643383279;
        let pi_raw_length = 30;
        let scale_decimals = T::PRECISION as i32 - pi_raw_length;
        Self(scale_raw(pi_raw, scale_decimals), std::marker::PhantomData)
    }

    pub fn two_pow_k(k: i32) -> Self {
        println!("k: {}", k);
        if k > 0 {
            FixedDecimal::one() << k
        } else if k < 0 {
            FixedDecimal::one() >> -k
        } else {
            FixedDecimal::one()
        }
    }

    pub fn floor(self) -> Self {
        Self::from_raw(self.0 / Self::scale() * Self::scale())
    }

    pub fn floor_i128(self) -> i128 {
        self.0 / Self::scale()
    }

    pub fn from_i128(x: i128) -> Self {
        Self(x * Self::scale(), std::marker::PhantomData)
    }

    pub fn from_raw(x: i128) -> Self {
        Self(x, std::marker::PhantomData)
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

        let decimal_part = if decimal_part.len() > T::PRECISION as usize {
            &decimal_part[..T::PRECISION as usize]
        } else {
            decimal_part
        };

        let mut result = Self::from_i128(
            integer_part
                .parse::<i128>()
                .map_err(|_| "Invalid integer part")?,
        );

        let scale = T::PRECISION as i32 - decimal_part.len() as i32;
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

    pub fn to_raw(&self) -> i128 {
        self.0
    }

    pub fn to_i128(&self) -> i128 {
        self.0 / Self::scale()
    }

    pub fn to_f64(&self) -> f64 {
        self.0 as f64 / Self::scale() as f64
    }

    pub fn to_string(&self) -> String {
        let decimal = self.0.abs() % Self::scale();
        let decimal_string = decimal.to_string();
        let decimal_str = decimal_string.trim_end_matches('0');

        if decimal_str.is_empty() {
            format!("{}", self.to_i128())
        } else {
            format!("{}.{}", self.to_i128(), decimal_str)
        }
    }

    pub fn neg(&self) -> Self {
        Self::from_raw(-self.0)
    }

    pub fn add(&self, right: Self) -> Self {
        Self::from_raw(self.0 + right.0)
    }

    pub fn add_i128(&self, right: i128) -> Self {
        Self::from_raw(self.0 + right * Self::scale())
    }

    pub fn sub(&self, right: Self) -> Self {
        Self::from_raw(self.0 - right.0)
    }

    pub fn sub_i128(&self, right: i128) -> Self {
        Self::from_raw(self.0 - right * Self::scale())
    }

    pub fn mul(&self, right: Self) -> Self {
        Self::from_raw((self.0 * right.0) / Self::scale())
    }

    pub fn mul_i128(&self, right: i128) -> Self {
        Self::from_raw(self.0 * right)
    }

    pub fn div(&self, right: Self) -> Self {
        Self::from_raw(self.0 * Self::scale() / right.0)
    }

    pub fn div_i128(&self, right: i128) -> Self {
        Self::from_raw(self.0 / right)
    }

    pub fn squared(&self) -> Self {
        Self::from_raw(self.0 * self.0 / Self::scale())
    }

    pub fn cubed(&self) -> Self {
        Self::from_raw(self.0 * self.0 / Self::scale() * self.0 / Self::scale())
    }

    pub fn tesseracted(&self) -> Self {
        Self::from_raw(
            self.0 * self.0 / Self::scale() * self.0 / Self::scale() * self.0 / Self::scale(),
        )
    }

    pub fn abs(&self) -> Self {
        Self::from_raw(self.0.abs())
    }
}

impl<T: FixedPrecision> fmt::Display for FixedDecimal<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_f64())
    }
}

impl<T: FixedPrecision> fmt::Debug for FixedDecimal<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FixedDecimal<{}>({})", T::PRECISION, self.to_string())
    }
}

impl<T: FixedPrecision> Add for FixedDecimal<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_raw(self.0 + rhs.0)
    }
}

impl<T: FixedPrecision> Sub for FixedDecimal<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_raw(self.0 - rhs.0)
    }
}

impl<T: FixedPrecision> Mul for FixedDecimal<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::from_raw((self.0 * rhs.0) / Self::scale())
    }
}

impl<T: FixedPrecision> Div for FixedDecimal<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::from_raw(self.0 * Self::scale() / rhs.0)
    }
}

impl<T: FixedPrecision> Neg for FixedDecimal<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::from_raw(-self.0)
    }
}

impl<T: FixedPrecision> AddAssign for FixedDecimal<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.add(rhs).0;
    }
}

impl<T: FixedPrecision> SubAssign for FixedDecimal<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = self.sub(rhs).0;
    }
}

impl<T: FixedPrecision> MulAssign for FixedDecimal<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = self.mul(rhs).0;
    }
}

impl<T: FixedPrecision> DivAssign for FixedDecimal<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.0 = self.div(rhs).0;
    }
}

impl<T: FixedPrecision> PartialOrd<FixedDecimal<T>> for FixedDecimal<T> {
    fn partial_cmp(&self, other: &FixedDecimal<T>) -> Option<Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl<T: FixedPrecision> Ord for FixedDecimal<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

macro_rules! impl_fixed_ops_with_primitive {
    ($($t:ty),*) => {
        $(
            impl<T: FixedPrecision> Add<$t> for FixedDecimal<T> {
                type Output = Self;
                fn add(self, rhs: $t) -> Self::Output {
                    self.add(FixedDecimal::from_i128(rhs as i128))
                }
            }

            impl<T: FixedPrecision> AddAssign<$t> for FixedDecimal<T> {
                fn add_assign(&mut self, rhs: $t) {
                    *self = self.add_i128(rhs as i128);
                }
            }

            impl<T: FixedPrecision> Add<FixedDecimal<T>> for $t {
                type Output = FixedDecimal<T>;
                fn add(self, rhs: FixedDecimal<T>) -> Self::Output {
                    FixedDecimal::from_i128(self as i128).add(rhs)
                }
            }

            impl<T: FixedPrecision> Sub<$t> for FixedDecimal<T> {
                type Output = Self;
                fn sub(self, rhs: $t) -> Self::Output {
                    self.sub(FixedDecimal::from_i128(rhs as i128))
                }
            }

            impl<T: FixedPrecision> SubAssign<$t> for FixedDecimal<T> {
                fn sub_assign(&mut self, rhs: $t) {
                    *self = self.sub_i128(rhs as i128);
                }
            }

            impl<T: FixedPrecision> Sub<FixedDecimal<T>> for $t {
                type Output = FixedDecimal<T>;
                fn sub(self, rhs: FixedDecimal<T>) -> Self::Output {
                    FixedDecimal::from_i128(self as i128).sub(rhs)
                }
            }

            impl<T: FixedPrecision> Mul<$t> for FixedDecimal<T> {
                type Output = Self;
                fn mul(self, rhs: $t) -> Self::Output {
                    self.mul_i128(rhs as i128)
                }
            }

            impl<T: FixedPrecision> MulAssign<$t> for FixedDecimal<T> {
                fn mul_assign(&mut self, rhs: $t) {
                    *self = self.mul_i128(rhs as i128);
                }
            }

            impl<T: FixedPrecision> Mul<FixedDecimal<T>> for $t {
                type Output = FixedDecimal<T>;
                fn mul(self, rhs: FixedDecimal<T>) -> Self::Output {
                    rhs.mul_i128(self as i128)
                }
            }

            impl<T: FixedPrecision> Div<$t> for FixedDecimal<T> {
                type Output = Self;
                fn div(self, rhs: $t) -> Self::Output {
                    self.div_i128(rhs as i128)
                }
            }

            impl<T: FixedPrecision> DivAssign<$t> for FixedDecimal<T> {
                fn div_assign(&mut self, rhs: $t) {
                    *self = self.div_i128(rhs as i128);
                }
            }

            impl<T: FixedPrecision> Div<FixedDecimal<T>> for $t {
                type Output = FixedDecimal<T>;
                fn div(self, rhs: FixedDecimal<T>) -> Self::Output {
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
            impl<T: FixedPrecision> Shl<$t> for FixedDecimal<T> {
                type Output = Self;
                fn shl(self, rhs: $t) -> Self::Output {
                    Self::from_raw(self.0 << rhs)
                }
            }

            impl<T: FixedPrecision> Shr<$t> for FixedDecimal<T> {
                type Output = Self;
                fn shr(self, rhs: $t) -> Self::Output {
                    Self::from_raw(self.0 >> rhs)
                }
            }

            impl<T: FixedPrecision> ShlAssign<$t> for FixedDecimal<T> {
                fn shl_assign(&mut self, rhs: $t) {
                    self.0 <<= rhs;
                }
            }

            impl<T: FixedPrecision> ShrAssign<$t> for FixedDecimal<T> {
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
            impl<T: FixedPrecision> PartialEq<$t> for FixedDecimal<T> {
                fn eq(&self, other: &$t) -> bool {
                    self.0 == (*other as i128) * Self::scale()
                }
            }

            impl<T: FixedPrecision> PartialEq<FixedDecimal<T>> for $t {
                fn eq(&self, other: &FixedDecimal<T>) -> bool {
                    (*self as i128) * FixedDecimal::<T>::scale() == other.0
                }
            }

            impl<T: FixedPrecision> PartialOrd<$t> for FixedDecimal<T> {
                fn partial_cmp(&self, other: &$t) -> Option<std::cmp::Ordering> {
                    Some(self.0.cmp(&((*other as i128) * Self::scale())))
                }
            }

            impl<T: FixedPrecision> PartialOrd<FixedDecimal<T>> for $t {
                fn partial_cmp(&self, other: &FixedDecimal<T>) -> Option<std::cmp::Ordering> {
                    Some(((*self as i128) * FixedDecimal::<T>::scale()).cmp(&other.0))
                }
            }
        )*
    };
}

impl_fixed_comparisons!(i128, i64, i32, isize, u128, u64, u32, usize);

impl<T: FixedPrecision> Sum for FixedDecimal<T> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(FixedDecimal::from_raw(0), |acc, x| acc + x)
    }
}

impl<'a, T: FixedPrecision> Sum<&'a FixedDecimal<T>> for FixedDecimal<T> {
    fn sum<I: Iterator<Item = &'a FixedDecimal<T>>>(iter: I) -> Self {
        let mut result = FixedDecimal::<T>::from_raw(0);
        for x in iter {
            result = result + x;
        }
        result
    }
}

impl<T: FixedPrecision> Serialize for FixedDecimal<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de, T: FixedPrecision> Deserialize<'de> for FixedDecimal<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FixedDecimal::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl<'a, T: FixedPrecision> Add<&'a FixedDecimal<T>> for FixedDecimal<T> {
    type Output = Self;
    fn add(self, rhs: &'a FixedDecimal<T>) -> Self::Output {
        Self::from_raw(self.0 + rhs.0)
    }
}
