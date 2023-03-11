//! 1. Two Sum
//!
//! Given an array of integers nums and an integer target,
//! return indices of the two numbers such that they add up to target.
//!
//! You may assume that each input would have exactly one solution,
//! and you may not use the same element twice.
//!
//! You can return the answer in any order.
//!
//! https://leetcode.com/problems/two-sum/

use std::{collections::HashMap, env, error, process::exit};

use indoc::indoc;

const USAGE: &'static str = indoc! {"
    Usage: two_sum <nums:int> <target:int>

    <nums>   = Array of integers
    <target> = Integer target

    Example: two_sum 2,7,11,15 9
"};

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut args = env::args();
    args.next();

    let nums = match args.next() {
        Some(values) => values
            .split(",")
            .map(|value| value.parse())
            .collect::<Result<_, _>>()?,
        None => show_usage_and_quit(),
    };

    let target = match args.next() {
        Some(value) => value.parse()?,
        None => show_usage_and_quit(),
    };

    let result = two_sum(nums, target);
    println!("{result:?}");

    Ok(())
}

fn show_usage_and_quit() -> ! {
    println!("{USAGE}");
    exit(0);
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::<i32, usize>::new();
    for (i, num) in nums.into_iter().enumerate() {
        let required = target - num;
        if let Some(other) = map.get(&required) {
            return vec![*other as i32, i as i32];
        }

        map.insert(num, i);
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::two_sum;

    #[test]
    fn test_two_sum() {
        assert_eq!(vec![0, 1], two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], two_sum(vec![3, 2, 4], 6));
        assert_eq!(vec![0, 1], two_sum(vec![3, 3], 6));
    }
}
