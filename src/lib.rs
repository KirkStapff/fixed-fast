mod cdf;
mod error;
mod exp;
mod fixed_decimal;
mod function;
mod interpolation;
mod ln;
mod lookup_table;
mod pdf;
mod sqrt;

pub use cdf::{CDFLinearInterpLookupTable, CDFV1};
pub use exp::{ExpLinearInterpLookupTable, ExpRangeReduceTaylor, ExpV1};
pub use fixed_decimal::FixedDecimal;
pub use ln::{LnArcTanhExpansion, LnLinearInterpLookupTable, LnV1};
pub use pdf::{PDFLinearInterpLookupTable, PDFV1};
pub use sqrt::{SqrtLinearInterpLookupTable, SqrtNewtonRaphson, SqrtV1};

#[cfg(test)]
mod tests {
    use crate::fixed_decimal::FixedDecimal;

    const ONE_SCALED_INTEGER: i128 = 1000000000;

    #[test]
    fn mul() {
        let a = FixedDecimal::<9>::from_i128(1);
        let b = FixedDecimal::<9>::from_i128(2);
        let c = a.mul(b);
        assert_eq!(c.to_i128(), 2);
        assert_eq!(c * 2_u64, FixedDecimal::<9>::from_i128(4));
    }

    #[test]
    fn div() {
        let a = FixedDecimal::<9>::from_i128(1);
        let b = FixedDecimal::<9>::from_i128(2);
        let c = a.div(b);
        assert_eq!(c.to_i128(), 0);
        let a = FixedDecimal::<9>::from_i128(5);
        let b = FixedDecimal::<9>::from_i128(3);
        let c = a.div(b);
        assert_eq!(c.to_i128(), 1);
    }

    #[test]
    fn div_as_float() {
        let a = FixedDecimal::<9>::from_i128(1);
        let b = FixedDecimal::<9>::from_i128(2);
        let c = a.div(b);
        assert_eq!(c.to_float(), 0.5);
        let a = FixedDecimal::<9>::from_i128(5);
        let b = FixedDecimal::<9>::from_i128(3);
        let c = a.div(b);
        assert_eq!(c.to_float(), 1.666666666);
    }

    #[test]
    fn squared() {
        let a = FixedDecimal::<9>::from_i128(2);
        let b = a.squared();
        assert_eq!(b.to_i128(), 4);
    }

    #[test]
    fn scale() {
        let a = FixedDecimal::<9>::scale_const();
        assert_eq!(a, ONE_SCALED_INTEGER);
    }

    #[test]
    fn to_integer() {
        let a = FixedDecimal::<9>::from_i128(1);
        assert_eq!(a.to_i128(), 1);
    }

    #[test]
    fn sum_vec() {
        let vec = vec![
            FixedDecimal::<9>::from_i128(1),
            FixedDecimal::<9>::from_i128(2),
            FixedDecimal::<9>::from_i128(3),
        ];
        assert_eq!(vec.iter().sum::<FixedDecimal<9>>(), 6);
    }

    #[test]
    fn ordering() {
        let a = FixedDecimal::<9>::from_i128(1);
        let b = FixedDecimal::<9>::from_i128(2);
        let c = FixedDecimal::<9>::from_i128(1);

        // Test less than/greater than
        assert!(a < b);
        assert!(b > a);

        // Test equality
        assert!(a == c);
        assert!(!(a != c));

        // Test comparison with integers
        assert!(a < 3);
        assert!(b > 1);
        assert!(a == 1);

        // Test min/max
        assert_eq!(a.min(b), a);
        assert_eq!(a.max(b), b);

        // Test sorting
        let mut vec = vec![b, a, c];
        vec.sort();
        assert_eq!(vec, vec![a, c, b]);
    }

    #[test]
    fn ln2() {
        let a = FixedDecimal::<18>::ln2();
        assert_eq!(a.to_string(), "0.693147180559945309");
    }

    #[test]
    fn e() {
        let a = FixedDecimal::<18>::e();
        assert_eq!(a.to_string(), "2.718281828459045235");
    }

    #[test]
    fn pi() {
        let a = FixedDecimal::<18>::pi();
        assert_eq!(a.to_string(), "3.141592653589793238");
    }

    #[test]
    fn negatives() {
        let a = FixedDecimal::<18>::from_i128(-10);
        assert_eq!(a.to_string(), "-10");
        let b: FixedDecimal<18> = a / 2;
        assert_eq!(b.to_string(), "-5");
        let c = FixedDecimal::<18>::from_str("-12.231231").unwrap();
        assert_eq!(c.to_string(), "-12.231231");
    }
}
