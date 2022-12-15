pub fn part_one(input: &str) -> Option<u64> {
    let monkeys = parse_input(input).unwrap();
    sim_monkey(monkeys, 20, 3.0)
}

pub fn sim_monkey(mut monkeys: Vec<Monkey>, rounds: u32, divisor: f64) -> Option<u64> {
    let len = monkeys.len();
    let mut cap = 1;
    for x in 0..monkeys.len() {
        cap = cap * monkeys[x].divis;
    }
    for _rounds in 0..rounds {
        for i in 0..len {
            for _j in 0..monkeys[i].items.len() {
                monkeys[i].inspections += 1;
                let mut item = monkeys[i].items.pop().unwrap();
                let operations: Vec<&str> =
                    monkeys[i].operations.split_ascii_whitespace().collect();
                match operations[3] {
                    "*" => match operations[4].parse::<u64>() {
                        Ok(number) => item *= number,
                        Err(_e) => item *= item,
                    },
                    "+" => match operations[4].parse::<u64>() {
                        Ok(number) => item += number,
                        Err(_e) => item += item,
                    },
                    &_ => (),
                }
                let num: f64 = item as f64 / divisor;
                item = num.floor() as u64;
                item = item % cap;
                match item % monkeys[i].divis == 0 {
                    true => {
                        let next_monkey = monkeys[i].true_throw as usize;
                        monkeys[next_monkey].items.insert(0, item.to_owned());
                    }
                    false => {
                        let next_monkey = monkeys[i].false_throw as usize;
                        monkeys[next_monkey].items.insert(0, item.to_owned());
                    }
                }
            }
        }
    }
    let mut inspected: Vec<u64> = vec![0; monkeys.len()];
    for i in 0..monkeys.len() {
        inspected.push(monkeys[i].inspections);
    }
    inspected.sort();
    inspected.reverse();
    Some(inspected[0] * inspected[1])
}

pub fn part_two(input: &str) -> Option<u64> {
    let monkeys = parse_input(input).unwrap();
    sim_monkey(monkeys, 10000, 1.0)
}

#[derive(Clone)]
pub struct Monkey {
    items: Vec<u64>,
    operations: String,
    divis: u64,
    true_throw: u32,
    false_throw: u32,
    inspections: u64,
}

impl Monkey {
    pub fn new() -> Monkey {
        Monkey {
            items: Vec::new(),
            operations: String::new(),
            divis: 0,
            true_throw: 0,
            false_throw: 0,
            inspections: 0,
        }
    }
}

pub fn parse_input(input: &str) -> Option<Vec<Monkey>> {
    let mut lines = input.lines();
    let mut line = lines.next();
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut monkey;
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
                    .unwrap();
                monkeys.push(monkey);
            }
            false => {
                if l == "" {
                    line = lines.next();
                    continue;
                }
                let mut monkey = &mut monkeys[index];
                let (left, right) = l.split_once(":").unwrap();
                match left.trim() {
                    "Starting items" => {
                        let str_items = right.split(", ");
                        for str_item in str_items {
                            monkey.items.insert(0, str_item.trim().parse().unwrap())
                        }
                    }
                    "Operation" => {
                        monkey.operations = String::from(right);
                    }
                    "Test" => {
                        monkey.divis = right.rsplit_once(" ").unwrap().1.parse::<u64>().unwrap()
                    }
                    "If true" => {
                        monkey.true_throw = right.rsplit_once(" ").unwrap().1.parse().unwrap()
                    }
                    "If false" => {
                        monkey.false_throw = right.rsplit_once(" ").unwrap().1.parse().unwrap()
                    }

                    _ => {}
                }
                monkeys[index] = monkey.to_owned();
            }
        }
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
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
