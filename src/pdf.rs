use std::marker::PhantomData;

use crate::{
    FixedDecimal, exp::range_reduce_taylor_exp, fixed_decimal::FixedPrecision, function::Function,
    interpolation::linear_interpolation, lookup_table::LookupTable, sqrt::sqrt_newton_raphson,
};

pub type PDFV1<T> = PDFLinearInterpLookupTable<T>;

pub struct PDF<T: FixedPrecision> {
    _precision: PhantomData<T>,
}

impl<T: FixedPrecision> PDF<T> {
    pub fn new() -> Self {
        Self {
            _precision: PhantomData,
        }
    }
}

impl<T: FixedPrecision> Function<T> for PDF<T> {
    fn evaluate(&self, x: FixedDecimal<T>) -> FixedDecimal<T> {
        pdf(x)
    }
}

pub fn pdf<T: FixedPrecision>(x: FixedDecimal<T>) -> FixedDecimal<T> {
    let coef = 1 / sqrt_newton_raphson::<T, 20>(2 * FixedDecimal::<T>::pi());
    let exponent = -x.squared() / 2;
    let result = coef * range_reduce_taylor_exp::<T, 20>(exponent);
    result
}

pub struct PDFLinearInterpLookupTable<T: FixedPrecision> {
    lookup: LookupTable<T>,
}

impl<T: FixedPrecision> PDFLinearInterpLookupTable<T> {
    pub fn new(start: FixedDecimal<T>, end: FixedDecimal<T>, step_size: FixedDecimal<T>) -> Self {
        Self {
            lookup: LookupTable::new(start, end, step_size, pdf::<T>),
        }
    }
}

impl<T: FixedPrecision> Function<T> for PDFLinearInterpLookupTable<T> {
    fn evaluate(&self, x: FixedDecimal<T>) -> FixedDecimal<T> {
        if x < self.lookup.start() || x > self.lookup.end() {
            return FixedDecimal::<T>::zero();
        }
        let index = self.lookup.get_index(x).expect("Index not found");
        let lower_value = self.lookup.step_size() * index + self.lookup.start();
        println!(
            "X: {} PDF Index: {} Lower Value: {} PDF: {} PDF+1: {}",
            x,
            index,
            lower_value,
            self.lookup.table[index],
            self.lookup.table[index + 1]
        );
        let result = linear_interpolation(
            x,
            lower_value,
            lower_value + self.lookup.step_size(),
            self.lookup.table[index],
            self.lookup.table[index + 1],
        );
        result
    }
}

mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    struct F10;

    impl FixedPrecision for F10 {
        const PRECISION: u32 = 10;
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    struct F14;

    impl FixedPrecision for F14 {
        const PRECISION: u32 = 14;
    }

    #[test]
    fn test_pdf() {
        let pdf = PDF::new();
        let x = FixedDecimal::<F10>::from_str("0").unwrap();
        assert_eq!(
            pdf.evaluate(x),
            FixedDecimal::<F10>::from_str("0.3989422804").unwrap()
        );
    }

    #[test]
    fn test_pdf_linear_interp_lookup_table() {
        let pdf = PDFLinearInterpLookupTable::<F14>::new(
            FixedDecimal::<F14>::from_str("-4").unwrap(),
            FixedDecimal::<F14>::from_str("4").unwrap(),
            FixedDecimal::<F14>::from_str("0.00001").unwrap(),
        );
        assert_eq!(
            pdf.evaluate(FixedDecimal::<F14>::from_str("-1.12313512").unwrap()),
            FixedDecimal::<F14>::from_str("0.21232125827745").unwrap()
        );
        assert_eq!(
            pdf.evaluate(FixedDecimal::<F14>::from_str("0").unwrap()),
            FixedDecimal::<F14>::from_str("0.39894228040143").unwrap()
        );
        assert_eq!(
            pdf.evaluate(FixedDecimal::<F14>::from_str("2.3463434").unwrap()),
            FixedDecimal::<F14>::from_str("0.02543568401209").unwrap()
        );
    }
}
