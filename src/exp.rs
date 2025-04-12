use crate::{
    FixedDecimal, function::Function, interpolation::linear_interpolation,
    lookup_table::LookupTable,
};

pub type ExpV1<const DECIMALS: u32> = ExpRangeReduceTaylor<DECIMALS, 10>;
pub struct ExpRangeReduceTaylor<const DECIMALS: u32, const TAYLOR_ORDER: u32> {}

impl<const DECIMALS: u32, const TAYLOR_ORDER: u32> ExpRangeReduceTaylor<DECIMALS, TAYLOR_ORDER> {
    pub fn new() -> Self {
        Self {}
    }
}

impl<const DECIMALS: u32, const TAYLOR_ORDER: u32> Function<DECIMALS>
    for ExpRangeReduceTaylor<DECIMALS, TAYLOR_ORDER>
{
    fn evaluate(&self, x: FixedDecimal<DECIMALS>) -> FixedDecimal<DECIMALS> {
        range_reduce_taylor_exp::<DECIMALS, TAYLOR_ORDER>(x)
    }
}

pub struct ExpLinearInterpLookupTable<const DECIMALS: u32, const TAYLOR_ORDER: u32> {
    lookup: LookupTable<DECIMALS>,
}

impl<const DECIMALS: u32, const TAYLOR_ORDER: u32>
    ExpLinearInterpLookupTable<DECIMALS, TAYLOR_ORDER>
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
                range_reduce_taylor_exp::<DECIMALS, TAYLOR_ORDER>,
            ),
        }
    }
}

impl<const DECIMALS: u32, const TAYLOR_ORDER: u32> Function<DECIMALS>
    for ExpLinearInterpLookupTable<DECIMALS, TAYLOR_ORDER>
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

pub fn range_reduce_taylor_exp<const DECIMALS: u32, const TAYLOR_ORDER: u32>(
    x: FixedDecimal<DECIMALS>,
) -> FixedDecimal<DECIMALS> {
    let ln2 = FixedDecimal::<DECIMALS>::ln2();
    let k = (x / ln2).floor_i128();
    let r = x - ln2 * FixedDecimal::from_i128(k);

    let mut term = FixedDecimal::<DECIMALS>::from_i128(1);
    let mut result = term;
    for i in 1..=TAYLOR_ORDER {
        term = term * r / i;
        result += term;
    }
    let range_gain = FixedDecimal::<DECIMALS>::two_pow_k(k as i32);
    result * range_gain
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_reduce_taylor_exp() {
        let x = FixedDecimal::<10>::from_str("1.0").unwrap();
        assert_eq!(
            range_reduce_taylor_exp::<10, 10>(x),
            FixedDecimal::<10>::from_str("2.7182818278").unwrap()
        );
        let x = FixedDecimal::<10>::from_str("-1.231231").unwrap();
        assert_eq!(
            range_reduce_taylor_exp::<10, 20>(x),
            FixedDecimal::<10>::from_str("0.291932986891").unwrap()
        );
        let x = FixedDecimal::<10>::from_str("0").unwrap();
        assert_eq!(
            range_reduce_taylor_exp::<10, 20>(x),
            FixedDecimal::<10>::from_str("1").unwrap()
        );
    }

    #[test]
    fn test_exp_linear_interp_lookup_table() {
        let table = ExpLinearInterpLookupTable::<10, 10>::new(
            FixedDecimal::<10>::from_str("-10").unwrap(),
            FixedDecimal::<10>::from_str("10").unwrap(),
            FixedDecimal::<10>::from_str("0.00001").unwrap(),
        );
        assert_eq!(
            table.evaluate(FixedDecimal::<10>::from_str("-1.12313512").unwrap()),
            FixedDecimal::<10>::from_str("0.3252584700").unwrap()
        );
        assert_eq!(
            table.evaluate(FixedDecimal::<10>::from_str("2").unwrap()),
            FixedDecimal::<10>::from_str("7.3890560972").unwrap()
        );
    }
}
