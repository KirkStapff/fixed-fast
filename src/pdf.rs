use crate::{
    FixedDecimal, exp::range_reduce_taylor_exp, function::Function,
    interpolation::linear_interpolation, lookup_table::LookupTable, sqrt::sqrt_newton_raphson,
};

pub type PDFV1 = PDFLinearInterpLookupTable<10>;

pub struct PDF<const DECIMALS: u32> {}

impl<const DECIMALS: u32> PDF<DECIMALS> {
    pub fn new() -> Self {
        Self {}
    }
}

impl<const DECIMALS: u32> Function<DECIMALS> for PDF<DECIMALS> {
    fn evaluate(&self, x: FixedDecimal<DECIMALS>) -> FixedDecimal<DECIMALS> {
        pdf(x)
    }
}

pub fn pdf<const DECIMALS: u32>(x: FixedDecimal<DECIMALS>) -> FixedDecimal<DECIMALS> {
    let coef = 1 / sqrt_newton_raphson::<DECIMALS, 20>(2 * FixedDecimal::<DECIMALS>::pi());
    let exponent = -x.squared() / 2;
    let result = coef * range_reduce_taylor_exp::<DECIMALS, 20>(exponent);
    result
}

pub struct PDFLinearInterpLookupTable<const DECIMALS: u32> {
    lookup: LookupTable<DECIMALS>,
}

impl<const DECIMALS: u32> PDFLinearInterpLookupTable<DECIMALS> {
    pub fn new(
        start: FixedDecimal<DECIMALS>,
        end: FixedDecimal<DECIMALS>,
        step_size: FixedDecimal<DECIMALS>,
    ) -> Self {
        Self {
            lookup: LookupTable::new(start, end, step_size, pdf::<DECIMALS>),
        }
    }
}

impl<const DECIMALS: u32> Function<DECIMALS> for PDFLinearInterpLookupTable<DECIMALS> {
    fn evaluate(&self, x: FixedDecimal<DECIMALS>) -> FixedDecimal<DECIMALS> {
        if x < self.lookup.start() || x > self.lookup.end() {
            return FixedDecimal::<DECIMALS>::zero();
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

    #[test]
    fn test_pdf() {
        let pdf = PDF::new();
        let x = FixedDecimal::<10>::from_str("0").unwrap();
        assert_eq!(
            pdf.evaluate(x),
            FixedDecimal::<10>::from_str("0.3989422804").unwrap()
        );
    }

    #[test]
    fn test_pdf_linear_interp_lookup_table() {
        let pdf = PDFLinearInterpLookupTable::<14>::new(
            FixedDecimal::<14>::from_str("-4").unwrap(),
            FixedDecimal::<14>::from_str("4").unwrap(),
            FixedDecimal::<14>::from_str("0.00001").unwrap(),
        );
        assert_eq!(
            pdf.evaluate(FixedDecimal::<14>::from_str("-1.12313512").unwrap()),
            FixedDecimal::<14>::from_str("0.21232125827745").unwrap()
        );
        assert_eq!(
            pdf.evaluate(FixedDecimal::<14>::from_str("0").unwrap()),
            FixedDecimal::<14>::from_str("0.39894228040143").unwrap()
        );
        assert_eq!(
            pdf.evaluate(FixedDecimal::<14>::from_str("2.3463434").unwrap()),
            FixedDecimal::<14>::from_str("0.02543568401209").unwrap()
        );
    }
}
