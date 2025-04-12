use std::marker::PhantomData;

use crate::{
    fixed_decimal::{Fixed, FixedDecimal},
    function::Function,
    interpolation::linear_interpolation,
    lookup_table::LookupTable,
};

pub type LnV1<T> = LnLinearInterpLookupTable<T, 12>;

pub struct LnArcTanhExpansion<T: Fixed, const APPROX_DEPTH: u32> {
    _precision: PhantomData<T>,
}

impl<T: Fixed, const APPROX_DEPTH: u32> Function<T> for LnArcTanhExpansion<T, APPROX_DEPTH> {
    fn evaluate(&self, x: FixedDecimal<T>) -> FixedDecimal<T> {
        range_reduce_arctanh_ln::<T, APPROX_DEPTH>(x)
    }
}

impl<T: Fixed, const APPROX_DEPTH: u32> LnArcTanhExpansion<T, APPROX_DEPTH> {
    pub fn new() -> Self {
        Self {
            _precision: PhantomData,
        }
    }
}

pub struct LnLinearInterpLookupTable<T: Fixed, const APPROX_DEPTH: u32> {
    lookup: LookupTable<T>,
}

impl<T: Fixed, const APPROX_DEPTH: u32> LnLinearInterpLookupTable<T, APPROX_DEPTH> {
    pub fn new(start: FixedDecimal<T>, end: FixedDecimal<T>, step_size: FixedDecimal<T>) -> Self {
        Self {
            lookup: LookupTable::new(
                start,
                end,
                step_size,
                range_reduce_arctanh_ln::<T, APPROX_DEPTH>,
            ),
        }
    }
}

impl<T: Fixed, const APPROX_DEPTH: u32> Function<T> for LnLinearInterpLookupTable<T, APPROX_DEPTH> {
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

fn range_reduce_arctanh_ln<T: Fixed, const APPROX_DEPTH: u32>(
    input: FixedDecimal<T>,
) -> FixedDecimal<T> {
    let mut shift_coef = 0;
    let mut input = input;
    if input == 0 {
        panic!("ln(0) is undefined");
    }
    while input > 2 {
        input /= 2;
        shift_coef += 1;
    }
    while input < 1 {
        input *= 2;
        shift_coef -= 1;
    }
    // ln(x) = 2 arctanh(x - 1 / x + 1) logarithmic expansion via inverse hyperbolic tangent

    let arctan_term: FixedDecimal<T> = (input - 1) / (input + 1);
    println!("arctan_term: {}", arctan_term.to_f64());
    let arctan_term_squared = arctan_term * arctan_term;
    println!("arctan_term_squared: {}", arctan_term_squared.to_f64());
    let mut nth_term = arctan_term;
    let mut running_sum = nth_term;
    for n in 1..APPROX_DEPTH {
        nth_term = nth_term * arctan_term_squared / (2 * n as i64 + 1);
        println!("nth_term: {}", nth_term.to_f64());
        running_sum += nth_term;
    }
    let shift: FixedDecimal<T> = FixedDecimal::<T>::ln2() * shift_coef;
    println!("shift: {}", shift.to_f64());
    println!("running_sum: {} ", running_sum.to_f64());
    let result: FixedDecimal<T> = running_sum * 2 + shift;
    println!("result: {}", result.to_string());
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    struct F18;

    impl Fixed for F18 {
        const PRECISION: u32 = 18;
    }

    #[test]
    fn test_function() {
        assert_eq!(
            range_reduce_arctanh_ln::<F18, 10>(FixedDecimal::<F18>::from_i128(1)),
            FixedDecimal::<F18>::from_i128(0)
        );
        let input = FixedDecimal::<F18>::from_str("1.4").unwrap();
        assert_eq!(
            range_reduce_arctanh_ln::<F18, 10>(input),
            FixedDecimal::<F18>::from_str("0.336436968116129286").unwrap()
        );
        let input = FixedDecimal::<F18>::from_str("69.3").unwrap();
        assert_eq!(
            range_reduce_arctanh_ln::<F18, 10>(input),
            FixedDecimal::<F18>::from_str("4.238444879656876612").unwrap()
        );
    }

    #[test]
    fn test_lookup_table() {
        // let ln = LnLinearInterpLookupTable::<F18, 10>::new(
        //     FixedDecimal::<F18>::from_str("0.000000001").unwrap(),
        //     FixedDecimal::<F18>::from_str("10").unwrap(),
        //     FixedDecimal::<F18>::from_str("0.01").unwrap(),
        // );
        // let input = FixedDecimal::<F18>::from_str("1.3453453453453453").unwrap();
        // assert_eq!(
        //     ln.evaluate(input),
        //     FixedDecimal::<F18>::from_str("0.296631876146752907").unwrap()
        // );
    }
}
