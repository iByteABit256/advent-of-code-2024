use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(12);

#[derive(Debug)]
struct GardenPlot {
    farm_type: char,
    area: u32,
    perimeter: u32,
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse_input(input);
    let fences = fence(&input, false);
    Some(
        fences
            .iter()
            .map(|garden_plot| garden_plot.area * garden_plot.perimeter)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse_input(input);
    let fences = fence(&input, true);
    println!("fences: {:#?}", fences);
    Some(
        fences
            .iter()
            .map(|garden_plot| garden_plot.area * garden_plot.perimeter)
            .sum(),
    )
}

fn fence(board: &[Vec<char>], part_two: bool) -> Vec<GardenPlot> {
    let mut res: Vec<GardenPlot> = Vec::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    for (idx_x, row) in board.iter().enumerate() {
        for (idx_y, _) in row.iter().enumerate() {
            if let Some(garden_plot) = define_plot(board, (idx_x, idx_y), &mut visited, part_two) {
                res.push(garden_plot);
            }
        }
    }

    res
}

fn define_plot(
    board: &[Vec<char>],
    start: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
    part_two: bool,
) -> Option<GardenPlot> {
    let mut to_visit: VecDeque<(usize, usize)> = VecDeque::from([start]);

    let farm_type = board[start.0][start.1];
    let mut area: u32 = 0;
    let mut perimeter: u32 = 0;

    while let Some(node) = to_visit.pop_front() {
        if visited.contains(&node) {
            continue;
        }

        let c = board[node.0][node.1];

        if c == farm_type {
            area += 1;
            perimeter += count_fences(board, node, part_two);
        }

        visited.insert(node);

        neighbours(board, node)
            .iter()
            .for_each(|&nb| to_visit.push_back(nb));
    }

    if area != 0 {
        return Some(GardenPlot { farm_type, area, perimeter });
    }

    None
}

fn neighbours(board: &[Vec<char>], node: (usize, usize)) -> Vec<(usize, usize)> {
    let x = node.0 as i32;
    let y = node.1 as i32;

    let farm_type = board[x as usize][y as usize];

    [(-1, 0), (0, 1), (1, 0), (0, -1)]
        .iter()
        .map(|&offset| add((x, y), offset))
        .filter(|n| !should_fence(board, n, farm_type, false))
        .map(|node| (node.0 as usize, node.1 as usize))
        .collect()
}

fn count_fences(board: &[Vec<char>], node: (usize, usize), part_two: bool) -> u32 {
    let x = node.0 as i32;
    let y = node.1 as i32;

    let farm_type = board[x as usize][y as usize];

    [(-1, 0), (0, 1), (1, 0), (0, -1)]
        .iter()
        .map(|&offset| add((x, y), offset))
        .filter(|n| should_fence(board, n, farm_type, part_two))
        .count() as u32
}

fn add(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
}

fn should_fence(board: &[Vec<char>], node: &(i32, i32), curr_type: char, part_two: bool) -> bool {
    let within_board = within_board(board, node);
    !within_board && !part_two
        || within_board && board[node.0 as usize][node.1 as usize] != curr_type
}

fn within_board(board: &[Vec<char>], node: &(i32, i32)) -> bool {
    let sz_x = board.len() as i32;
    let sz_y = board[0].len() as i32;

    node.0 >= 0 && node.0 < sz_x && node.1 >= 0 && node.1 < sz_y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
