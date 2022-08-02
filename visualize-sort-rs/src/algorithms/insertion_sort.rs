use crate::{
    algorithm::Algorithm,
    emit::{EmitResult, EmitVec},
};

#[derive(Clone)]
pub struct InsertionSort;

impl Algorithm for InsertionSort {
    fn name(&self) -> String {
        String::from("InsertionSort")
    }

    fn sort(&self, source: &mut EmitVec) -> EmitResult<()> {
        for index in 0..source.len() {
            let key = source.get(index)?;
            let mut j = (index as i32) - 1;

            while j > -1 && key < source.get(j as usize)? {
                source.set((j + 1) as usize, source.get(j as usize)?)?;
                j -= 1;
            }

            source.set((j + 1) as usize, key)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;

    use super::*;

    #[test]
    fn test_sort() -> EmitResult<()> {
        let (tx, _rx) = mpsc::channel();
        let mut array = [6.0, 8.0, 7.0, 4.0, 3.0, 2.0, 1.0, 0.0, 9.0, 5.0];
        let mut vec = EmitVec::borrow(&tx, &mut array, 0);

        InsertionSort.sort(&mut vec)?;

        assert_eq!(&array, &[0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);

        Ok(())
    }
}
