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

pub use cdf::{CDFBowlingRational, CDFLinearInterpLookupTable, CDFV1};
pub use exp::{ExpLinearInterpLookupTable, ExpRangeReduceTaylor, ExpV1};
pub use fixed_decimal::{FixedDecimal, FixedPrecision};
pub use function::Function;
pub use ln::{LnArcTanhExpansion, LnLinearInterpLookupTable, LnV1};
pub use pdf::{PDFLinearInterpLookupTable, PDFV1};
pub use sqrt::{SqrtLinearInterpLookupTable, SqrtNewtonRaphson, SqrtV1};
#[cfg(test)]
mod tests {
    use crate::fixed_decimal::{FixedDecimal, FixedPrecision};

    const ONE_SCALED_INTEGER: i128 = 1000000000;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    struct F9;
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    struct F18;

    impl FixedPrecision for F9 {
        const PRECISION: u32 = 9;
    }

    impl FixedPrecision for F18 {
        const PRECISION: u32 = 18;
    }

    #[test]
    fn mul() {
        let a = FixedDecimal::<F9>::from_i128(1);
        let b = FixedDecimal::<F9>::from_i128(2);
        let c = a.mul(b);
        assert_eq!(c.to_i128(), 2);
        assert_eq!(c * 2_u64, FixedDecimal::<F9>::from_i128(4));
        let a = FixedDecimal::<F9>::from_i128(1325235);
        let b = FixedDecimal::<F9>::from_i128(3123123);
        let c = a.mul(b);
        assert_eq!(c.to_f64(), 4138871908905.0);
        let a = FixedDecimal::<F9>::from_i128(1325235);
        let b = 3123123;
        let c: FixedDecimal<F9> = a * b;
        assert_eq!(c.to_f64(), 4138871908905.0);
        let d: FixedDecimal<F9> = b * a;
        assert_eq!(d.to_f64(), 4138871908905.0);
    }

    #[test]
    fn div() {
        let a = FixedDecimal::<F9>::from_i128(1);
        let b = FixedDecimal::<F9>::from_i128(2);
        let c = a.div(b);
        assert_eq!(c.to_i128(), 0);
        let a = FixedDecimal::<F9>::from_i128(5);
        let b = FixedDecimal::<F9>::from_i128(3);
        let c = a.div(b);
        assert_eq!(c.to_i128(), 1);
        let a = FixedDecimal::<F9>::from_i128(1325235);
        let b = FixedDecimal::<F9>::from_i128(3123123);
        let c = a.div(b);
        assert_eq!(c.to_f64(), 0.424330069);
        let a = FixedDecimal::<F9>::from_i128(1325235);
        let b = 3123123;
        let c: FixedDecimal<F9> = a / b;
        assert_eq!(c.to_f64(), 0.424330069);
        let d: FixedDecimal<F9> = b / a;
        assert_eq!(d.to_f64(), 2.356655989);
    }

    #[test]
    fn add() {
        let a = FixedDecimal::<F9>::from_i128(1);
        let b = FixedDecimal::<F9>::from_i128(2);
        let c = a + b;
        assert_eq!(c.to_i128(), 3);
        let a = FixedDecimal::<F9>::from_i128(1325235);
        let b = FixedDecimal::<F9>::from_i128(3123123);
        let c = a + b;
        assert_eq!(c.to_f64(), 4448358.0);
        let a = FixedDecimal::<F9>::from_i128(1325235);
        let b = 3123123;
        let c: FixedDecimal<F9> = a + b;
        assert_eq!(c.to_f64(), 4448358.0);
        let d: FixedDecimal<F9> = b + a;
        assert_eq!(d.to_f64(), 4448358.0);
    }

    #[test]
    fn sub() {
        let a = FixedDecimal::<F9>::from_i128(1);
        let b = FixedDecimal::<F9>::from_i128(2);
        let c = a - b;
        assert_eq!(c.to_i128(), -1);
        let a = FixedDecimal::<F9>::from_i128(1325235);
        let b = FixedDecimal::<F9>::from_i128(3123123);
        let c = a - b;
        assert_eq!(c.to_f64(), -1797888.0);
        let a = FixedDecimal::<F9>::from_i128(1325235);
        let b = 3123123;
        let c: FixedDecimal<F9> = a - b;
        assert_eq!(c.to_f64(), -1797888.0);
        let d: FixedDecimal<F9> = b - a;
        assert_eq!(d.to_f64(), 1797888.0);
    }

    #[test]
    fn mul_assign() {
        let mut a = FixedDecimal::<F9>::from_i128(1);
        let b = FixedDecimal::<F9>::from_i128(2);
        a *= b;
        assert_eq!(a.to_i128(), 2);
        let mut a = FixedDecimal::<F9>::from_i128(1325235);
        let b = FixedDecimal::<F9>::from_i128(3123123);
        a *= b;
        assert_eq!(a.to_f64(), 4138871908905.0);
        let mut a = FixedDecimal::<F9>::from_i128(1325235);
        let b = 3123123;
        a *= b;
        assert_eq!(a.to_f64(), 4138871908905.0);
    }

