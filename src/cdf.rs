use std::marker::PhantomData;

use crate::{
    FixedDecimal,
    error::Result,
    exp::range_reduce_taylor_exp,
    fixed_decimal::FixedPrecision,
    function::{Function, TryFunction},
    interpolation::linear_interpolation,
    lookup_table::LookupTable,
};

pub type CDFV1<T> = CDFLinearInterpLookupTable<T>;

pub struct CDFCustomAprox<T: FixedPrecision> {
    _precision: PhantomData<T>,
    coefficients: [FixedDecimal<T>; 13],
}

impl<T: FixedPrecision> CDFCustomAprox<T> {
    pub fn new() -> Self {
        Self {
            _precision: PhantomData,
            coefficients: [
                FixedDecimal::from_str("-0.00000000436953479").unwrap(), //0
                FixedDecimal::from_str("1.59576962000000000").unwrap(),  //1
                FixedDecimal::from_str("-0.00000955404601").unwrap(),    //2
                FixedDecimal::from_str("0.07274169990000000").unwrap(),  //3
                FixedDecimal::from_str("-0.00026565023900000").unwrap(), //4
                FixedDecimal::from_str("0.00050857094000000").unwrap(),  //5
                FixedDecimal::from_str("-0.00079653385500000").unwrap(), //6
                FixedDecimal::from_str("0.00059778488700000").unwrap(),  //7
                FixedDecimal::from_str("-0.00040077230600000").unwrap(), //8
                FixedDecimal::from_str("0.00014965658000000").unwrap(),  //9
                FixedDecimal::from_str("-0.00002988796070000").unwrap(), //10
                FixedDecimal::from_str("0.00000306494352000").unwrap(),  //11
                FixedDecimal::from_str("-0.00000012783404900").unwrap(), //12
            ],
        }
    }
}

impl<T: FixedPrecision> Function<T> for CDFCustomAprox<T> {
    fn evaluate(&self, x: FixedDecimal<T>) -> FixedDecimal<T> {
        if x < FixedDecimal::<T>::from_str("-6").unwrap() {
            return FixedDecimal::<T>::zero();
        }
        if x > FixedDecimal::<T>::from_str("6").unwrap() {
            return FixedDecimal::<T>::one();
        }
        topher_cdf(x, &self.coefficients)
    }
}

impl<T: FixedPrecision> TryFunction<T> for CDFCustomAprox<T> {
    fn try_evaluate(&self, x: FixedDecimal<T>) -> Result<FixedDecimal<T>> {
        Ok(self.evaluate(x)) // evaluation itself is safe within given domain
    }
}

pub fn topher_cdf<T: FixedPrecision>(
    x: FixedDecimal<T>,
    coefficients: &[FixedDecimal<T>; 13],
) -> FixedDecimal<T> {
    if x < 0 {
        return FixedDecimal::<T>::one() - topher_cdf(-x, coefficients);
    }
    let f = x.polynomial(coefficients);
    let denominator_exponent = range_reduce_taylor_exp::<T, 30>(-f);
    let result = FixedDecimal::<T>::one() / (FixedDecimal::<T>::one() + denominator_exponent);
    result
}
pub struct CDFLinearInterpLookupTable<T: FixedPrecision> {
    lookup: LookupTable<T>,
}

impl<T: FixedPrecision> CDFLinearInterpLookupTable<T> {
    pub fn new(end: FixedDecimal<T>, step_size: FixedDecimal<T>) -> Self {
        let custom_aprox = CDFCustomAprox::new();
        Self {
            lookup: LookupTable::new(FixedDecimal::zero(), end, step_size, |x| {
                custom_aprox.evaluate(x)
            }),
        }
    }
}

impl<T: FixedPrecision> Function<T> for CDFLinearInterpLookupTable<T> {
    fn evaluate(&self, x: FixedDecimal<T>) -> FixedDecimal<T> {
        if x < 0 {
            return FixedDecimal::<T>::one() - self.evaluate(-x);
        }
        if x >= self.lookup.end() {
            return FixedDecimal::<T>::one();
        }
        let index = self.lookup.get_index(x).expect("Index not found");
        if index + 1 >= self.lookup.table.len() {
            return self.lookup.table[index];
        }
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

impl<T: FixedPrecision> TryFunction<T> for CDFLinearInterpLookupTable<T> {
    fn try_evaluate(&self, x: FixedDecimal<T>) -> Result<FixedDecimal<T>> {
        if x < 0 {
            return self.try_evaluate(-x).map(|v| FixedDecimal::<T>::one() - v);
        }
        if x >= self.lookup.end() {
            return Ok(FixedDecimal::<T>::one());
        }
        let index = self.lookup.get_index(x)?;
        if index + 1 >= self.lookup.table.len() {
            return Ok(self.lookup.table[index]);
        }
        let lower_value = self.lookup.step_size() * index + self.lookup.start();
        Ok(linear_interpolation(
            x,
            lower_value,
            lower_value + self.lookup.step_size(),
            self.lookup.table[index],
            self.lookup.table[index + 1],
        ))
    }
}

mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    struct F9;

    impl FixedPrecision for F9 {
        const PRECISION: u32 = 9;
    }

    #[test]
    fn test_cdf() {
        let cdf = CDFCustomAprox::new();
        let x = FixedDecimal::<F9>::from_str("1.16685").unwrap();
        assert_eq!(
            cdf.evaluate(x),
            FixedDecimal::<F9>::from_str("0.878364523159478638").unwrap()
        );
        let x = FixedDecimal::<F9>::from_str("-1.12313512").unwrap();
        assert_eq!(
            cdf.evaluate(x),
            FixedDecimal::<F9>::from_str("0.130690057273233524").unwrap()
        );
    }

    #[test]
    fn test_cdf_linear_interp_lookup_table() {
        let table = CDFLinearInterpLookupTable::<F9>::new(
            FixedDecimal::<F9>::from_str("6").unwrap(),
            FixedDecimal::<F9>::from_str("0.00001").unwrap(),
        );
        assert_eq!(
            table.evaluate(FixedDecimal::<F9>::from_str("-1.12313512").unwrap()),
            FixedDecimal::<F9>::from_str("0.130690058").unwrap()
        );
    }
}
