use crate::algorithm::Algorithm;

use self::{insertion_sort::InsertionSort, quicksort::QuickSort};
use lazy_static::lazy_static;

mod insertion_sort;
mod quicksort;

lazy_static! {
    pub static ref ALGORITHMS: [Box<dyn Algorithm>; 2] =
        [Box::new(QuickSort), Box::new(InsertionSort)];
}
