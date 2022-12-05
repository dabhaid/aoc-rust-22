pub fn part_one(input: &str) -> Option<u32> {
    let mut summed_sacks = sum_elves(format_input(input));
    summed_sacks.sort();
    return Some(summed_sacks[summed_sacks.len() - 1]);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut summed_sacks = sum_elves(format_input(input));
    summed_sacks.sort();
    return Some(summed_sacks.iter().rev().take(3).sum());
}

pub fn sum_elves(input: Vec<Vec<u32>>) -> Vec<u32> {
    let mut elfsum: Vec<u32> = Vec::from([]);
    for elf in input {
        elfsum.push(elf.iter().sum());
    }
    return elfsum;
}

pub fn format_input(input: &str) -> Vec<Vec<u32>> {
    let mut elves = Vec::from([]);
    let mut elf = Vec::from([]);
    for line in input.lines() {
        if line != "" {
            elf.push(line.parse().unwrap());
        } else {
            elves.push(elf);
            elf = Vec::from([]);
        }
    }
    return elves;
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
