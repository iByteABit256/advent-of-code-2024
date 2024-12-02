advent_of_code::solution!(2);

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports = parse_input(input);
    let safe_reports = reports
        .into_iter()
        .filter(|rep| is_report_safe(rep))
        .count() as u32;

    Some(safe_reports)
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = parse_input(input);
    let mut safe_reports = 0;

    for rep in &reports {
        let mut rep_safe = is_report_safe(rep);

        if !rep_safe {
            rep_safe = second_chance(rep);
        }

        if rep_safe {
            safe_reports += 1;
        }
    }

    Some(safe_reports)
}

fn second_chance(rep: &[i32]) -> bool {
    for remove_idx in 0..rep.len() {
        let altered_rep: Vec<_> = rep
            .iter()
            .enumerate()
            .filter(|&(j, _)| j != remove_idx)
            .map(|(_, &val)| val)
            .collect();

        if is_report_safe(&altered_rep) {
            return true;
        }
    }

    false
}

fn is_report_safe(rep: &[i32]) -> bool {
    let mut rep_safe = true;

    let first_diff = rep[0] - rep[1];

    if first_diff == 0 {
        return false;
    }
    let direction = first_diff / first_diff.abs();

    for i in 0..rep.len() - 1 {
        let diff = rep[i] - rep[i + 1];

        if diff * direction < 0 || diff.abs() < 1 || diff.abs() > 3 {
            rep_safe = false;
            break;
        }
    }

    rep_safe
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
