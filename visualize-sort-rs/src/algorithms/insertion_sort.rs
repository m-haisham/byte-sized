use std::sync::mpsc::SendError;

use crate::{algorithm::Algorithm, event::Event, report::ReportedIndex};

#[derive(Clone)]
pub struct InsertionSort;

impl Algorithm for InsertionSort {
    fn name(&self) -> String {
        String::from("InsertionSort")
    }

    fn sort(&self, source: &mut impl ReportedIndex<f32>) -> Result<(), SendError<Event>> {
        for index in 0..source.len() {
            let key = source.get(index)?;
            let mut j = (index as i32) - 1;

            while j > -1 && key < source.get(j as usize)? {
                source.set((j as usize) + 1, source.get(j as usize)?)?;
                j -= 1;
            }

            source.set((j as usize) + 1, key)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::report::TestVec;

    use super::*;

    #[test]
    fn test_sort() -> Result<(), SendError<Event>> {
        let mut source = TestVec(vec![6.0, 8.0, 7.0, 4.0, 3.0, 2.0, 1.0, 0.0, 9.0, 5.0]);
        InsertionSort.sort(&mut source)?;

        assert_eq!(
            source.0,
            vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]
        );

        Ok(())
    }
}
