use std::marker::PhantomData;

use crate::{
    FixedDecimal, fixed_decimal::FixedPrecision, function::Function,
    interpolation::linear_interpolation, lookup_table::LookupTable,
};

pub type ExpV1<T> = ExpLinearInterpLookupTable<T, 10>;
pub struct ExpRangeReduceTaylor<T: FixedPrecision, const TAYLOR_ORDER: u32> {
    _precision: PhantomData<T>,
}

impl<T: FixedPrecision, const TAYLOR_ORDER: u32> ExpRangeReduceTaylor<T, TAYLOR_ORDER> {
    pub fn new() -> Self {
        Self {
            _precision: PhantomData,
        }
    }
}

impl<T: FixedPrecision, const TAYLOR_ORDER: u32> Function<T>
    for ExpRangeReduceTaylor<T, TAYLOR_ORDER>
{
    fn evaluate(&self, x: FixedDecimal<T>) -> FixedDecimal<T> {
        range_reduce_taylor_exp::<T, TAYLOR_ORDER>(x)
    }
}

pub struct ExpLinearInterpLookupTable<T: FixedPrecision, const TAYLOR_ORDER: u32> {
    lookup: LookupTable<T>,
}

impl<T: FixedPrecision, const TAYLOR_ORDER: u32> ExpLinearInterpLookupTable<T, TAYLOR_ORDER> {
    pub fn new(start: FixedDecimal<T>, end: FixedDecimal<T>, step_size: FixedDecimal<T>) -> Self {
        Self {
            lookup: LookupTable::new(
                start,
                end,
                step_size,
                range_reduce_taylor_exp::<T, TAYLOR_ORDER>,
            ),
        }
    }
}

impl<T: FixedPrecision, const TAYLOR_ORDER: u32> Function<T>
    for ExpLinearInterpLookupTable<T, TAYLOR_ORDER>
{
    fn evaluate(&self, x: FixedDecimal<T>) -> FixedDecimal<T> {
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

pub fn range_reduce_taylor_exp<T: FixedPrecision, const TAYLOR_ORDER: u32>(
    x: FixedDecimal<T>,
) -> FixedDecimal<T> {
    let ln2 = FixedDecimal::<T>::ln2();
    println!("x: {} ln2: {}", x.to_f64(), ln2.to_f64());
    let k = (x / ln2).floor_i128();
    let r = x - ln2 * FixedDecimal::from_i128(k);

    let mut term = FixedDecimal::<T>::from_i128(1);
    let mut result = term;
    for i in 1..=TAYLOR_ORDER {
        term = term * r / i;
        result += term;
    }
    println!("k: {}", k);
    let range_gain = FixedDecimal::<T>::two_pow_k(k as i32);
    result * range_gain
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    struct F10;

    impl FixedPrecision for F10 {
        const PRECISION: u32 = 10;
    }

    #[test]
    fn test_range_reduce_taylor_exp() {
        let x = FixedDecimal::<F10>::from_str("1.0").unwrap();
        assert_eq!(
            range_reduce_taylor_exp::<F10, 10>(x),
            FixedDecimal::<F10>::from_str("2.7182818278").unwrap()
        );
        let x = FixedDecimal::<F10>::from_str("-1.231231").unwrap();
        assert_eq!(
            range_reduce_taylor_exp::<F10, 20>(x),
            FixedDecimal::<F10>::from_str("0.291932986891").unwrap()
        );
        let x = FixedDecimal::<F10>::from_str("0").unwrap();
        assert_eq!(
            range_reduce_taylor_exp::<F10, 20>(x),
            FixedDecimal::<F10>::from_str("1").unwrap()
        );
    }

    #[test]
    fn test_exp_linear_interp_lookup_table() {
        let table = ExpLinearInterpLookupTable::<F10, 10>::new(
            FixedDecimal::<F10>::from_str("-10").unwrap(),
            FixedDecimal::<F10>::from_str("10").unwrap(),
            FixedDecimal::<F10>::from_str("0.00001").unwrap(),
        );
        assert_eq!(
            table.evaluate(FixedDecimal::<F10>::from_str("-1.12313512").unwrap()),
            FixedDecimal::<F10>::from_str("0.3252584700").unwrap()
        );
        assert_eq!(
            table.evaluate(FixedDecimal::<F10>::from_str("2").unwrap()),
            FixedDecimal::<F10>::from_str("7.3890560972").unwrap()
        );
    }
}
