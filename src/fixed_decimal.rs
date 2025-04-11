use core::fmt;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

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

    pub fn sub(self, right: Self) -> Self {
        Self(self.0 - right.0)
    }

    pub fn mul(self, right: Self) -> Self {
        Self((self.0 * right.0) / Self::scale())
    }

    pub fn mul_i128(self, right: i128) -> Self {
        Self(self.0 * right)
    }

    pub fn mul_usize(self, right: usize) -> Self {
        Self(self.0 * (right as i128))
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

impl<const DECIMALS: u32> std::ops::Add<FixedDecimal<DECIMALS>> for FixedDecimal<DECIMALS> {
    type Output = Self;

    fn add(self, right: FixedDecimal<DECIMALS>) -> Self {
        self.add(right)
    }
}

impl<const DECIMALS: u32> std::ops::Add<i128> for FixedDecimal<DECIMALS> {
    type Output = Self;

    fn add(self, right: i128) -> Self {
        self.add(FixedDecimal::from_integer(right))
    }
}

impl<const DECIMALS: u32> std::ops::Add<usize> for FixedDecimal<DECIMALS> {
    type Output = Self;

    fn add(self, right: usize) -> Self {
        self.add(FixedDecimal::from_integer(right as i128))
    }
}

impl<const DECIMALS: u32> std::ops::Add<i64> for FixedDecimal<DECIMALS> {
    type Output = Self;

    fn add(self, right: i64) -> Self {
        self.add(FixedDecimal::from_integer(right as i128))
    }
}

impl<const DECIMALS: u32> std::ops::AddAssign<FixedDecimal<DECIMALS>> for FixedDecimal<DECIMALS> {
    fn add_assign(&mut self, right: FixedDecimal<DECIMALS>) {
        *self = self.add(right);
    }
}

impl<const DECIMALS: u32> std::ops::Sub<FixedDecimal<DECIMALS>> for FixedDecimal<DECIMALS> {
    type Output = Self;

    fn sub(self, right: FixedDecimal<DECIMALS>) -> Self {
        self.sub(right)
    }
}

impl<const DECIMALS: u32> std::ops::Sub<i128> for FixedDecimal<DECIMALS> {
    type Output = Self;

    fn sub(self, right: i128) -> Self {
        self.sub(FixedDecimal::from_integer(right))
    }
}

impl<const DECIMALS: u32> std::ops::Sub<i64> for FixedDecimal<DECIMALS> {
    type Output = Self;

    fn sub(self, right: i64) -> Self {
        self.sub(FixedDecimal::from_integer(right as i128))
    }
}

impl<const DECIMALS: u32> std::ops::Sub<usize> for FixedDecimal<DECIMALS> {
    type Output = Self;

    fn sub(self, right: usize) -> Self {
        self.sub(FixedDecimal::from_integer(right as i128))
    }
}

impl<const DECIMALS: u32> std::ops::Mul<i128> for FixedDecimal<DECIMALS> {
    type Output = Self;

    fn mul(self, right: i128) -> Self {
        self.mul_i128(right)
    }
}

impl<const DECIMALS: u32> std::ops::Mul<FixedDecimal<DECIMALS>> for i128 {
    type Output = FixedDecimal<DECIMALS>;

    fn mul(self, right: FixedDecimal<DECIMALS>) -> FixedDecimal<DECIMALS> {
        right.mul_i128(self)
    }
}

impl<const DECIMALS: u32> std::ops::Mul<usize> for FixedDecimal<DECIMALS> {
    type Output = Self;

    fn mul(self, right: usize) -> Self {
        self.mul_usize(right)
    }
}

impl<const DECIMALS: u32> std::ops::Mul<FixedDecimal<DECIMALS>> for usize {
    type Output = FixedDecimal<DECIMALS>;

    fn mul(self, right: FixedDecimal<DECIMALS>) -> FixedDecimal<DECIMALS> {
        right.mul_usize(self)
    }
}

impl<const DECIMALS: u32> std::ops::Mul<FixedDecimal<DECIMALS>> for FixedDecimal<DECIMALS> {
    type Output = Self;

    fn mul(self, right: FixedDecimal<DECIMALS>) -> Self {
        self.mul(right)
    }
}

impl<const DECIMALS: u32> std::ops::Div<FixedDecimal<DECIMALS>> for FixedDecimal<DECIMALS> {
    type Output = Self;

    fn div(self, right: FixedDecimal<DECIMALS>) -> Self {
        self.div(right)
    }
}

impl<const DECIMALS: u32> std::ops::Div<i128> for FixedDecimal<DECIMALS> {
    type Output = Self;

    fn div(self, right: i128) -> Self {
        self.div(FixedDecimal::from_integer(right))
    }
}

impl<const DECIMALS: u32> std::ops::Div<i64> for FixedDecimal<DECIMALS> {
    type Output = Self;

    fn div(self, right: i64) -> Self {
        self.div(FixedDecimal::from_integer(right as i128))
    }
}

impl<const DECIMALS: u32> std::ops::Shl<u32> for FixedDecimal<DECIMALS> {
    type Output = Self;

    fn shl(self, right: u32) -> Self {
        Self(self.0 << right)
    }
}

impl<const DECIMALS: u32> std::ops::ShlAssign<u32> for FixedDecimal<DECIMALS> {
    fn shl_assign(&mut self, right: u32) {
        self.0 <<= right;
    }
}

impl<const DECIMALS: u32> std::ops::Shr<u32> for FixedDecimal<DECIMALS> {
    type Output = Self;

    fn shr(self, right: u32) -> Self {
        Self(self.0 >> right)
    }
}

impl<const DECIMALS: u32> std::ops::ShrAssign<u32> for FixedDecimal<DECIMALS> {
    fn shr_assign(&mut self, right: u32) {
        self.0 >>= right;
    }
}

impl<const DECIMALS_1: u32, const DECIMALS_2: u32> PartialOrd<FixedDecimal<DECIMALS_2>>
    for FixedDecimal<DECIMALS_1>
{
    fn partial_cmp(&self, other: &FixedDecimal<DECIMALS_2>) -> Option<Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl<const DECIMALS_1: u32, const DECIMALS_2: u32> PartialEq<FixedDecimal<DECIMALS_2>>
    for FixedDecimal<DECIMALS_1>
{
    fn eq(&self, other: &FixedDecimal<DECIMALS_2>) -> bool {
        self.0 == other.0
    }
}

impl<const DECIMALS: u32> PartialOrd<i128> for FixedDecimal<DECIMALS> {
    fn partial_cmp(&self, other: &i128) -> Option<Ordering> {
        Some(self.0.cmp(&(*other * Self::scale())))
    }
}

impl<const DECIMALS: u32> PartialEq<i128> for FixedDecimal<DECIMALS> {
    fn eq(&self, other: &i128) -> bool {
        self.0 == *other * Self::scale()
    }
}
