use std::marker::PhantomData;

use crate::{
    error::{FixedFastError, Result},
    fixed_decimal::{FixedDecimal, FixedPrecision},
    function::{Function, TryFunction},
    interpolation::linear_interpolation,
    lookup_table::LookupTable,
};

pub type SqrtV1<T> = SqrtLinearInterpLookupTable<T, 12>;

pub struct SqrtNewtonRaphson<T: FixedPrecision, const APPROX_DEPTH: u32> {
    _precision: PhantomData<T>,
}

impl<T: FixedPrecision, const APPROX_DEPTH: u32> SqrtNewtonRaphson<T, APPROX_DEPTH> {
    pub fn new() -> Self {
        Self {
            _precision: PhantomData,
        }
    }
}

impl<T: FixedPrecision, const APPROX_DEPTH: u32> Function<T>
    for SqrtNewtonRaphson<T, APPROX_DEPTH>
{
    fn evaluate(&self, x: FixedDecimal<T>) -> FixedDecimal<T> {
        sqrt_newton_raphson::<T, APPROX_DEPTH>(x)
    }
}

pub struct SqrtLinearInterpLookupTable<T: FixedPrecision, const APPROX_DEPTH: u32> {
    lookup: LookupTable<T>,
}

impl<T: FixedPrecision, const APPROX_DEPTH: u32> SqrtLinearInterpLookupTable<T, APPROX_DEPTH> {
    pub fn new(start: FixedDecimal<T>, end: FixedDecimal<T>, step_size: FixedDecimal<T>) -> Self {
        Self {
            lookup: LookupTable::new(
                start,
                end,
                step_size,
                sqrt_newton_raphson::<T, APPROX_DEPTH>,
            ),
        }
    }
}

impl<T: FixedPrecision, const APPROX_DEPTH: u32> Function<T>
    for SqrtLinearInterpLookupTable<T, APPROX_DEPTH>
{
    fn evaluate(&self, x: FixedDecimal<T>) -> FixedDecimal<T> {
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

pub fn sqrt_newton_raphson<T: FixedPrecision, const APPROX_DEPTH: u32>(
    x: FixedDecimal<T>,
) -> FixedDecimal<T> {
    if x == 0 {
        return FixedDecimal::<T>::from_i128(0);
    }
    let mut y = x / 2_i64;
    for _ in 0..APPROX_DEPTH {
        y = (y + x.div(y)) / 2_i64;
    }
    y
}

pub fn sqrt_newton_raphson_try<T: FixedPrecision, const APPROX_DEPTH: u32>(
    x: FixedDecimal<T>,
) -> Result<FixedDecimal<T>> {
    if x < FixedDecimal::<T>::zero() {
        return Err(FixedFastError::DomainError(
            "sqrt is undefined for negative numbers",
        ));
    }
    if x == FixedDecimal::<T>::zero() {
        return Ok(FixedDecimal::<T>::zero());
    }
    let mut y = x / 2_i64;
    for _ in 0..APPROX_DEPTH {
        y = (y + x.div(y)) / 2_i64;
    }
    Ok(y)
}

// TryFunction implementation for direct sqrt algorithm
impl<T: FixedPrecision, const APPROX_DEPTH: u32> TryFunction<T>
    for SqrtNewtonRaphson<T, APPROX_DEPTH>
{
    fn try_evaluate(&self, x: FixedDecimal<T>) -> Result<FixedDecimal<T>> {
        sqrt_newton_raphson_try::<T, APPROX_DEPTH>(x)
    }
}

// TryFunction implementation for lookup table based sqrt
impl<T: FixedPrecision, const APPROX_DEPTH: u32> TryFunction<T>
    for SqrtLinearInterpLookupTable<T, APPROX_DEPTH>
{
    fn try_evaluate(&self, x: FixedDecimal<T>) -> Result<FixedDecimal<T>> {
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
    struct F18;

    impl FixedPrecision for F18 {
        const PRECISION: u32 = 18;
    }

    #[test]
    fn test_sqrt_newton_raphson() {
        let input = FixedDecimal::<F18>::from_str("1.3453453453453453").unwrap();
        assert_eq!(
            sqrt_newton_raphson::<F18, 12>(input),
            FixedDecimal::<F18>::from_str("1.159890229868906732").unwrap()
        );
    }

    #[test]
    fn test_sqrt_linear_interp_lookup_table() {
        let sqrt = SqrtLinearInterpLookupTable::<F18, 12>::new(
            FixedDecimal::<F18>::from_str("0").unwrap(),
            FixedDecimal::<F18>::from_str("40").unwrap(),
            FixedDecimal::<F18>::from_str("0.00001").unwrap(),
        );
        let input = FixedDecimal::<F18>::from_str("27.234124123124").unwrap();
        assert_eq!(
            sqrt.evaluate(input),
            FixedDecimal::<F18>::from_str("5.218632399692833084").unwrap()
        );
    }
}
