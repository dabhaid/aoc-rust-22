use std::convert::TryFrom;

pub fn part_one(input: &str) -> Option<String> {
    let (moves, mut crates) = parse_input(input);
    for [count, from, to] in moves {
        for _i in 0..count {
            let c = crates[from - 1].pop().unwrap();
            crates[to - 1].push(c);
        }
    }
    let mut result = String::new();
    for i in crates {
        result.push(*i.last().unwrap());
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let (moves, mut crates) = parse_input(input);
    for [count, from, to] in moves {
        let mut take: Vec<char> = Vec::new();
        for _i in 0..count {
            take.push(crates[from - 1].pop().unwrap());
        }
        take.reverse();
        crates[to - 1].append(&mut take);
    }
    let mut result = String::new();
    for i in crates {
        result.push(*i.last().unwrap());
    }
    Some(result)
}

pub fn parse_input(input: &str) -> (Vec<[usize; 3]>, Vec<Vec<char>>) {
    // There are two inputs, the moves and the crates
    // We know how many crates there are when we the to that line
    // So lets read backwards

    let mut moves = Vec::new();
    let mut lines = input.lines().rev();
    let mut line = lines.next();
    while !line.unwrap().is_empty() {
        //move 1 from 2 to 1
        //move 14 from 2 to 1
        let mut offset = 0;
        let count;
        if line.unwrap().len() > 18 {
            let count_string = &line.unwrap()[5..7];
            count = usize::try_from(count_string.parse::<u32>().unwrap()).unwrap();
            offset = 1;
        } else {
            count = usize::try_from(line.unwrap().chars().nth(5).unwrap().to_digit(10).unwrap())
                .unwrap();
        }
        moves.push([
            count,
            usize::try_from(
                line.unwrap()
                    .chars()
                    .nth(12 + offset)
                    .unwrap()
                    .to_digit(10)
                    .unwrap(),
            )
            .unwrap(),
            usize::try_from(
                line.unwrap()
                    .chars()
                    .nth(17 + offset)
                    .unwrap()
                    .to_digit(10)
                    .unwrap(),
            )
            .unwrap(),
        ]);

        line = lines.next();
    }
    moves.reverse();
    let crates_config = lines.next().unwrap().split_ascii_whitespace();
    let num_crates: u32 = crates_config.last().unwrap().parse::<u32>().unwrap();
    let mut crates: Vec<Vec<char>> = vec![Vec::new(); num_crates.try_into().unwrap()];
    line = lines.next();
    while !line.is_none() {
        for i in 0..num_crates {
            let element;
            let index: usize = i.try_into().unwrap();
            if i == 0 {
                element = line.unwrap().chars().nth(index + 1).unwrap();
            } else {
                element = line.unwrap().chars().nth(1 + (4 * index)).unwrap();
            }
            if !element.is_whitespace() {
                let cr: &mut Vec<char> = &mut crates[usize::try_from(i).unwrap()];
                cr.push(element);
            }
        }
        line = lines.next();
    }
    (moves, crates)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = advent_of_code::read_file("examples", 5);
        let moves = Vec::from([[1, 2, 1], [3, 1, 3], [2, 2, 1], [1, 1, 2]]);
        let crates = Vec::from([
            Vec::from(['Z', 'N']),
            Vec::from(['M', 'C', 'D']),
            Vec::from(['P']),
        ]);
        assert_eq!(parse_input(&input), (moves, crates));
    }
    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
