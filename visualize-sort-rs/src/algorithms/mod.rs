use crate::algorithm::Algorithm;

use self::{insertion_sort::InsertionSort, quicksort::QuickSort};

mod insertion_sort;
mod quicksort;

pub fn algorithms() -> [Box<dyn Algorithm>; 2] {
    [Box::new(QuickSort), Box::new(InsertionSort)]
}
