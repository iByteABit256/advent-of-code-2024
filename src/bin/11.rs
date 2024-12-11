advent_of_code::solution!(11);

fn parse_input(input: &str) -> HashMap<u64, u64> {
    input
        .split_whitespace()
        .map(|s| (s.parse::<u64>().expect("String is not a number"), 1))
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut input = parse_input(input);
    for _ in 0..25 {
        input = blink(&input);
    }

    Some(input.values().sum::<u64>())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut input = parse_input(input);
    for _ in 0..75 {
        input = blink(&input);
    }

    Some(input.values().sum::<u64>())
}

use std::collections::HashMap;

fn blink(stones: &HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_stones = HashMap::with_capacity(stones.len());

    for (&stone, count) in stones {
        match stone {
            0 => *new_stones.entry(1).or_default() += count,
            _ => {
                let d = count_digits(stone);
                if d % 2 == 0 {
                    let (l, r) = split_digits(stone, d);
                    *new_stones.entry(l).or_default() += count;
                    *new_stones.entry(r).or_default() += count;
                } else {
                    *new_stones.entry(stone * 2024).or_default() += count;
                }
            }
        }
    }

    new_stones
}

fn count_digits(n: u64) -> u64 {
    n.ilog10() as u64 + 1
}

fn split_digits(n: u64, d: u64) -> (u64, u64) {
    let divisor = 10u64.pow((d / 2) as u32);
    let l = n / divisor;
    let r = n % divisor;
    (l, r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
