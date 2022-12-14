pub fn part_one(input: &str) -> Option<u32> {
    let mut monkeys = parse_input(input).unwrap();
    let mut inspected: Vec<u32> = vec![0; monkeys.len()];
    let len = monkeys.len();
    let mut items: Vec<Vec<u32>> = Vec::new();
    for monkey in &mut monkeys[..] {
        items.push(monkey.items.clone());
    }
    for _rounds in 0..20 {
        for i in 0..len {
            for j in 0..items[i].len() {
                inspected[i] += 1;
                let operations: Vec<&str> =
                    monkeys[i].operations.split_ascii_whitespace().collect();
                match operations[1] {
                    "*" => match operations[2].parse::<u32>() {
                        Ok(number) => items[i][j] *= number,
                        Err(E) => items[i][j] *= items[i][j],
                    },
                    "+" => match operations[2].parse::<u32>() {
                        Ok(number) => items[i][j] += number,
                        Err(E) => items[i][j] += items[i][j],
                    },
                    &_ => (),
                }
                match items[i][j] % monkeys[i].divis == 0 {
                    true => {
                        let item = items[i].remove(j);
                        items[monkeys[i].true_throw as usize].push(item);
                    }
                    false => {
                        let item = items[i].remove(j);
                        items[monkeys[i].false_throw as usize].push(item);
                    }
                }
            }
        }
    }
    inspected.sort();
    inspected.reverse();
    Some(inspected[0] * inspected[1])
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[derive(Clone)]
pub struct Monkey {
    items: Vec<u32>,
    operations: String,
    divis: u32,
    true_throw: u32,
    false_throw: u32,
}

trait Throw {
    fn throw(&mut self, object: u32, other: &mut Monkey);
}

impl Monkey {
    pub fn new() -> Monkey {
        Monkey {
            items: Vec::new(),
            operations: String::new(),
            divis: 0,
            true_throw: 0,
            false_throw: 0,
        }
    }
}

pub fn parse_input(input: &str) -> Option<Vec<Monkey>> {
    let mut lines = input.lines();
    let mut line = lines.next();
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut items: Vec<Vec<u32>> = Vec::new();
    let mut monkey = Monkey::new();
    let mut index = 0;
    while !line.is_none() {
        let l = line.unwrap();
        match l.starts_with("Monkey") {
            true => {
                monkey = Monkey::new();
                index = l
                    .rsplit_once(" ")
                    .unwrap()
                    .1
                    .split_once(":")
                    .unwrap()
                    .0
                    .parse()
                    .unwrap()
            }
            false => {
                if l == "" {
                    break;
                }
                let (left, right) = l.split_once(":").unwrap();
                match left {
                    "Starting items" => {
                        let str_items = right.split(" ");
                        for str_item in str_items {
                            items[index].push(str_item.parse().unwrap());
                        }
                    }
                    "Operation" => {
                        monkey.operations = String::from(right);
                    }
                    "Test" => {
                        monkey.divis = right.rsplit_once(" ").unwrap().1.parse::<u32>().unwrap()
                    }
                    "If true" => {
                        monkey.true_throw = right.rsplit_once(" ").unwrap().1.parse().unwrap()
                    }
                    "If false" => {
                        monkey.false_throw = right.rsplit_once(" ").unwrap().1.parse().unwrap()
                    }

                    _ => {}
                }
            }
        }
        monkeys.push(monkey.to_owned());
        line = lines.next();
    }
    Some(monkeys)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), None);
    }
}
