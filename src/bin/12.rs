use std::collections::HashMap;

advent_of_code::solution!(12);

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse_input(input);
    let fences = fence(&input);
    println!("Fences: {:#?}", fences);
    Some(fences.values().map(|&(area, fences)| area * fences).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn fence(board: &[Vec<char>]) -> HashMap<char, (u32, u32)> {
    let mut res: HashMap<char, (u32, u32)> = HashMap::new();

    for (idx_x, row) in board.iter().enumerate() {
        for (idx_y, &c) in row.iter().enumerate() {
            let new_area: u32;
            let new_fences: u32;

            if let Some(&(area, fences)) = res.get(&c) {
                new_area = area + 1;
                new_fences = fences + count_fences(board, (idx_x, idx_y));
            } else {
                new_area = 1;
                new_fences = count_fences(board, (idx_x, idx_y));
            }

            res.insert(c, (new_area, new_fences));
        }
    }

    res
}

fn count_fences(board: &[Vec<char>], node: (usize, usize)) -> u32 {
    let x = node.0 as i32;
    let y = node.1 as i32;

    let farm_type = board[x as usize][y as usize];

    [(-1, 0), (0, 1), (1, 0), (0, -1)]
        .iter()
        .map(|&offset| add((x, y), offset))
        .filter(|n| should_fence(board, n, farm_type))
        .count() as u32
}

fn add(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
}

fn should_fence(board: &[Vec<char>], node: &(i32, i32), curr_type: char) -> bool {
    !within_board(board, node) || board[node.0 as usize][node.1 as usize] != curr_type
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
        assert_eq!(result, Some(772));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
