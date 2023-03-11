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

use std::{collections::HashMap, env, error, num::ParseIntError, process::exit, str::FromStr};

use clap::{arg, command, value_parser};

#[derive(Clone)]
struct Nums(Vec<i32>);

impl FromStr for Nums {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = s
            .split(",")
            .map(|v| v.trim().parse())
            .collect::<Result<_, _>>()?;
        Ok(Nums(inner))
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let matches = command!()
        .arg(arg!([nums] "Array of integers to search on").value_parser(value_parser!(Nums)))
        .arg(arg!([target] "The target sum").value_parser(value_parser!(i32)))
        .get_matches();

    let Some(Nums(nums)) = matches.get_one::<Nums>("nums") else { exit(0) };
    let Some(target) = matches.get_one::<i32>("target") else { exit(0) };

    let result = two_sum(nums.clone(), *target);
    println!("{result:?}");

    Ok(())
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
