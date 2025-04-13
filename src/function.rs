use crate::fixed_decimal::{FixedDecimal, FixedPrecision};

pub trait Function<T: FixedPrecision> {
    fn evaluate(&self, x: FixedDecimal<T>) -> FixedDecimal<T>;
}
