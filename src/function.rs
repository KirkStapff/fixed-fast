use crate::{
    error::Result,
    fixed_decimal::{FixedDecimal, FixedPrecision},
};

pub trait Function<T: FixedPrecision> {
    fn evaluate(&self, x: FixedDecimal<T>) -> FixedDecimal<T>;
}

pub trait TryFunction<T: FixedPrecision> {
    fn try_evaluate(&self, x: FixedDecimal<T>) -> Result<FixedDecimal<T>>;
}
