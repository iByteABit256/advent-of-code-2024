use std::collections::HashMap;

advent_of_code::solution!(1);

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in input.lines() {
        if let Some((first, second)) = line.split_once(' ') {
            if let (Ok(num1), Ok(num2)) =
                (first.trim().parse::<u32>(), second.trim().parse::<u32>())
            {
                list1.push(num1);
                list2.push(num2);
            }
        }
    }

    (list1, list2)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut l1, mut l2) = parse_input(input);

    l1.sort();
    l2.sort();

    let mut sum: u32 = 0;

    for i in 0..l1.len() {
        sum += (l1[i] as i32 - l2[i] as i32).unsigned_abs();
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (l1, l2) = parse_input(input);

    let mut appearances: HashMap<u32, u32> = HashMap::new();
    let mut sum: u32 = 0;

    for n in l1 {
        match appearances.get(&n) {
            Some(count) => sum += n * count,
            None => {
                let count = l2.iter().filter(|&num| *num == n).count() as u32;
                appearances.insert(n, count);
                sum += n * count;
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2285373));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
