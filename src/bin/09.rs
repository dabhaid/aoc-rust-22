use std::collections::HashSet;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    pub fn new() -> Coordinate {
        return Coordinate { x: 0, y: 0 };
    }
}

pub trait Distance {
    fn distance(&self, other: &Coordinate) -> Coordinate;
}

pub trait Move {
    fn advance(&mut self, other: &Direction);
}

impl Distance for Coordinate {
    fn distance(self: &Coordinate, other: &Coordinate) -> Coordinate {
        return Coordinate {
            x: other.x - self.x,
            y: other.y - self.y,
        };
    }
}

impl Move for Coordinate {
    fn advance(self: &mut Coordinate, direction: &Direction) {
        match direction {
            Direction::Down => self.y -= 1,
            Direction::Up => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut tail_moves: HashSet<Coordinate> = HashSet::new();
    let mut h = Coordinate::new();
    let mut t = Coordinate::new();
    let moves = parse_input(input);
    for m in moves {
        for _i in 0..m.1 {
            h.advance(&m.0);
            tail_move(&mut t, &h);
            tail_moves.insert(t);
        }
    }
    Some(tail_moves.len() as u32)
}

pub fn tail_move(t: &mut Coordinate, h: &Coordinate) {
    let distance = t.distance(h);
    // the diagonals
    if distance.x > 1 && distance.y >= 1 {
        t.x += 1;
        t.y += 1;
    } else if distance.x < -1 && distance.y >= 1 {
        t.x -= 1;
        t.y += 1;
    } else if distance.x > 1 && distance.y <= -1 {
        t.x += 1;
        t.y -= 1;
    } else if distance.x < -1 && distance.y <= -1 {
        t.x -= 1;
        t.y -= 1;
    } else if distance.x >= 1 && distance.y > 1 {
        t.x += 1;
        t.y += 1;
    } else if distance.x <= -1 && distance.y > 1 {
        t.x -= 1;
        t.y += 1;
    } else if distance.x >= 1 && distance.y < -1 {
        t.x += 1;
        t.y -= 1;
    } else if distance.x <= -1 && distance.y < -1 {
        t.x -= 1;
        t.y -= 1;
    } else if distance.x > 1 {
        t.x += 1;
    } else if distance.x < -1 {
        t.x -= 1;
    } else if distance.y > 1 {
        t.y += 1;
    } else if distance.y < -1 {
        t.y -= 1;
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut tail_moves: HashSet<Coordinate> = HashSet::new();
    let mut knots = vec![Coordinate::new(); 10];
    let moves = parse_input(input);
    for m in moves {
        for _i in 0..m.1 {
            &knots[0].advance(&m.0);
            for i in 1..10 {
                let relative_head = knots[i - 1].clone();
                tail_move(&mut knots[i], &relative_head);
            }
            tail_moves.insert(knots[9]);
        }
    }
    Some(tail_moves.len() as u32)
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn from_str(s: &str) -> Direction {
    match s {
        "L" => Direction::Left,
        "R" => Direction::Right,
        "D" => Direction::Down,
        "U" => Direction::Up,
        _ => panic!("that's not supposed to happen"),
    }
}

pub fn parse_input(input: &str) -> Vec<(Direction, u32)> {
    let mut moves: Vec<(Direction, u32)> = Vec::new();
    for line in input.lines() {
        let (direction, steps) = line.split_once(" ").unwrap();
        moves.push((from_str(direction), steps.parse().unwrap()));
    }
    moves
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), (Some(13)));
    }

    #[test]
    fn test_part_two() {
        let input = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";
        assert_eq!(part_two(&input), Some(36));
    }
}
