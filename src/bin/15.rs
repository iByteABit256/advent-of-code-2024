use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(15);

#[derive(Debug, Clone)]
struct Warehouse {
    board: Vec<Vec<char>>,
    player: (usize, usize),
    movements: VecDeque<(i32, i32)>,
}

fn parse_input(input: &str, part_two: bool) -> Warehouse {
    let mut input = input.to_string();

    if part_two {
        input = input.replace("#", "##");
        input = input.replace("O", "[]");
        input = input.replace(".", "..");
        input = input.replace("@", "@.");
    }

    let mut lines = input.lines();

    let board: Vec<Vec<char>> = lines
        .by_ref()
        .take_while(|line| !line.trim().is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let player = board
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == '@').map(|x| (y, x)))
        .expect("Player position '@' not found in the board");

    let movement_map: HashMap<char, (i32, i32)> =
        HashMap::from([('<', (0, -1)), ('>', (0, 1)), ('v', (1, 0)), ('^', (-1, 0))]);

    let movements: VecDeque<(i32, i32)> = lines
        .flat_map(|line| line.chars())
        .filter_map(|c| movement_map.get(&c).copied())
        .collect();

    Warehouse {
        board,
        player,
        movements,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let warehouse = parse_input(input, false);
    Some(calculate(&make_moves(&warehouse)))
}

pub fn part_two(input: &str) -> Option<u32> {
    // let warehouse = parse_input(input, true);
    // Some(calculate(&make_moves(&warehouse)))
    None
}

fn calculate(warehouse: &Warehouse) -> u32 {
    warehouse
        .board
        .iter()
        .enumerate()
        .map(|(idx_x, row)| {
            row.iter()
                .enumerate()
                .map(|(idx_y, &c)| {
                    if c == 'O' {
                        100 * idx_x as u32 + idx_y as u32
                    } else {
                        0
                    }
                })
                .sum::<u32>()
        })
        .sum()
}

fn make_moves(warehouse: &Warehouse) -> Warehouse {
    let mut warehouse = warehouse.clone();

    while let Some(next_move) = warehouse.movements.pop_front() {
        // print_board(&warehouse);
        // println!("Current move: {:#?}", next_move);
        let (x, y) = (warehouse.player.0 as i32, warehouse.player.1 as i32);
        let new_pos = add((x, y), next_move);
        let new_pos_char = warehouse.board[new_pos.0 as usize][new_pos.1 as usize];

        match new_pos_char {
            '.' => {
                swap_pos(&mut warehouse, (x, y), new_pos);
                warehouse.player = (new_pos.0 as usize, new_pos.1 as usize);
            }
            'O' | '[' | ']' => {
                if let Some(warehouse_updated) = move_obstacles(&warehouse, new_pos, next_move) {
                    warehouse = warehouse_updated;
                }
            }
            _ => continue,
        }
    }

    warehouse
}

fn move_obstacles(warehouse: &Warehouse, obj: (i32, i32), dir: (i32, i32)) -> Option<Warehouse> {
    let mut warehouse = warehouse.clone();

    let mut curr_loc = add(obj, dir);
    let mut curr_char = warehouse.board[curr_loc.0 as usize][curr_loc.1 as usize];
    let mut prev_char = curr_char;

    let part_two = curr_char == '[' || curr_char == ']';

    while curr_char == 'O' || curr_char == '[' || curr_char == ']' {
        prev_char = curr_char;
        curr_loc = add(curr_loc, dir);
        curr_char = warehouse.board[curr_loc.0 as usize][curr_loc.1 as usize];
    }

    if curr_char == '#' {
        return None;
    } else {
        if part_two {
            // TODO :')

            let matching: char = if prev_char == '[' { ']' } else { '[' };
            let matching_loc = add(curr_loc, if prev_char == '[' { (0, 1) } else { (0, -1) });

            println!("matching: {}, matching loc: {:#?}", matching, matching_loc);

            warehouse.board[curr_loc.0 as usize][curr_loc.1 as usize] = prev_char;
            warehouse.board[matching_loc.0 as usize][matching_loc.1 as usize] = matching;
            warehouse.board[obj.0 as usize][obj.1 as usize] = '@';
            warehouse.board[warehouse.player.0 as usize][warehouse.player.1 as usize] = '.';
            warehouse.player = (obj.0 as usize, obj.1 as usize);

            print_board(&warehouse);

            let opp_dir = if dir.0 == 0 {
                (0, dir.1 * -2)
            } else {
                (dir.0 * -1, 0)
            };
            curr_loc = add(curr_loc, opp_dir);
            curr_char = warehouse.board[curr_loc.0 as usize][curr_loc.1 as usize];
            while curr_char == '[' || curr_char == ']' {
                let matching: char = if prev_char == '[' { ']' } else { '[' };
                let matching_loc = add(curr_loc, if prev_char == '[' { (0, 1) } else { (0, -1) });

                println!("curr character: {}, curr loc: {:#?}", prev_char, curr_loc);

                warehouse.board[curr_loc.0 as usize][curr_loc.1 as usize] = prev_char;
                warehouse.board[matching_loc.0 as usize][matching_loc.1 as usize] = matching;

                prev_char = curr_char;

                curr_loc = add(curr_loc, opp_dir);
                curr_char = warehouse.board[curr_loc.0 as usize][curr_loc.1 as usize];

                print_board(&warehouse);
            }
        } else {
            warehouse.board[curr_loc.0 as usize][curr_loc.1 as usize] = 'O';
            warehouse.board[obj.0 as usize][obj.1 as usize] = '@';
            warehouse.board[warehouse.player.0 as usize][warehouse.player.1 as usize] = '.';
            warehouse.player = (obj.0 as usize, obj.1 as usize);
        }
    }

    Some(warehouse)
}

fn swap_pos(warehouse: &mut Warehouse, a: (i32, i32), b: (i32, i32)) {
    let a = (a.0 as usize, a.1 as usize);
    let b = (b.0 as usize, b.1 as usize);
    let a_char = warehouse.board[a.0][a.1];
    let b_char = warehouse.board[b.0][b.1];

    warehouse.board[a.0][a.1] = b_char;
    warehouse.board[b.0][b.1] = a_char;
}

fn add(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
}

fn print_board(warehouse: &Warehouse) {
    warehouse.board.iter().for_each(|row| {
        row.iter().for_each(|&c| print!("{}", c));
        println!();
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
