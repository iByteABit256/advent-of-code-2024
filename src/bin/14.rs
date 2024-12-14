use std::collections::HashMap;

advent_of_code::solution!(14);

#[derive(Debug)]
struct GameData {
    robots: Vec<Robot>,
    board_size: (i32, i32),
}

#[derive(Debug)]
struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
}

type Quadrant = ((i32, i32), (i32, i32));

fn parse_input(input: &str) -> GameData {
    let mut max_x = 0;
    let mut max_y = 0;

    let robots: Vec<Robot> = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let pos_part = parts[0].trim_start_matches("p=").split_once(',').unwrap();
            let vel_part = parts[1].trim_start_matches("v=").split_once(',').unwrap();

            let pos = (
                str::parse::<i32>(pos_part.0).unwrap(),
                str::parse::<i32>(pos_part.1).unwrap(),
            );
            let vel = (
                str::parse::<i32>(vel_part.0).unwrap(),
                str::parse::<i32>(vel_part.1).unwrap(),
            );

            max_x = max_x.max(pos.0);
            max_y = max_y.max(pos.1);

            Robot { pos, vel }
        })
        .collect();

    GameData {
        robots,
        board_size: (max_x + 1, max_y + 1),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut input = parse_input(input);
    for _ in 0..100 {
        tick(&mut input);
    }
    Some(safety_factor(&input))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut input = parse_input(input);
    let mut step = 0;
    loop {
        tick(&mut input);
        let safety_factor = safety_factor(&input);
        if safety_factor < 95000000 {
            println!("Step: {}, Safety factor = {}", step, safety_factor);
            print_robots(&input);
            return Some(step + 1);
        }
        step += 1;
    }
}

fn tick(game: &mut GameData) {
    let board_sz = game.board_size;

    for r in &mut game.robots {
        r.pos = add_with_clipping(r.pos, r.vel, board_sz);
    }
}

fn add_with_clipping(a: (i32, i32), b: (i32, i32), limits: (i32, i32)) -> (i32, i32) {
    let clip = |n: i32, sz: i32| (n + sz) % sz;

    (clip(a.0 + b.0, limits.0), clip(a.1 + b.1, limits.1))
}

fn safety_factor(game: &GameData) -> u32 {
    let quadrants = quadrants(game.board_size);
    let mut quad_count: HashMap<usize, u32> = HashMap::with_capacity(4);
    game.robots.iter().for_each(|r| {
        if let Some(quad) = within_quadrant(r.pos, &quadrants) {
            *quad_count.entry(quad).or_default() += 1
        }
    });

    quad_count.values().product()
}

fn within_quadrant(pos: (i32, i32), quadrants: &[Quadrant]) -> Option<usize> {
    for (idx, quad) in quadrants.iter().enumerate() {
        if pos.0 >= quad.0 .0 && pos.0 <= quad.1 .0 && pos.1 >= quad.0 .1 && pos.1 <= quad.1 .1 {
            return Some(idx);
        }
    }
    None
}

fn quadrants(board_sz: (i32, i32)) -> Vec<Quadrant> {
    let sx = board_sz.0;
    let sy = board_sz.1;
    let lx = sx / 2;
    let ly = sy / 2;

    vec![
        ((0, 0), (lx - 1, ly - 1)),
        ((sx - lx, 0), (sx - 1, ly - 1)),
        ((0, sy - ly), (lx - 1, sy - 1)),
        ((sx - lx, sy - ly), (sx - 1, sy - 1)),
    ]
}

fn print_robots(game: &GameData) {
    let robot_exists = |pos: (i32, i32)| game.robots.iter().filter(|&r| pos == r.pos).next();

    for y in 0..game.board_size.1 {
        for x in 0..game.board_size.0 {
            let robot_exists = robot_exists((x, y)).is_some();
            if robot_exists { print!("#") } else { print!(" ") }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(229868730));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7861));
    }
}
