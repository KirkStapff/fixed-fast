use crate::{
    fixed_decimal::FixedDecimal, function::Function, interpolation::linear_interpolation,
    lookup_table::LookupTable,
};

pub type SqrtV1<const DECIMALS: u32> = SqrtLinearInterpLookupTable<DECIMALS, 12>;

pub struct SqrtNewtonRaphson<const DECIMALS: u32, const APPROX_DEPTH: u32> {}

impl<const DECIMALS: u32, const APPROX_DEPTH: u32> SqrtNewtonRaphson<DECIMALS, APPROX_DEPTH> {
    pub fn new() -> Self {
        Self {}
    }
}

impl<const DECIMALS: u32, const APPROX_DEPTH: u32> Function<DECIMALS>
    for SqrtNewtonRaphson<DECIMALS, APPROX_DEPTH>
{
    fn evaluate(&self, x: FixedDecimal<DECIMALS>) -> FixedDecimal<DECIMALS> {
        sqrt_newton_raphson::<DECIMALS, APPROX_DEPTH>(x)
    }
}

pub struct SqrtLinearInterpLookupTable<const DECIMALS: u32, const APPROX_DEPTH: u32> {
    lookup: LookupTable<DECIMALS>,
}

impl<const DECIMALS: u32, const APPROX_DEPTH: u32>
    SqrtLinearInterpLookupTable<DECIMALS, APPROX_DEPTH>
{
    pub fn new(
        start: FixedDecimal<DECIMALS>,
        end: FixedDecimal<DECIMALS>,
        step_size: FixedDecimal<DECIMALS>,
    ) -> Self {
        Self {
            lookup: LookupTable::new(
                start,
                end,
                step_size,
                sqrt_newton_raphson::<DECIMALS, APPROX_DEPTH>,
            ),
        }
    }
}

impl<const DECIMALS: u32, const APPROX_DEPTH: u32> Function<DECIMALS>
    for SqrtLinearInterpLookupTable<DECIMALS, APPROX_DEPTH>
{
    fn evaluate(&self, x: FixedDecimal<DECIMALS>) -> FixedDecimal<DECIMALS> {
        let index = self.lookup.get_index(x).expect("Index not found");
        let lower_value = self.lookup.step_size() * index + self.lookup.start();
        linear_interpolation(
            x,
            lower_value,
            lower_value + self.lookup.step_size(),
            self.lookup.table[index],
            self.lookup.table[index + 1],
        )
    }
}

pub fn sqrt_newton_raphson<const DECIMALS: u32, const APPROX_DEPTH: u32>(
    x: FixedDecimal<DECIMALS>,
) -> FixedDecimal<DECIMALS> {
    if x == 0 {
        return FixedDecimal::<DECIMALS>::from_i128(0);
    }
    let mut y = x / 2_i64;
    for _ in 0..APPROX_DEPTH {
        y = (y + x.div(y)) / 2_i64;
    }
    y
}

mod tests {
    use super::*;

    #[test]
    fn test_sqrt_newton_raphson() {
        let input = FixedDecimal::<18>::from_str("1.3453453453453453").unwrap();
        assert_eq!(
            sqrt_newton_raphson::<18, 12>(input),
            FixedDecimal::<18>::from_str("1.159890229868906732").unwrap()
        );
    }

    #[test]
    fn test_sqrt_linear_interp_lookup_table() {
        let sqrt = SqrtLinearInterpLookupTable::<18, 12>::new(
            FixedDecimal::<18>::from_str("0").unwrap(),
            FixedDecimal::<18>::from_str("40").unwrap(),
            FixedDecimal::<18>::from_str("0.00001").unwrap(),
        );
        let input = FixedDecimal::<18>::from_str("27.234124123124").unwrap();
        assert_eq!(
            sqrt.evaluate(input),
            FixedDecimal::<18>::from_str("5.218632399692833084").unwrap()
        );
    }
}
