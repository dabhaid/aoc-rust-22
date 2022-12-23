use std::collections::{HashMap, HashSet, VecDeque};

use array2d::Array2D;

pub fn part_one(input: &str) -> Option<u32> {
    let array = parse_input(input);

    let path = depth_first_search(&array);
    print!("{:?}", path);
    Some(path.unwrap().len() as u32)
}

pub fn depth_first_search(array: &Array2D<char>) -> Option<Vec<(char, usize)>> {
    let blobs = get_letter_blobs(array);
    let mut visited: HashSet<(char, usize)> = HashSet::new();
    let mut history: Vec<(char, usize)> = Vec::new();
    let mut queue = VecDeque::new();
    let mut letters: Vec<char> = Vec::new();
    // a little cheating, the start is S,0 the end is E,0
    queue.push_back(('S', 0));
    let end = ('E', 0);

    while let Some(current_location) = queue.pop_back() {
        history.push(current_location);
        if current_location == end {
            return Some(history);
        }

        let neighbor_blobs: Vec<(char, usize)> = get_neighbor_blobs(&blobs, current_location);

        for neighbor in neighbor_blobs {
            if visited.insert(neighbor) {
                queue.push_back(neighbor);
                letters.push(neighbor.0);
            }
        }
    }
    None
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

pub type Point = (usize, usize);
pub type BlobIndex = HashMap<char, Vec<Vec<Point>>>;
pub type BlobAddress = (char, usize);

pub fn get_letter_blobs(array: &Array2D<char>) -> BlobIndex {
    let mut array_copy = array.clone().to_owned();
    let mut hashmap: HashMap<char, Vec<Vec<Point>>> = HashMap::new();
    for i in 0..array_copy.column_len() {
        for j in 0..array_copy.row_len() {
            if *array_copy.get(i, j).unwrap() != ' ' {
                let label = array_copy.get(i, j).unwrap().clone();
                let blob = destructive_flood_fill(&mut array_copy, (i, j));
                if !hashmap.contains_key(&label) {
                    hashmap.insert(label, vec![blob]);
                } else {
                    hashmap.get_mut(&label).unwrap().push(blob);
                }
            }
        }
    }
    hashmap
}

// Flood-fill algorithm for finding blobs
// Destroys the array it searches over
pub fn destructive_flood_fill(array: &mut Array2D<char>, coord: Point) -> Vec<Point> {
    let mut blob: Vec<Point> = Vec::new();
    let mut queue: Vec<Point> = Vec::new();
    let label = array.get(coord.0, coord.1).unwrap().clone();

    queue.push(coord);

    while let Some(point) = queue.pop() {
        blob.push(point);
        let neighbors = get_neighbors(&array, point, &label);
        for neighbor in neighbors {
            queue.push(neighbor);
            array.set(neighbor.0, neighbor.1, ' ').ok();
        }
    }
    return blob;
}

pub fn get_neighbor_blobs(blobs: &BlobIndex, current_location: BlobAddress) -> Vec<BlobAddress> {
    let (level, blob) = current_location;
    let next_level = next_level(level);
    let previous_level = prev_level(level);
    let current_blob = &blobs.get(&level).unwrap()[blob];
    let uplevel_blobs = blobs.get(&next_level).unwrap();
    let downlevel_blobs = blobs.get(&previous_level).unwrap();
    let mut neighbors: Vec<BlobAddress> = Vec::new();

    for i in 0..uplevel_blobs.len() {
        match find_connection(&current_blob, &uplevel_blobs[i]) {
            Some(_x) => neighbors.push((next_level, i)),
            None => (),
        }
    }
    for i in 0..downlevel_blobs.len() {
        match find_connection(&current_blob, &downlevel_blobs[i]) {
            Some(_x) => neighbors.push((previous_level, i)),
            None => (),
        }
    }
    // dirt hack
    if (level == 'r') {
        let l2_blobs = blobs.get(&prev_level(previous_level)).unwrap();
        for i in 0..l2_blobs.len() {
            match find_connection(&current_blob, &l2_blobs[i]) {
                Some(_x) => neighbors.push((prev_level(previous_level), i)),
                None => (),
            }
        }
    }
    neighbors
}

pub fn get_neighbors(
    array: &Array2D<char>,
    point: (usize, usize),
    label: &char,
) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = Vec::new();

    let cols = array.row_len();
    let rows = array.column_len();
    if point.0 + 1 < rows {
        if array.get(point.0 + 1, point.1).unwrap() == label {
            neighbors.push((point.0 + 1, point.1));
        }
    }
    // North
    if point.0 as i32 - 1 > 0 {
        if array.get(point.0 - 1, point.1).unwrap() == label {
            neighbors.push((point.0 - 1, point.1));
        }
    }

    if point.1 + 1 < cols {
        if array.get(point.0, point.1 + 1).unwrap() == label {
            neighbors.push((point.0, point.1 + 1));
        }
    }
    // West
    if (point.1 as i32) - 1 > 0 {
        if array.get(point.0, point.1 - 1).unwrap() == label {
            neighbors.push((point.0, point.1 - 1));
        }
    }
    neighbors
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
    end as i32 - start as i32
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

pub fn prev_level(current: char) -> char {
    match current {
        'S' => 'S', // hack
        'a' => 'S',
        'E' => 'z',
        _ => char::from_u32(current as u32 - 1).unwrap(),
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
