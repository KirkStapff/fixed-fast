use crate::{
    FixedDecimal, exp::range_reduce_taylor_exp, function::Function,
    interpolation::linear_interpolation, lookup_table::LookupTable,
};

pub type CDFV1 = CDFLinearInterpLookupTable<10>;

pub struct CDFBowlingRational<const DECIMALS: u32> {}

impl<const DECIMALS: u32> CDFBowlingRational<DECIMALS> {
    pub fn new() -> Self {
        Self {}
    }
}

impl<const DECIMALS: u32> Function<DECIMALS> for CDFBowlingRational<DECIMALS> {
    fn evaluate(&self, x: FixedDecimal<DECIMALS>) -> FixedDecimal<DECIMALS> {
        bowling_cdf(x)
    }
}

pub fn bowling_cdf<const DECIMALS: u32>(x: FixedDecimal<DECIMALS>) -> FixedDecimal<DECIMALS> {
    let expo_term = FixedDecimal::<DECIMALS>::from_str("-1.5976").unwrap() * x
        - FixedDecimal::<DECIMALS>::from_str("0.07056").unwrap() * x.cubed();
    let denominator_exponent = range_reduce_taylor_exp::<DECIMALS, 10>(expo_term);
    FixedDecimal::<DECIMALS>::one() / (FixedDecimal::<DECIMALS>::one() + denominator_exponent)
}

pub struct CDFLinearInterpLookupTable<const DECIMALS: u32> {
    lookup: LookupTable<DECIMALS>,
}

impl<const DECIMALS: u32> CDFLinearInterpLookupTable<DECIMALS> {
    pub fn new(
        start: FixedDecimal<DECIMALS>,
        end: FixedDecimal<DECIMALS>,
        step_size: FixedDecimal<DECIMALS>,
    ) -> Self {
        Self {
            lookup: LookupTable::new(start, end, step_size, bowling_cdf::<DECIMALS>),
        }
    }
}

impl<const DECIMALS: u32> Function<DECIMALS> for CDFLinearInterpLookupTable<DECIMALS> {
    fn evaluate(&self, x: FixedDecimal<DECIMALS>) -> FixedDecimal<DECIMALS> {
        if x < self.lookup.start() {
            return FixedDecimal::<DECIMALS>::zero();
        }
        if x >= self.lookup.end() {
            return FixedDecimal::<DECIMALS>::one();
        }
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

mod tests {
    use super::*;

    #[test]
    fn test_cdf() {
        let cdf = CDFBowlingRational::new();
        let x = FixedDecimal::<10>::from_str("0").unwrap();
        assert_eq!(
            cdf.evaluate(x),
            FixedDecimal::<10>::from_str("0.5").unwrap()
        );
    }

    #[test]
    fn test_cdf_linear_interp_lookup_table() {
        let table = CDFLinearInterpLookupTable::<10>::new(
            FixedDecimal::<10>::from_str("-4").unwrap(),
            FixedDecimal::<10>::from_str("4").unwrap(),
            FixedDecimal::<10>::from_str("0.00001").unwrap(),
        );
        assert_eq!(
            table.evaluate(FixedDecimal::<10>::from_str("-1.12313512").unwrap()),
            FixedDecimal::<10>::from_str("0.1307564188").unwrap()
        );
    }
}
