use std::cmp;

pub fn part_one(input: &str) -> Option<u32> {
    let mut accum = 0;
    let tuple_pairs = format_input(input);
    for tuple_pair in tuple_pairs {
        let ((a, b), (x, y)) = tuple_pair;
        if (a >= x && b <= y) || (x >= a && y <= b) {
            accum += 1;
        }
    }
    return Some(accum);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut accum = 0;
    let tuple_pairs = format_input(input);
    for tuple_pair in tuple_pairs {
        let ((a, b), (x, y)) = tuple_pair;
        if cmp::max(a, x) <= cmp::min(b, y) {
            accum += 1;
        }
    }
    return Some(accum);
}

pub fn format_input(input: &str) -> Vec<((u32, u32), (u32, u32))> {
    let mut results = Vec::from([]);
    for assign_pair in input.split('\n') {
        let pairs = assign_pair.split(",").collect::<Vec<&str>>();
        let first: Vec<&str> = pairs[0].split("-").collect();
        let second: Vec<&str> = pairs[1].split("-").collect();
        results.push((
            (
                first[0].parse::<u32>().unwrap(),
                first[1].parse::<u32>().unwrap(),
            ),
            (
                second[0].parse::<u32>().unwrap(),
                second[1].parse::<u32>().unwrap(),
            ),
        ));
    }
    return results;
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input: String = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8".to_string();
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input: String = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8".to_string();
        assert_eq!(part_two(&input), Some(4));
    }
}
