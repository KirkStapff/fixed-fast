use crate::{
    fixed_decimal::FixedDecimal, function::Function, interpolation::linear_interpolation,
    lookup_table::LookupTable,
};

pub type Ln<const DECIMALS: u32> = LnLinearInterpLookupTable<DECIMALS, 12>;

pub struct LnArcTanhExpansion<const DECIMALS: u32, const APPROX_DEPTH: u32> {}

impl<const DECIMALS: u32, const APPROX_DEPTH: u32> Function<DECIMALS>
    for LnArcTanhExpansion<DECIMALS, APPROX_DEPTH>
{
    fn evaluate(self, x: FixedDecimal<DECIMALS>) -> FixedDecimal<DECIMALS> {
        arctanh_ln::<DECIMALS, APPROX_DEPTH>(x)
    }
}

impl<const DECIMALS: u32, const APPROX_DEPTH: u32> LnArcTanhExpansion<DECIMALS, APPROX_DEPTH> {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct LnLinearInterpLookupTable<const DECIMALS: u32, const APPROX_DEPTH: u32> {
    lookup: LookupTable<DECIMALS>,
}

impl<const DECIMALS: u32, const APPROX_DEPTH: u32>
    LnLinearInterpLookupTable<DECIMALS, APPROX_DEPTH>
{
    pub fn new(
        start: FixedDecimal<DECIMALS>,
        end: FixedDecimal<DECIMALS>,
        step_size: FixedDecimal<DECIMALS>,
    ) -> Self {
        Self {
            lookup: LookupTable::new(start, end, step_size, arctanh_ln::<DECIMALS, APPROX_DEPTH>),
        }
    }
}

impl<const DECIMALS: u32, const APPROX_DEPTH: u32> Function<DECIMALS>
    for LnLinearInterpLookupTable<DECIMALS, APPROX_DEPTH>
{
    fn evaluate(self, x: FixedDecimal<DECIMALS>) -> FixedDecimal<DECIMALS> {
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

fn arctanh_ln<const DECIMALS: u32, const APPROX_DEPTH: u32>(
    input: FixedDecimal<DECIMALS>,
) -> FixedDecimal<DECIMALS> {
    let ln2 = FixedDecimal::<DECIMALS>::from_str("0.6931471805599453094172321214581765680")
        .expect("Invalid ln2");
    let mut shift_coef = 0;
    let mut input = input;
    if input == 0 {
        panic!("ln(0) is undefined");
    }
    while input > 2 {
        input >>= 1;
        shift_coef += 1;
    }
    while input < 1 {
        input <<= 1;
        shift_coef -= 1;
    }
    // ln(x) = 2 arctanh(x - 1 / x + 1) logarithmic expansion via inverse hyperbolic tangent

    let arctan_term = (input - 1_i64) / (input + 1_i64);
    let arctan_term_squared = arctan_term * arctan_term;
    let mut nth_term = arctan_term;
    let mut running_sum = nth_term;
    for n in 1..APPROX_DEPTH {
        nth_term = nth_term * arctan_term_squared / (2 * n as i64 + 1);
        running_sum += nth_term;
    }
    2_usize * running_sum + shift_coef as usize * ln2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        assert_eq!(
            arctanh_ln::<18, 10>(FixedDecimal::<18>::from_integer(1)),
            FixedDecimal::<18>::from_integer(0)
        );
        let input = FixedDecimal::<18>::from_str("1.4").unwrap();
        assert_eq!(
            arctanh_ln::<18, 10>(input),
            FixedDecimal::<18>::from_str("0.336436968116129286").unwrap()
        );
        let input = FixedDecimal::<18>::from_str("69.3").unwrap();
        assert_eq!(
            arctanh_ln::<18, 10>(input),
            FixedDecimal::<18>::from_str("4.238444879656876612").unwrap()
        );
    }

    #[test]
    fn test_lookup_table() {
        let ln = LnLinearInterpLookupTable::<18, 10>::new(
            FixedDecimal::<18>::from_str("0.000000001").unwrap(),
            FixedDecimal::<18>::from_str("10").unwrap(),
            FixedDecimal::<18>::from_str("0.00001").unwrap(),
        );
        let input = FixedDecimal::<18>::from_str("1.3453453453453453").unwrap();
        assert_eq!(
            ln.evaluate(input),
            FixedDecimal::<18>::from_str("0.296631876146752907").unwrap()
        );
    }
}
