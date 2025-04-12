use crate::fixed_decimal::{FixedPrecision, FixedDecimal};

pub trait Function<T: FixedPrecision> {
    fn evaluate(&self, x: FixedDecimal<T>) -> FixedDecimal<T>;
}
