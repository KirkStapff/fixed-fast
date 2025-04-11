use crate::fixed_decimal::FixedDecimal;

pub fn linear_interpolation<const DECIMALS: u32>(
    x: FixedDecimal<DECIMALS>,
    x1: FixedDecimal<DECIMALS>,
    x2: FixedDecimal<DECIMALS>,
    y1: FixedDecimal<DECIMALS>,
    y2: FixedDecimal<DECIMALS>,
) -> FixedDecimal<DECIMALS> {
    let dx = x2.sub(x1);
    let dy = y2.sub(y1);
    let t = x.sub(x1).div(dx);
    y1.add(t.mul(dy))
}
