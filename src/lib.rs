mod error;
mod fixed_decimal;
mod function;
mod interpolation;
mod ln;
mod lookup_table;
mod sqrt;

pub use fixed_decimal::FixedDecimal;
pub use ln::{Ln, LnArcTanhExpansion, LnLinearInterpLookupTable};
pub use sqrt::{Sqrt, SqrtLinearInterpLookupTable, SqrtNewtonRaphson};

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
}