    #[test]
    fn div_assign() {
        let mut a = FixedDecimal::<F9>::from_i128(1);
        let b = FixedDecimal::<F9>::from_i128(2);
        a /= b;
        assert_eq!(a.to_f64(), 0.5);
        let mut a = FixedDecimal::<F9>::from_i128(1325235);
        let b = FixedDecimal::<F9>::from_i128(3123123);
        a /= b;
        assert_eq!(a.to_f64(), 0.424330069);
        let mut a = FixedDecimal::<F9>::from_i128(1325235);
        let b = 3123123;
        a /= b;
        assert_eq!(a.to_f64(), 0.424330069);
    }

    #[test]
    fn add_assign() {
        let mut a = FixedDecimal::<F9>::from_i128(1);
        let b = FixedDecimal::<F9>::from_i128(2);
        a += b;
        assert_eq!(a.to_f64(), 3.0);
        let mut a = FixedDecimal::<F9>::from_i128(1325235);
        let b = FixedDecimal::<F9>::from_i128(3123123);
        a += b;
        assert_eq!(a.to_f64(), 4448358.0);
        let mut a = FixedDecimal::<F9>::from_i128(1325235);
        let b = 3123123;
        a += b;
    }

    #[test]
    fn neg() {
        let a = FixedDecimal::<F9>::from_i128(1);
        let b = -a;
        assert_eq!(b.to_i128(), -1);
    }

    #[test]
    fn sub_assign() {
        let mut a = FixedDecimal::<F9>::from_i128(1);
        let b = FixedDecimal::<F9>::from_i128(2);
        a -= b;
        assert_eq!(a.to_i128(), -1);
        let mut a = FixedDecimal::<F9>::from_i128(1325235);
        let b = FixedDecimal::<F9>::from_i128(3123123);
        a -= b;
        assert_eq!(a.to_f64(), -1797888.0);
        let mut a = FixedDecimal::<F9>::from_i128(1325235);
        let b = 3123123;
    }

    #[test]
    fn div_as_float() {
        let a = FixedDecimal::<F9>::from_i128(1);
        let b = FixedDecimal::<F9>::from_i128(2);
        let c = a.div(b);
        assert_eq!(c.to_f64(), 0.5);
        let a = FixedDecimal::<F9>::from_i128(5);
        let b = FixedDecimal::<F9>::from_i128(3);
        let c = a.div(b);
        assert_eq!(c.to_f64(), 1.666666666);
        let a = FixedDecimal::<F9>::from_i128(1325235);
        let b = FixedDecimal::<F9>::from_i128(3123123);
        let c = a.div(b);
        assert_eq!(c.to_f64(), 0.424330069);
    }

    #[test]
    fn squared() {
        let a = FixedDecimal::<F9>::from_i128(2);
        let b = a.squared();
        assert_eq!(b.to_i128(), 4);
        let a = FixedDecimal::<F9>::from_i128(1325);
        let b = a.squared();
        assert_eq!(b.to_f64(), 1755625.0);
    }

    #[test]
    fn scale() {
        let a = FixedDecimal::<F9>::scale();
        assert_eq!(a, ONE_SCALED_INTEGER);
    }

    #[test]
    fn to_integer() {
        let a = FixedDecimal::<F9>::from_i128(1);
        assert_eq!(a.to_i128(), 1);
    }

    #[test]
    fn sum_vec() {
        let vec = vec![
            FixedDecimal::<F9>::from_i128(1),
            FixedDecimal::<F9>::from_i128(2),
            FixedDecimal::<F9>::from_i128(3),
        ];
        assert_eq!(vec.iter().sum::<FixedDecimal<F9>>(), 6);
    }

    #[test]
    fn ordering() {
        let a = FixedDecimal::<F9>::from_i128(1);
        let b = FixedDecimal::<F9>::from_i128(2);
        let c = FixedDecimal::<F9>::from_i128(1);

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
        let a = FixedDecimal::<F18>::ln2();
        assert_eq!(a.to_string(), "0.693147180559945309");
    }

    #[test]
    fn e() {
        let a = FixedDecimal::<F18>::e();
        assert_eq!(a.to_string(), "2.718281828459045235");
    }

    #[test]
    fn pi() {
        let a = FixedDecimal::<F18>::pi();
        assert_eq!(a.to_string(), "3.141592653589793238");
    }

    #[test]
    fn negatives() {
        let a = FixedDecimal::<F18>::from_i128(-10);
        assert_eq!(a.to_string(), "-10");
        let b: FixedDecimal<F18> = a / 2;
        assert_eq!(b.to_string(), "-5");
        let c = FixedDecimal::<F18>::from_str("-12.231231").unwrap();
        assert_eq!(c.to_string(), "-12.231231");
    }
}
