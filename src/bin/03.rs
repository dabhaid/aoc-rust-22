use std::collections::HashSet;
pub fn part_one(input: &str) -> Option<u32> {
    let packs: Vec<&str> = format_input(input).unwrap();
    let mut accum = 0;
    for pack in packs {
        assert!(pack.len() % 2 == 0);
        let (a, b) = pack.split_at(pack.len() / 2);
        let set_a: HashSet<char> = HashSet::from_iter(a.chars());
        let set_b = HashSet::from_iter(b.chars());
        let intersect: HashSet<&char> = set_a.intersection(&set_b).collect();

        if intersect.len() > 0 {
            for item in intersect {
                accum += item_value(&item);
            }
        }
    }
    return Some(accum);
}

pub fn item_value(input: &char) -> u32 {
    let val = *input as u32;
    if (97..123).contains(&val) {
        return val - 96;
    } else {
        return val - 38;
    };
}

pub fn part_two(input: &str) -> Option<u32> {
    let packs: Vec<&str> = format_input(input).unwrap();
    let mut accum = 0;
    let group_packs: Vec<&[_]> = packs.chunks(3).collect();
    for packs in group_packs {
        let packs0: HashSet<char> = HashSet::from_iter(packs.get(0)?.chars());
        let packs1: HashSet<char> = HashSet::from_iter(packs.get(1)?.chars());
        let packs2: HashSet<char> = HashSet::from_iter(packs.get(2)?.chars());
        let mut common: HashSet<char> = packs0.intersection(&packs1).map(|x| *x).collect();
        common = common.intersection(&packs2).map(|x| *x).collect();
        if common.len() > 0 {
            for item in common {
                accum += item_value(&item);
            }
        }
    }
    return Some(accum);
}

pub fn format_input(input: &str) -> Option<Vec<&str>> {
    let mut vecs: Vec<&str> = [].to_vec();
    for line in input.split('\n') {
        vecs.push(line);
    }
    return Some(vecs);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        // In Rust all strings are string literals - neat!
        let str_input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(part_one(&str_input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
