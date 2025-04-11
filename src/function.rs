use crate::fixed_decimal::FixedDecimal;

pub trait Function<const DECIMALS: u32> {
    fn evaluate(&self, x: FixedDecimal<DECIMALS>) -> FixedDecimal<DECIMALS>;
}
