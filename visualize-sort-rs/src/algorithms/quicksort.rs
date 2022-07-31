use crate::algorithm::Algorithm;

pub struct QuickSort;

impl Algorithm for QuickSort {
    fn name() -> String {
        String::from("QuickSort")
    }

    fn sort(source: &mut Vec<f32>) {
        Self::bootstrap(source, 0, (source.len() - 1) as i32)
    }
}

impl QuickSort {
    fn bootstrap(source: &mut Vec<f32>, low: i32, high: i32) {
        if low < high {
            let pivot = Self::partition(source, low, high);

            Self::bootstrap(source, low, pivot - 1);
            Self::bootstrap(source, pivot + 1, high);
        }
    }

    fn partition(source: &mut Vec<f32>, low: i32, high: i32) -> i32 {
        let pivot = source[high as usize];
        let mut i = low - 1;

        for j in (low as usize)..(high as usize) {
            if source[j] < pivot {
                i += 1;

                let temp = source[i as usize];
                source[i as usize] = source[j];
                source[j] = temp;
            }
        }

        let temp = source[(i + 1) as usize];
        source[(i + 1) as usize] = source[high as usize];
        source[high as usize] = temp;

        return i + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let mut source = vec![6.0, 8.0, 7.0, 4.0, 3.0, 2.0, 1.0, 0.0, 9.0, 5.0];
        QuickSort::sort(&mut source);

        assert_eq!(
            source,
            vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]
        );
    }
}
