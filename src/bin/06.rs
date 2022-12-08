use std::collections::HashSet;
pub fn part_one(input: &str) -> Option<u32> {
    let charstream = input.chars().collect::<Vec<char>>();
    let window_size = 4;
    let windows = charstream.windows(window_size);
    for (i, window) in windows.enumerate() {
        let mut set = HashSet::new();
        for char in window {
            set.insert(char);
        }
        if set.len() == window_size {
            return Some(i as u32 + window_size as u32);
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let charstream = input.chars().collect::<Vec<char>>();
    let window_size = 14;
    let windows = charstream.windows(window_size);
    for (i, window) in windows.enumerate() {
        let mut set = HashSet::new();
        for char in window {
            set.insert(char);
        }
        if set.len() == window_size {
            return Some(i as u32 + window_size as u32);
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
