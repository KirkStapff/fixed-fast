use crate::fixed_decimal::{Fixed, FixedDecimal};

pub trait Function<T: Fixed> {
    fn evaluate(&self, x: FixedDecimal<T>) -> FixedDecimal<T>;
}
