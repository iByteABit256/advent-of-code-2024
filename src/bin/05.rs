advent_of_code::solution!(5);

use std::collections::HashMap;

use multimap::MultiMap;

#[derive(Debug)]
struct ParsedInput {
    rules: MultiMap<u32, u32>,
    updates: Vec<Vec<u32>>,
}

fn parse_input(input: &str) -> ParsedInput {
    let mut rules = MultiMap::new();
    let mut updates = Vec::new();

    let mut is_second_section = false;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            is_second_section = true;
            continue;
        }

        if !is_second_section {
            if let Some((left, right)) = line.split_once('|') {
                let key: u32 = right.trim().parse().expect("Invalid number");
                let value: u32 = left.trim().parse().expect("Invalid number");
                rules.insert(key, value);
            }
        } else {
            let nums: Vec<u32> = line
                .split(',')
                .map(|num| num.trim().parse().expect("Invalid number"))
                .collect();
            updates.push(nums);
        }
    }

    ParsedInput { rules, updates }
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse_input(input);
    let valid_updates = filter_updates(input, true);

    Some(valid_updates.iter().map(|up| up[up.len() / 2]).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse_input(input);
    let invalid_updates = filter_updates(input, false);

    Some(invalid_updates.iter().map(|up| up[up.len() / 2]).sum())
}

fn filter_updates(mut input: ParsedInput, valid: bool) -> Vec<Vec<u32>> {
    let mut valid_updates: Vec<Vec<u32>> = Vec::new();
    let mut invalid_updates: Vec<Vec<u32>> = Vec::new();

    for up in &mut input.updates {
        let mut is_valid = true;
        let mut needs_checking = true;
        let mut times_checked = 0;

        while needs_checking {
            is_valid = true;
            let mut incorrect_pages: HashMap<u32, usize> = HashMap::new();

            for idx in 0..up.len() {
                let page = up[idx];

                if let Some(&idx_left) = incorrect_pages.get(&page) {
                    is_valid = false;

                    if valid {
                        needs_checking = false;
                        break;
                    } else {
                        up.swap(idx, idx_left);
                    }
                }

                if let Some(rules) = input.rules.get_vec(&page) {
                    for &r in rules {
                        incorrect_pages.insert(r, idx);
                    }
                }
            }

            if is_valid || valid {
                needs_checking = false;
            }

            times_checked += 1;
        }

        if valid && is_valid {
            valid_updates.push(up.clone());
        } else if times_checked > 1 {
            invalid_updates.push(up.clone());
        }
    }

    if valid {
        valid_updates
    } else {
        invalid_updates
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
