use crate::algorithm::Algorithm;

use self::quicksort::QuickSort;

mod quicksort;

pub fn algorithms() -> [impl Algorithm; 1] {
    [QuickSort]
}
