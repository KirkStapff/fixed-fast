use crate::{
    error::{FixedFastError, Result},
    fixed_decimal::FixedDecimal,
};

pub struct LookupTable<const DECIMALS: u32> {
    pub table: Vec<FixedDecimal<DECIMALS>>,
    pub start: FixedDecimal<DECIMALS>,
    pub end: FixedDecimal<DECIMALS>,
    pub step_size: FixedDecimal<DECIMALS>,
}

impl<const DECIMALS: u32> LookupTable<DECIMALS> {
    pub fn new(
        start: FixedDecimal<DECIMALS>,
        end: FixedDecimal<DECIMALS>,
        step_size: FixedDecimal<DECIMALS>,
        f: impl Fn(FixedDecimal<DECIMALS>) -> FixedDecimal<DECIMALS>,
    ) -> Self {
        let table_size = ((end.sub(start)).div(step_size)).to_integer() as usize;
        let mut table = Vec::new();
        for i in 0..table_size {
            let x = start + step_size * i;
            table.push(f(x));
        }
        Self {
            table,
            start,
            end,
            step_size,
        }
    }

    pub fn get_index(&self, x: FixedDecimal<DECIMALS>) -> Result<usize> {
        if x < self.start || x > self.end {
            return Err(FixedFastError::OutOfRange(x.to_integer() as usize));
        }
        let index = ((x.sub(self.start)).div(self.step_size)).to_integer() as usize;
        Ok(index)
    }

    pub fn step_size(&self) -> FixedDecimal<DECIMALS> {
        self.step_size
    }

    pub fn start(&self) -> FixedDecimal<DECIMALS> {
        self.start
    }

    pub fn end(&self) -> FixedDecimal<DECIMALS> {
        self.end
    }
}
