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
        let a = FixedDecimal::<9>::from_integer(1);
        let b = FixedDecimal::<9>::from_integer(2);
        let c = a.mul(b);
        assert_eq!(c.to_integer(), 2);
    }

    #[test]
    fn div() {
        let a = FixedDecimal::<9>::from_integer(1);
        let b = FixedDecimal::<9>::from_integer(2);
        let c = a.div(b);
        assert_eq!(c.to_integer(), 0);
        let a = FixedDecimal::<9>::from_integer(5);
        let b = FixedDecimal::<9>::from_integer(3);
        let c = a.div(b);
        assert_eq!(c.to_integer(), 1);
    }

    #[test]
    fn div_as_float() {
        let a = FixedDecimal::<9>::from_integer(1);
        let b = FixedDecimal::<9>::from_integer(2);
        let c = a.div(b);
        assert_eq!(c.to_float(), 0.5);
        let a = FixedDecimal::<9>::from_integer(5);
        let b = FixedDecimal::<9>::from_integer(3);
        let c = a.div(b);
        assert_eq!(c.to_float(), 1.666666666);
    }

    #[test]
    fn squared() {
        let a = FixedDecimal::<9>::from_integer(2);
        let b = a.squared();
        assert_eq!(b.to_integer(), 4);
    }

    #[test]
    fn scale() {
        let a = FixedDecimal::<9>::scale_const();
        assert_eq!(a, ONE_SCALED_INTEGER);
    }

    #[test]
    fn to_integer() {
        let a = FixedDecimal::<9>::from_integer(1);
        assert_eq!(a.to_integer(), 1);
    }
}
