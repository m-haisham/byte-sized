use crate::{
    algorithm::Algorithm,
    emit::{EmitResult, EmitVec},
};

pub struct MergeSort;

impl Algorithm for MergeSort {
    fn name(&self) -> String {
        String::from("MergeSort")
    }

    fn sort(&self, source: &mut EmitVec) -> EmitResult<()> {
        self.boostrap(source)
    }
}

impl MergeSort {
    fn boostrap(&self, source: &mut EmitVec) -> EmitResult<()> {
        if source.len() > 1 {
            let mid = source.len() / 2;
            let len = source.len();

            let mut left = EmitVec::clone_slice(&source, 0..mid)?;
            let mut right = EmitVec::clone_slice(&source, mid..len)?;

            self.boostrap(&mut left)?;
            self.boostrap(&mut right)?;

            left.set_highlight(false);
            right.set_highlight(false);

            let (mut i, mut j, mut k) = (0, 0, 0);

            while i < left.len() && j < right.len() {
                if left.get(i)? < right.get(j)? {
                    source.set(k, left.get(i)?)?;
                    i += 1;
                } else {
                    source.set(k, right.get(j)?)?;
                    j += 1;
                }

                k += 1;
            }

            while i < left.len() {
                source.set(k, left.get(i)?)?;
                i += 1;
                k += 1;
            }

            while j < right.len() {
                source.set(k, right.get(j)?)?;
                j += 1;
                k += 1;
            }
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

        MergeSort.sort(&mut vec)?;

        assert_eq!(&array, &[0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);

        Ok(())
    }
}
