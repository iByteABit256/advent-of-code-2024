advent_of_code::solution!(9);

fn parse_input(input: &str, part_two: bool) -> Vec<(u32, u32)> {
    let mut transformed_filemap: Vec<(u32, u32)> = Vec::new();
    let file_map: Vec<u32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).expect("Input char was not a digit"))
        .collect();

    let mut file_id = 0;
    for (idx, &val) in file_map.iter().enumerate() {
        if idx % 2 == 0 {
            if part_two {
                transformed_filemap.push((file_id + 1, val));
            } else {
                for _ in 0..val {
                    transformed_filemap.push((file_id + 1, 1));
                }
            }
            file_id += 1;
        } else {
            transformed_filemap.push((0, val));
        }
    }

    transformed_filemap
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input, false);
    Some(checksum(optimize_filesystem(input), false))
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input, true);
    Some(checksum(optimize_filesystem(input), true))
}

fn checksum(filemap: Vec<(u32, u32)>, part_two: bool) -> u64 {
    if part_two {
        let mut cur_idx = 0;

        filemap
            .iter()
            .map(|(file_id, size)| {
                if *file_id == 0 {
                    cur_idx += *size;
                    0
                } else {
                    let res = (0..*size)
                        .map(|offset| ((*file_id - 1) * (cur_idx + offset)) as u64)
                        .sum::<u64>();
                    cur_idx += *size;
                    res
                }
            })
            .sum()
    } else {
        filemap
            .iter()
            .filter(|(file_id, _)| *file_id != 0)
            .enumerate()
            .map(|(idx, (file_id, _))| (idx as u32 * (file_id - 1)) as u64)
            .sum()
    }
}

fn optimize_filesystem(mut filemap: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    // Optimize the filemap by dual indexing
    for idx_r in (0..filemap.len()).rev() {
        for idx_l in 0..idx_r {
            let free = filemap[idx_l];
            let used = filemap[idx_r];

            // Left is free space and can contain right
            if free.0 == 0 && used.0 != 0 && free.1 >= used.1 {
                filemap[idx_l] = (0, free.1 - used.1);
                filemap[idx_r] = (0, used.1);
                filemap.insert(idx_l, (used.0, used.1));
            }
        }
    }

    filemap.retain(|&f| f != (0, 0));

    filemap
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
