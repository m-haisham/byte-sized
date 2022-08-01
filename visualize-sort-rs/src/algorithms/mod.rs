use crate::algorithm::Algorithm;

use self::quicksort::QuickSort;

mod quicksort;

pub fn algorithms() -> &'static [impl Algorithm + Clone; 1] {
    &[QuickSort]
}
