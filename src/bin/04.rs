advent_of_code::solution!(4);

pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let board = parse_input(input);
    Some(xmas_count(&board))
}

pub fn part_two(input: &str) -> Option<u32> {
    let board = parse_input(input);
    Some(x_mas_count(&board))
}

// ===========================================
//                  Part 1
// ===========================================

fn xmas_count(board: &Vec<Vec<char>>) -> u32 {
    board
        .iter()
        .enumerate()
        .map(|(x, row)| {
            row.iter()
                .enumerate()
                .filter(|(y, _)| board[x][*y] == 'X')
                .map(|(y, _)| local_xmas_count(board, (x, y), None, 0, 'X'))
                .sum::<u32>()
        })
        .sum()
}

fn local_xmas_count(
    board: &Vec<Vec<char>>,
    location: (usize, usize),
    direction: Option<(i32, i32)>,
    count: u32,
    context: char,
) -> u32 {
    let expected_next = xmas_next_letter(context);

    match expected_next {
        Some(ex_next) => {
            let mut nb_count = 0;

            for (nb, nb_dir) in neighbours(board, location, direction) {
                let (x, y) = nb;
                let nb_char = board[x][y];

                if nb_char == ex_next {
                    nb_count += local_xmas_count(board, (x, y), Some(nb_dir), count, ex_next);
                }
            }
            nb_count
        }
        None => count + 1,
    }
}

type Neighbor = ((usize, usize), (i32, i32));

fn neighbours(
    board: &[Vec<char>],
    location: (usize, usize),
    direction: Option<(i32, i32)>,
) -> Vec<Neighbor> {
    let x = location.0 as i32;
    let y = location.1 as i32;

    let max_x = board.len() as i32;
    let max_y = board[0].len() as i32;

    let mut nb: Vec<Neighbor> = Vec::new();

    let populate_nb = |nb: &mut Vec<Neighbor>, (x, y): (i32, i32), (off_x, off_y): (i32, i32)| {
        let new_x = x + off_x;
        let new_y = y + off_y;

        if !(new_x < 0
            || new_y < 0
            || new_x >= max_x
            || new_y >= max_y
            || (new_x == x && new_y == y))
        {
            nb.push(((new_x as usize, new_y as usize), (off_x, off_y)));
        }
    };

    match direction {
        Some(dir) => populate_nb(&mut nb, (x, y), dir),
        None => {
            for offset_x in -1..=1 {
                for offset_y in -1..=1 {
                    populate_nb(&mut nb, (x, y), (offset_x, offset_y));
                }
            }
        }
    }

    nb
}

fn xmas_next_letter(c: char) -> Option<char> {
    match c {
        'X' => Some('M'),
        'M' => Some('A'),
        'A' => Some('S'),
        _ => None,
    }
}

// ===========================================
//                  Part 2
// ===========================================

fn x_mas_count(board: &[Vec<char>]) -> u32 {
    board
        .iter()
        .enumerate()
        .map(|(x, row)| {
            row.iter()
                .enumerate()
                .filter(|(y, _)| board[x][*y] == 'A')
                .filter(|(y, _)| local_is_x_mas(board, (x, *y)))
                .count()
        })
        .sum::<usize>() as u32
}

fn local_is_x_mas(board: &[Vec<char>], location: (usize, usize)) -> bool {
    let x = location.0 as i32;
    let y = location.1 as i32;

    let max_x = board.len() as i32;
    let max_y = board[0].len() as i32;

    let mut mas_count = 0;

    // Checks two left corners
    for left_corner in [-1, 1] {
        let offset_x = -1;
        let offset_y = left_corner;

        let new_x = x + offset_x;
        let new_y = y + offset_y;

        // Right corners
        let nx_opp = x + 1;
        let ny_opp = y - offset_y;

        let location_valid = |nx, ny| !(nx < 0 || ny < 0 || nx >= max_x || ny >= max_y);

        let is_mas = |a: char, b: char| a == 'M' && b == 'S' || a == 'S' && b == 'M';

        if location_valid(new_x, new_y) && location_valid(nx_opp, ny_opp) {
            if is_mas(
                board[new_x as usize][new_y as usize],
                board[nx_opp as usize][ny_opp as usize],
            ) {
                mas_count += 1;
            }

            if mas_count == 2 {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
