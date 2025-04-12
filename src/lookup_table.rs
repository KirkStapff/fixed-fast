use crate::{
    error::{FixedFastError, Result},
    fixed_decimal::{Fixed, FixedDecimal},
};

pub struct LookupTable<T: Fixed> {
    pub table: Vec<FixedDecimal<T>>,
    pub start: FixedDecimal<T>,
    pub end: FixedDecimal<T>,
    pub step_size: FixedDecimal<T>,
}

impl<T: Fixed> LookupTable<T> {
    pub fn new(
        start: FixedDecimal<T>,
        end: FixedDecimal<T>,
        step_size: FixedDecimal<T>,
        f: impl Fn(FixedDecimal<T>) -> FixedDecimal<T>,
    ) -> Self {
        let table_size = ((end.sub(start)).div(step_size)).to_i128() as usize;
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

    pub fn get_index(&self, x: FixedDecimal<T>) -> Result<usize> {
        if x < self.start || x > self.end {
            return Err(FixedFastError::OutOfRange(x.to_i128() as usize));
        }
        let index = ((x.sub(self.start)).div(self.step_size)).to_i128() as usize;
        Ok(index)
    }

    pub fn step_size(&self) -> FixedDecimal<T> {
        self.step_size
    }

    pub fn start(&self) -> FixedDecimal<T> {
        self.start
    }

    pub fn end(&self) -> FixedDecimal<T> {
        self.end
    }
}
