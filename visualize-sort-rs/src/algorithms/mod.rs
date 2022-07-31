use crate::algorithm::Algorithm;

use self::quicksort::QuickSort;

mod quicksort;

pub fn algorithms() -> [impl Algorithm + Clone; 1] {
    [QuickSort]
}
