use std::collections::HashSet;

advent_of_code::solution!(10);

struct TopographicMap {
    map: Vec<Vec<u32>>,
    trailheads: Vec<(usize, usize)>,
}

fn parse_input(input: &str) -> TopographicMap {
    let mut trailheads: Vec<(usize, usize)> = Vec::new();

    let map = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| {
                    if c == '0' {
                        trailheads.push((i, j));
                    }
                    c.to_digit(10).expect("Not a digit")
                })
                .collect()
        })
        .collect();

    TopographicMap { map, trailheads }
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse_input(input);
    Some(
        input
            .trailheads
            .iter()
            .map(|&th| count_paths(&input.map, th, false))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse_input(input);
    Some(
        input
            .trailheads
            .iter()
            .map(|&th| count_paths(&input.map, th, true))
            .sum(),
    )
}

fn count_paths(map: &[Vec<u32>], start: (usize, usize), part_two: bool) -> u32 {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut to_visit: Vec<(usize, usize)> = vec![start];
    let mut total_count = 0;

    while let Some(node) = to_visit.pop() {
        if visited.contains(&node) {
            continue;
        }
        
        let height = map[node.0][node.1];

        if height == 9 {
            total_count += 1;
        }

        if !part_two {
            visited.insert(node);
        }

        neighbours(map, node)
            .iter()
            .for_each(|&nb| to_visit.push(nb));
    }

    total_count
}

fn neighbours(map: &[Vec<u32>], node: (usize, usize)) -> Vec<(usize, usize)> {
    let (x, y) = (node.0 as i32, node.1 as i32);
    [(x - 1, y), (x, y + 1), (x + 1, y), (x, y - 1)]
        .iter()
        .filter(|&nb| within_map(map, nb) && is_ascending(map, &(x, y), &(nb.0, nb.1)))
        .map(|&nb| (nb.0 as usize, nb.1 as usize))
        .collect()
}

fn is_ascending(map: &[Vec<u32>], node_a: &(i32, i32), node_b: &(i32, i32)) -> bool {
    let height_a = map[node_a.0 as usize][node_a.1 as usize];
    let height_b = map[node_b.0 as usize][node_b.1 as usize];
    if height_b > height_a {
        height_b - height_a == 1
    } else {
        false
    }
}

fn within_map(map: &[Vec<u32>], node: &(i32, i32)) -> bool {
    let sz_x = map.len() as i32;
    let sz_y = map[0].len() as i32;

    node.0 >= 0 && node.0 < sz_x && node.1 >= 0 && node.1 < sz_y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
