use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse_operations(input, false)
            .iter()
            .map(|(x, y)| x * y)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        parse_operations(input, true)
            .iter()
            .map(|(x, y)| x * y)
            .sum(),
    )
}

fn parse_operations(input: &str, enable_dodonts: bool) -> Vec<(u32, u32)> {
    let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))|(don't\(\))|(do\(\))").unwrap();

    let mut result = Vec::new();
    let mut is_active = true;

    for caps in re.captures_iter(input) {
        match (
            caps.get(1).map(|m| m.as_str()),
            caps.get(4).is_some(),
            caps.get(5).is_some(),
        ) {
            (Some(_), false, false) if is_active => {
                if let (Ok(n1), Ok(n2)) = (
                    caps.get(2).unwrap().as_str().parse::<u32>(),
                    caps.get(3).unwrap().as_str().parse::<u32>(),
                ) {
                    result.push((n1, n2));
                }
            }
            (None, true, false) if enable_dodonts => is_active = false,
            (None, false, true) if enable_dodonts => is_active = true,
            _ => {}
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
