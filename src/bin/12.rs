use std::{cmp::max, collections::HashMap};

use array2d::Array2D;

pub fn part_one(input: &str) -> Option<u32> {
    let array = parse_input(input);
    let end_array = find_in_array('E', &array);
    let end = *end_array.first().unwrap();

    // strategy
    // try to move towards E
    //search for the nearest next step up

    let binding = find_in_array('S', &array);
    let mut location = *binding.first().unwrap();

    let mut counter = 0;
    let blobs = blob_detection(&array);
    while location != end {
        location = decide_next_move(location, &array, &blobs);
        print!(
            "At {:?}:{:#?}\n",
            location,
            array.get(location.0, location.1).unwrap()
        );
        counter += 1;
        if counter == 1000 {
            break;
        }
    }
    Some(counter)
}

// prefer to go towards the end

pub fn decide_next_move(
    location: (usize, usize),
    array: &Array2D<char>,
    blobs: &HashMap<char, Vec<Vec<(usize, usize)>>>,
) -> (usize, usize) {
    let current_blob = find_in_array(*array.get(location.0, location.1).unwrap(), &array);
    let next_blob = find_in_array(
        next_level(*array.get(location.0, location.1).unwrap()),
        &array,
    );

    let target = find_connection(&current_blob, &next_blob).unwrap();
    print!(
        "target: {:#?}, {:#?}\n",
        target,
        array.get(target.0, target.1).unwrap()
    );
    let mut moves = get_next_moves(&location, &(array.column_len(), array.row_len()));
    let mut bad_moves: Vec<usize> = Vec::new();
    for i in 0..moves.len() {
        if !valid_move(
            *array.get(location.0, location.1).unwrap(),
            *array.get(moves[i].0, moves[i].1).unwrap(),
        ) {
            bad_moves.push(i);
        }
    }
    bad_moves.reverse();
    for i in bad_moves {
        moves.remove(i);
    }
    let mut distance: Vec<i32> = vec![0; moves.len()];
    for i in 0..moves.len() {
        distance[i] = get_distance(&moves[i], &target) as i32;
    }
    let mut min = max(array.column_len(), array.row_len());
    let mut index = 0;
    for i in 0..distance.len() {
        if distance[i] < min as i32 {
            min = distance[i] as usize;
            index = i;
        }
    }
    (moves[index].0, moves[index].1)
}

pub fn valid_move(current: char, next: char) -> bool {
    if current == next {
        return true;
    }
    if next_level(current) == next {
        return true;
    }
    if current as u32 > next as u32 {
        return true;
    }
    false
}

pub fn blob_detection(array: &Array2D<char>) -> HashMap<char, Vec<Vec<(usize, usize)>>> {
    let mut blobs = HashMap::new();
    for i in 0..array.column_len() {
        for j in 0..array.row_len() {
            let char = array.get(i, j).unwrap();
            if blobs.get(char).is_none() {
                let mut char_blobs = Vec::new();
                let mut char_blob = Vec::new();
                char_blob.push((i, j));
                char_blobs.push(char_blob);
                blobs.insert(*char, char_blobs);
            } else {
                let char_blobs = blobs.get_mut(char).unwrap();
                for k in 0..char_blobs.len() {
                    let char_blob = char_blobs.get_mut(k).unwrap();
                    if is_connected((i, j), &char_blob) {
                        char_blob.push((i, j));
                        continue;
                    } else {
                        char_blobs.push(Vec::from([(i, j)]));
                    }
                }
            }
        }
    }
    blobs
}

pub fn get_next_moves(current: &(usize, usize), bounds: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut moves: Vec<(usize, usize)> = Vec::new();
    if current.0 as i32 + 1 < bounds.0 as i32 {
        moves.push((current.0 + 1, current.1));
    }
    if current.0 as i32 - 1 >= 0 as i32 {
        moves.push((current.0 - 1, current.1));
    }
    if current.1 as i32 + 1 < bounds.1 as i32 {
        moves.push((current.0, current.1 + 1));
    }
    if current.1 as i32 - 1 >= 0 as i32 {
        moves.push((current.0, current.1 - 1));
    }
    moves
}

pub fn find_connection(
    first: &Vec<(usize, usize)>,
    second: &Vec<(usize, usize)>,
) -> Option<(usize, usize)> {
    for j in second {
        if is_connected(*j, first) {
            return Some(*j);
        }
    }
    None
}

pub fn is_connected(point: (usize, usize), blob: &Vec<(usize, usize)>) -> bool {
    for bloblet in blob {
        if get_distance(&point, bloblet) == 1 {
            return true;
        }
    }
    false
}

pub fn get_vertical_distance(
    one: &(usize, usize),
    two: &(usize, usize),
    array: &Array2D<char>,
) -> i32 {
    let mut start = *array.get(one.0, one.1).unwrap();
    let mut end = *array.get(two.0, two.1).unwrap();

    if start == 'S' {
        start = 'a';
    }
    if end == 'S' {
        end = 'a';
    }
    if start == 'E' {
        start = 'z';
    }
    if end == 'E' {
        end = 'z';
    }
    return (end as i32 - start as i32);
}

pub fn get_distance(one: &(usize, usize), two: &(usize, usize)) -> u32 {
    let x: i32 = two.0 as i32 - one.0 as i32;
    let y: i32 = two.1 as i32 - one.1 as i32;
    return (x.abs() + y.abs()) as u32;
}

pub fn next_level(current: char) -> char {
    match current {
        'S' => 'a',
        'z' => 'E',
        _ => char::from_u32(current as u32 + 1).unwrap(),
    }
}

pub fn find_in_array(letter: char, array: &Array2D<char>) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    for i in 0..array.column_len() {
        for j in 0..array.row_len() {
            if *array.get(i, j).unwrap() == letter {
                result.push((i, j));
            }
        }
    }
    result
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

pub fn parse_input(input: &str) -> Array2D<char> {
    let mut rows: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        rows.push(line.chars().collect());
    }

    let array = Array2D::from_rows(&rows);
    array
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), None);
    }
}
