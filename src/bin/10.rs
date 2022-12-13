use std::fmt;

pub fn part_one(input: &str) -> Option<i32> {
    let mut instructions = parse_input(input);
    instructions.reverse();
    let mut x: i32 = 1;
    let mut busy = 0;
    let mut result: i32 = 0;
    let mut clock = 0;
    let mut value_checks: i32 = 0;
    while !instructions.is_empty() {
        if busy == 0 {
            x += result;
            let (instruction, arg) = instructions.pop().unwrap();
            match instruction {
                Instruction::AddX => {
                    busy += 2;
                    result = arg.unwrap();
                }
                Instruction::NoOp => {
                    busy += 1;
                    result = 0;
                }
            }
        }
        busy -= 1;
        clock += 1;
        if clock == 20 || (clock - 20) % 40 == 0 {
            print!("Clock tick{}: {}\n", clock, clock * x);
            value_checks += clock * x;
        }
    }
    Some(value_checks)
}

#[derive(Debug)]
pub struct Line {
    line: String,
}

impl PartialEq for Line {
    fn eq(&self, other: &Line) -> bool {
        return self.line == other.line;
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.line)
    }
}

#[derive(Debug)]
pub struct Lines(pub Vec<Line>);

impl PartialEq for Lines {
    fn eq(&self, other: &Lines) -> bool {
        let matching = self.0.iter().zip(&other.0).filter(|&(a, b)| a != b).count();
        return matching == 0;
    }
}

impl Eq for Lines {
    fn assert_receiver_is_total_eq(&self) {}
}

impl fmt::Display for Lines {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, line| {
            result.and_then(|_| writeln!(f, "{}", line))
        })
    }
}

pub fn part_two(input: &str) -> Option<Lines> {
    let mut instructions = parse_input(input);
    instructions.reverse();
    let mut x: i32 = 1;
    let mut busy = 0;
    let mut result: i32 = 0;
    let mut clock = 0;
    let mut lines: Lines = Lines { 0: Vec::new() };
    let mut line: Line = Line {
        line: "".to_string(),
    };
    while !instructions.is_empty() {
        if busy == 0 {
            x += result;
            let (instruction, arg) = instructions.pop().unwrap();
            match instruction {
                Instruction::AddX => {
                    busy += 2;
                    result = arg.unwrap();
                }
                Instruction::NoOp => {
                    busy += 1;
                    result = 0;
                }
            }
        }
        busy -= 1;
        if clock > 0 && clock % 40 == 0 {
            lines.0.push(Line {
                line: line.to_string(),
            });

            line.line = String::new();
        }
        if (x - clock % 40).abs() <= 1 {
            line.line.push('#')
        } else {
            line.line.push('.');
        }
        clock += 1;
    }
    lines.0.push(line);
    //Some(lines)
    Some(lines)
}

pub enum Instruction {
    AddX = 2,
    NoOp = 1,
}

impl Instruction {
    pub fn from(s: &str) -> Instruction {
        match s {
            "addx" => Instruction::AddX,
            "noop" => Instruction::NoOp,
            _ => panic!("That shouldn't happen!"),
        }
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

pub fn parse_input(input: &str) -> Vec<(Instruction, Option<i32>)> {
    let mut instructions: Vec<(Instruction, Option<i32>)> = Vec::new();
    for line in input.lines() {
        let split_result = line.split_once(" ");
        match split_result {
            None => instructions.push((Instruction::from(line), None)),
            Some((instruction, arg)) => {
                instructions.push((Instruction::from(instruction), arg.parse().ok()))
            }
        }
    }
    instructions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        let str_lines = [
            "##..##..##..##..##..##..##..##..##..##..",
            "###...###...###...###...###...###...###.",
            "####....####....####....####....####....",
            "#####.....#####.....#####.....#####.....",
            "######......######......######......####",
            "#######.......#######.......#######.....",
        ];
        let mut lines: Lines = Lines { 0: Vec::new() };
        for line in str_lines {
            lines.0.push(Line {
                line: line.to_string(),
            });
        }
        assert_eq!(part_two(&input).unwrap(), lines);
    }
}
