use std::collections::HashSet;

use multimap::MultiMap;

advent_of_code::solution!(8);

#[derive(Debug)]
struct Board {
    antennas: MultiMap<char, (u32, u32)>,
    dimensions: (usize, usize),
}

fn parse_input(input: &str) -> Board {
    let mut antennas = MultiMap::new();
    let lines: Vec<&str> = input.lines().collect();

    let height = lines.len();
    let width = lines[0].len();

    for (row_index, line) in lines.iter().enumerate() {
        for (col_index, c) in line.chars().enumerate() {
            if c.is_alphanumeric() {
                antennas.insert(c, (row_index as u32, col_index as u32));
            }
        }
    }

    Board {
        antennas,
        dimensions: (height, width),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let board = parse_input(input);
    Some(count_antinodes(&board, false))
}

pub fn part_two(input: &str) -> Option<u32> {
    let board = parse_input(input);
    Some(count_antinodes(&board, true))
}

fn count_antinodes(board: &Board, part_two: bool) -> u32 {
    let mut antinodes: HashSet<(u32, u32)> = HashSet::new();

    for (_, antennas) in &board.antennas {
        for ant1 in antennas {
            if part_two {
                antinodes.insert(*ant1);
            }

            for ant2 in antennas {
                if ant1 != ant2 {
                    if part_two {
                        let mut factor = 1;
                        while let Some(antnd_loc) =
                            antinode_location(board, ant1, ant2, Some(factor))
                        {
                            antinodes.insert(antnd_loc);
                            factor += 1;
                        }
                    } else if let Some(antnd_loc) = antinode_location(board, ant1, ant2, None) {
                        antinodes.insert(antnd_loc);
                    }
                }
            }
        }
    }

    antinodes.len() as u32
}

fn antinode_location(
    board: &Board,
    p1: &(u32, u32),
    p2: &(u32, u32),
    factor: Option<i32>,
) -> Option<(u32, u32)> {
    let p1 = (p1.0 as i32, p1.1 as i32);
    let p2 = (p2.0 as i32, p2.1 as i32);

    let d = if let Some(f) = factor {
        point_mul(point_diff(p2, p1), f)
    } else {
        point_diff(p2, p1)
    };
    let q = point_diff(p1, d);
    let (board_x, board_y) = board.dimensions;

    if within_board(board_x, q.0) && within_board(board_y, q.1) {
        return Some((q.0 as u32, q.1 as u32));
    }

    None
}

fn point_mul(p: (i32, i32), f: i32) -> (i32, i32) {
    (p.0 * f, p.1 * f)
}

fn point_diff(p1: (i32, i32), p2: (i32, i32)) -> (i32, i32) {
    (p1.0 - p2.0, p1.1 - p2.1)
}

fn within_board(board_sz: usize, x: i32) -> bool {
    x >= 0 && x < board_sz as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
