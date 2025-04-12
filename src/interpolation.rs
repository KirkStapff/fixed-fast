use crate::fixed_decimal::{Fixed, FixedDecimal};

pub fn linear_interpolation<T: Fixed>(
    x: FixedDecimal<T>,
    x1: FixedDecimal<T>,
    x2: FixedDecimal<T>,
    y1: FixedDecimal<T>,
    y2: FixedDecimal<T>,
) -> FixedDecimal<T> {
    let dx = x2.sub(x1);
    let dy = y2.sub(y1);
    let t = x.sub(x1).div(dx);
    y1.add(t.mul(dy))
}
