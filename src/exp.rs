use crate::{FixedDecimal, function::Function};

pub struct ExpRangeReduceTaylor<const DECIMALS: u32, const TAYLOR_ORDER: u32> {}

impl<const DECIMALS: u32, const TAYLOR_ORDER: u32> ExpRangeReduceTaylor<DECIMALS, TAYLOR_ORDER> {
    pub fn new() -> Self {
        Self {}
    }
}

impl<const DECIMALS: u32, const TAYLOR_ORDER: u32> Function<DECIMALS>
    for ExpRangeReduceTaylor<DECIMALS, TAYLOR_ORDER>
{
    fn evaluate(&self, x: FixedDecimal<DECIMALS>) -> FixedDecimal<DECIMALS> {
        range_reduce_taylor_exp::<DECIMALS, TAYLOR_ORDER>(x)
    }
}

fn range_reduce_taylor_exp<const DECIMALS: u32, const TAYLOR_ORDER: u32>(
    x: FixedDecimal<DECIMALS>,
) -> FixedDecimal<DECIMALS> {
    let ln2 = FixedDecimal::<DECIMALS>::ln2();
    let k = (x / ln2).floor_i128();
    let r = x - ln2 * FixedDecimal::from_i128(k);

    let mut term = FixedDecimal::<DECIMALS>::from_i128(1);
    let mut result = term;
    for i in 1..=TAYLOR_ORDER {
        term = term * r / i;
        result += term;
    }
    let range_gain = FixedDecimal::<DECIMALS>::two_pow_k(k as i32);
    result * range_gain
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_reduce_taylor_exp() {
        let x = FixedDecimal::<18>::from_str("1.0").unwrap();
        assert_eq!(
            range_reduce_taylor_exp::<18, 10>(x),
            FixedDecimal::<18>::from_str("2.718281828458928456").unwrap()
        );
        let x = FixedDecimal::<12>::from_str("-1.231231").unwrap();
        assert_eq!(
            range_reduce_taylor_exp::<12, 20>(x),
            FixedDecimal::<12>::from_str("0.291932986891").unwrap()
        );
        let x = FixedDecimal::<12>::from_str("0").unwrap();
        assert_eq!(
            range_reduce_taylor_exp::<12, 20>(x),
            FixedDecimal::<12>::from_str("1").unwrap()
        );
    }
}
