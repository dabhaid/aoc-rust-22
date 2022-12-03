use array2d::Array2D;
use std::collections::HashMap;

pub fn get_scores_pt1() -> HashMap<&'static str, u32> {
    return HashMap::from([("X", 1), ("Y", 2), ("Z", 3), ("W", 6), ("D", 3), ("L", 0)]);
}

pub fn get_outcome_pt1() -> HashMap<&'static str, &'static str> {
    return HashMap::from([
        ("AX", "D"),
        ("AY", "W"),
        ("AZ", "L"),
        ("BX", "L"),
        ("BY", "D"),
        ("BZ", "W"),
        ("CX", "W"),
        ("CY", "L"),
        ("CZ", "D"),
    ]);
}

pub fn get_scores_pt2() -> HashMap<&'static str, u32> {
    return HashMap::from([("X", 0), ("Y", 3), ("Z", 6), ("A", 1), ("B", 2), ("C", 3)]);
}

// Return the needed move, rather than the WLD outcome
pub fn get_outcome_pt2() -> HashMap<&'static str, &'static str> {
    return HashMap::from([
        ("AX", "C"),
        ("AY", "A"),
        ("AZ", "B"),
        ("BX", "A"),
        ("BY", "B"),
        ("BZ", "C"),
        ("CX", "B"),
        ("CY", "C"),
        ("CZ", "A"),
    ]);
}

pub fn format_input(input: &str) -> Array2D<&str> {
    let mut vecs: Vec<Vec<&str>> = [].to_vec();
    for line in input.split('\n') {
        let strs: Vec<&str> = line.split_ascii_whitespace().collect();
        if !strs.is_empty() {
            vecs.push(strs);
        }
    }
    return Array2D::from_rows(&vecs);
}

pub fn part_one(input: &str) -> Option<u32> {
    return Some(score(
        &format_input(input),
        get_scores_pt1(),
        get_outcome_pt1(),
    ));
}

pub fn part_two(_input: &str) -> Option<u32> {
    return Some(score(
        &format_input(_input),
        get_scores_pt2(),
        get_outcome_pt2(),
    ));
}

pub fn score(
    input: &Array2D<&str>,
    scores: HashMap<&'static str, u32>,
    outcome: HashMap<&'static str, &'static str>,
) -> u32 {
    let mut _accum = 0;
    for row_iter in input.rows_iter() {
        let row: Vec<&&str> = row_iter.collect();
        let play = format!("{}{}", row[0], row[1]);
        let result = outcome.get(play.as_str()).unwrap();
        _accum += scores[result];
        _accum += scores[row[1]];
    }
    return _accum;
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let rows = vec![vec!["A", "X"], vec!["B", "Y"], vec!["C", "Z"]];
        let array = Array2D::from_rows(&rows);
        assert_eq!(score(&array, get_scores_pt1(), get_outcome_pt1()), 15);
    }

    #[test]
    fn test_example_pt2() {
        let rows = vec![vec!["A", "Y"], vec!["B", "X"], vec!["C", "Z"]];
        let array = Array2D::from_rows(&rows);
        assert_eq!(score(&array, get_scores_pt2(), get_outcome_pt2()), 12);
    }
}
