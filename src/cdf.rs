use std::marker::PhantomData;

use crate::{
    FixedDecimal, exp::range_reduce_taylor_exp, fixed_decimal::Fixed, function::Function,
    interpolation::linear_interpolation, lookup_table::LookupTable,
};

pub type CDFV1<T> = CDFLinearInterpLookupTable<T>;

pub struct CDFBowlingRational<T: Fixed> {
    _precision: PhantomData<T>,
}

impl<T: Fixed> CDFBowlingRational<T> {
    pub fn new() -> Self {
        Self {
            _precision: PhantomData,
        }
    }
}

impl<T: Fixed> Function<T> for CDFBowlingRational<T> {
    fn evaluate(&self, x: FixedDecimal<T>) -> FixedDecimal<T> {
        bowling_cdf(x)
    }
}

pub fn bowling_cdf<T: Fixed>(x: FixedDecimal<T>) -> FixedDecimal<T> {
    let expo_term = FixedDecimal::<T>::from_str("-1.5976").unwrap() * x
        - FixedDecimal::<T>::from_str("0.07056").unwrap() * x.cubed();
    let denominator_exponent = range_reduce_taylor_exp::<T, 10>(expo_term);
    FixedDecimal::<T>::one() / (FixedDecimal::<T>::one() + denominator_exponent)
}

pub struct CDFLinearInterpLookupTable<T: Fixed> {
    lookup: LookupTable<T>,
}

impl<T: Fixed> CDFLinearInterpLookupTable<T> {
    pub fn new(start: FixedDecimal<T>, end: FixedDecimal<T>, step_size: FixedDecimal<T>) -> Self {
        Self {
            lookup: LookupTable::new(start, end, step_size, bowling_cdf),
        }
    }
}

impl<T: Fixed> Function<T> for CDFLinearInterpLookupTable<T> {
    fn evaluate(&self, x: FixedDecimal<T>) -> FixedDecimal<T> {
        if x < self.lookup.start() {
            return FixedDecimal::<T>::zero();
        }
        if x >= self.lookup.end() {
            return FixedDecimal::<T>::one();
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

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    struct F10;

    impl Fixed for F10 {
        const PRECISION: u32 = 10;
    }

    #[test]
    fn test_cdf() {
        let cdf = CDFBowlingRational::new();
        let x = FixedDecimal::<F10>::from_str("0").unwrap();
        assert_eq!(
            cdf.evaluate(x),
            FixedDecimal::<F10>::from_str("0.5").unwrap()
        );
    }

    #[test]
    fn test_cdf_linear_interp_lookup_table() {
        let table = CDFLinearInterpLookupTable::<F10>::new(
            FixedDecimal::<F10>::from_str("-4").unwrap(),
            FixedDecimal::<F10>::from_str("4").unwrap(),
            FixedDecimal::<F10>::from_str("0.00001").unwrap(),
        );
        assert_eq!(
            table.evaluate(FixedDecimal::<F10>::from_str("-1.12313512").unwrap()),
            FixedDecimal::<F10>::from_str("0.1307564188").unwrap()
        );
    }
}
