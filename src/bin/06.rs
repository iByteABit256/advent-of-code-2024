use std::collections::HashSet;

advent_of_code::solution!(6);

struct GameData {
    board: Vec<Vec<char>>,
    player: Player,
    distinct_positions: u32,
    finished: bool,
    corner_history: HashSet<Corner>,
    caught_in_loop: bool,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Player {
    position: (usize, usize),
    direction: char,
}

type Corner = Player;

fn parse_input(input: &str) -> GameData {
    let mut player_position = None;
    let mut player_direction = None;

    let board: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(row_index, line)| {
            line.chars()
                .enumerate()
                .map(|(col_index, c)| {
                    if c != '.' && c != '#' {
                        player_position = Some((row_index, col_index));
                        player_direction = Some(c);
                    }
                    c
                })
                .collect()
        })
        .collect();

    let (position, direction) = match (player_position, player_direction) {
        (Some(pos), Some(dir)) => (pos, dir),
        _ => panic!("Player position or direction not found in the input!"),
    };

    GameData {
        board,
        player: Player {
            position,
            direction,
        },
        distinct_positions: 0,
        finished: false,
        corner_history: HashSet::new(),
        caught_in_loop: false,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut game = parse_input(input);

    while !game.finished {
        print_board(&game.board);
        game = walk(game);
    }

    Some(game.distinct_positions)
}

pub fn part_two(input: &str) -> Option<u32> {
    let game = parse_input(input);
    let mut dead_loop_count = 0;

    for i in 0..game.board.len() {
        for j in 0..game.board[i].len() {
            let mut board_copy = game.board.clone();

            if board_copy[i][j] == '.' {
                board_copy[i][j] = '#'
            }

            let mut changed_game = GameData {
                board: board_copy.clone(),
                player: game.player.clone(),
                distinct_positions: 0,
                finished: false,
                corner_history: HashSet::new(),
                caught_in_loop: false,
            };

            while !changed_game.finished {
                print_board(&board_copy);
                changed_game = walk(changed_game);
            }

            if changed_game.caught_in_loop {
                dead_loop_count += 1;
            }
        }
    }

    Some(dead_loop_count)
}

fn walk(mut game: GameData) -> GameData {
    let mut board = game.board;
    let mut player = game.player;
    let (x, y) = player.position;

    // Mark current position
    board[x][y] = 'X';

    let (new_player, caught_in_loop) = check_object(&board, &player, &mut game.corner_history);

    if !caught_in_loop {
        if let Some(new_player) = new_player {
            let (new_x, new_y) = new_player.position;
            player = new_player;

            if board[new_x][new_y] != 'X' {
                game.distinct_positions += 1;
            }
            board[new_x][new_y] = player.direction;
        } else {
            game.finished = true;
            game.distinct_positions += 1;
        }
    } else {
        game.finished = true;
    }

    GameData {
        board,
        player,
        distinct_positions: game.distinct_positions,
        finished: game.finished,
        corner_history: game.corner_history,
        caught_in_loop,
    }
}

fn check_object(
    board: &[Vec<char>],
    player: &Player,
    corner_history: &mut HashSet<Corner>,
) -> (Option<Player>, bool) {
    let mut direction = player.direction;

    while let Some((new_x, new_y)) = new_pos(board, player.position, map_direction(direction)) {
        if board[new_x][new_y] != '#' {
            return (
                Some(Player {
                    position: (new_x, new_y),
                    direction,
                }),
                false,
            );
        }

        let corner = Corner {
            position: (new_x, new_y),
            direction,
        };

        if corner_history.contains(&corner) {
            return (None, true);
        }

        corner_history.insert(corner);
        direction = rotate(direction);
    }

    (None, false)
}

fn new_pos(board: &[Vec<char>], pos: (usize, usize), dir: (i32, i32)) -> Option<(usize, usize)> {
    let new_x = pos.0 as i32 + dir.0;
    let new_y = pos.1 as i32 + dir.1;

    if !within_bounds(board, (new_x, new_y)) {
        None
    } else {
        Some((new_x as usize, new_y as usize))
    }
}

fn within_bounds(board: &[Vec<char>], pos: (i32, i32)) -> bool {
    let xsz = board.len() as i32;
    let ysz = board[0].len() as i32;
    let (x, y) = pos;

    0 <= x && x < xsz && 0 <= y && y < ysz
}

fn map_direction(player_direction: char) -> (i32, i32) {
    match player_direction {
        '^' => (-1, 0),
        '>' => (0, 1),
        'v' => (1, 0),
        _ => (0, -1),
    }
}

fn rotate(player_direction: char) -> char {
    match player_direction {
        '^' => '>',
        '>' => 'v',
        'v' => '<',
        _ => '^',
    }
}

fn print_board(board: &[Vec<char>]) {
    for row in board {
        println!();
        for char in row {
            print!("{}", char);
        }
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
