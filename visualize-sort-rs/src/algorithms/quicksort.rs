use crate::{
    algorithm::Algorithm,
    emit::{EmitResult, EmitVec},
};

#[derive(Clone)]
pub struct QuickSort;

impl Algorithm for QuickSort {
    fn name(&self) -> String {
        String::from("QuickSort")
    }

    fn sort(&self, source: &mut EmitVec) -> EmitResult<()> {
        Self::bootstrap(source, 0, (source.len() - 1) as i32)
    }
}

impl QuickSort {
    fn bootstrap(source: &mut EmitVec, low: i32, high: i32) -> EmitResult<()> {
        if low < high {
            let pivot = Self::partition(source, low, high)?;

            Self::bootstrap(source, low, pivot - 1)?;
            Self::bootstrap(source, pivot + 1, high)?;
        };

        Ok(())
    }

    fn partition(source: &mut EmitVec, low: i32, high: i32) -> EmitResult<i32> {
        let pivot = source.get(high as usize)?;
        let mut i = low - 1;

        for j in (low as usize)..(high as usize) {
            if source.get(j)? < pivot {
                i += 1;
                source.swap(i as usize, j)?;
            }
        }

        source.swap((i + 1) as usize, high as usize)?;
        return Ok(i + 1);
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::mpsc, thread};

    use super::*;

    #[test]
    fn test_sort() -> EmitResult<()> {
        let (tx, _rx) = mpsc::channel();

        let mut array = [6.0, 8.0, 7.0, 4.0, 3.0, 2.0, 1.0, 0.0, 9.0, 5.0];
        let mut vec = EmitVec::new(&tx, &mut array, 0);

        QuickSort.sort(&mut vec)?;

        assert_eq!(&array, &[0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
        Ok(())
    }
}
