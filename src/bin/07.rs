advent_of_code::solution!(7);

struct Equation {
    test_val: u64,
    numbers: Vec<u64>,
}

impl Equation {
    fn eval(&self, concat_enabled: bool) -> bool {
        eval_rec(self, 0, concat_enabled)
    }
}

fn parse_input(input: &str) -> Vec<Equation> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut parts = line.split(':');
            let test_val = parts
                .next()
                .expect("Missing test value")
                .trim()
                .parse::<u64>()
                .expect("Invalid test value");

            let numbers = parts
                .next()
                .expect("Missing numbers list")
                .split_whitespace()
                .map(|num| num.parse::<u64>().expect("Invalid number"))
                .collect();

            Equation { test_val, numbers }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let equations = parse_input(input);
    Some(
        equations
            .iter()
            .filter(|&eq| eq.eval(false))
            .map(|eq| eq.test_val)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = parse_input(input);
    Some(
        equations
            .iter()
            .filter(|&eq| eq.eval(true))
            .map(|eq| eq.test_val)
            .sum(),
    )
}

fn eval_rec(equation: &Equation, total: u64, concat_enabled: bool) -> bool {
    let test_val = equation.test_val;
    let nums = &equation.numbers;

    if total > test_val {
        return false;
    }

    if nums.is_empty() {
        return total == test_val;
    }

    // Making the assumption that no number will be zero, so zero is assumed to be the first recursive call.
    let mul_total = if total != 0 { total * nums[0] } else { nums[0] };
    let add_total = total + nums[0];

    let new_nums = nums[1..].to_vec();

    let updated_equation = Equation {
        test_val,
        numbers: new_nums,
    };

    eval_rec(&updated_equation, mul_total, concat_enabled)
        || eval_rec(&updated_equation, add_total, concat_enabled)
        || concat_enabled && eval_rec(&updated_equation, concat(total, nums[0]), concat_enabled)
}

fn concat(a: u64, b: u64) -> u64 {
    (a.to_string() + &b.to_string())
        .parse::<u64>()
        .expect("Could not concat")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
