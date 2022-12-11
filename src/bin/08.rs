pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let cols = grid[0].len();
    let rows = grid.len();
    let mut visible = vec![vec![0; cols]; rows];

    // top
    visible[..][0] = vec![1; cols];

    // left
    for row in 0..rows {
        visible[row][0] = 1;
    }

    // bottom
    visible[..][cols - 1] = vec![1; rows];

    // right
    for i in 0..cols {
        visible[i][rows - 1] = 1;
    }

    for row in 1..(rows - 1) {
        for col in 1..(cols - 1) {
            // top
            let mut top_view: Vec<u32> = Vec::new();
            for i in 0..row {
                top_view.push(grid[i][col]);
            }
            top_view.sort();
            if top_view.last().unwrap() < &grid[row][col] {
                visible[row][col] = 1;
            }

            // bottom
            let mut bottom_view: Vec<u32> = Vec::new();
            for i in (row + 1)..rows {
                bottom_view.push(grid[i][col]);
            }
            bottom_view.sort();
            if bottom_view.last().unwrap() < &grid[row][col] {
                visible[row][col] = 1;
            }

            // left
            let mut left_view: Vec<u32> = grid[row][0..col].to_vec();
            left_view.sort();
            if left_view.last().unwrap() < &grid[row][col] {
                visible[row][col] = 1;
            }

            // right
            let mut right_view: Vec<u32> = grid[row][col + 1..cols].to_vec();
            right_view.sort();
            if right_view.last().unwrap() < &grid[row][col] {
                visible[row][col] = 1;
            }
        }
    }
    Some(sum_array(visible))
}

pub fn sum_array(input: Vec<Vec<u32>>) -> u32 {
    let mut accum = 0;
    for rows in input {
        for col in rows {
            accum += col;
        }
    }
    accum
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let cols = grid[0].len();
    let rows = grid.len();
    let mut visible = vec![vec![0; cols]; rows];

    // top
    visible[..][0] = vec![1; cols];

    // left
    for row in 0..rows {
        visible[row][0] = 1;
    }

    // bottom
    visible[..][cols - 1] = vec![1; rows];

    // right
    for i in 0..cols {
        visible[i][rows - 1] = 1;
    }

    for row in 1..(rows - 1) {
        for col in 1..(cols - 1) {
            // top
            let mut top_score = 0;
            let mut top_view: Vec<u32> = Vec::new();
            for i in 0..row {
                top_view.push(grid[i][col]);
            }
            top_view.reverse();
            for tree in top_view {
                top_score += 1;
                if tree >= grid[row][col] {
                    break;
                }
            }

            // bottom
            let mut bottom_score = 0;
            let mut bottom_view: Vec<u32> = Vec::new();
            for i in (row + 1)..rows {
                bottom_view.push(grid[i][col]);
            }
            for tree in bottom_view {
                bottom_score += 1;
                if tree >= grid[row][col] {
                    break;
                }
            }

            // left
            let mut left_score = 0;
            let mut left_view: Vec<u32> = grid[row][0..col].to_vec();
            left_view.reverse();
            for tree in left_view {
                left_score += 1;
                if tree >= grid[row][col] {
                    break;
                }
            }

            // right
            let mut right_score = 0;
            let right_view: Vec<u32> = grid[row][col + 1..cols].to_vec();
            for tree in right_view {
                right_score += 1;
                if tree >= grid[row][col] {
                    break;
                }
            }
            visible[row][col] = top_score * left_score * right_score * bottom_score;
        }
    }
    Some(get_max_element(visible))
}

pub fn get_max_element(input: Vec<Vec<u32>>) -> u32 {
    let mut max = 0;
    for rows in input {
        for col in rows {
            if col > max {
                max = col;
            }
        }
    }
    max
}

pub fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let line_array: Vec<&str> = input.lines().collect();
    let height = line_array.len();
    let width = line_array
        .first()
        .unwrap()
        .chars()
        .collect::<Vec<char>>()
        .len();
    let mut grid = vec![vec![0; width]; height];
    for (i, line) in line_array.iter().enumerate() {
        for (j, char) in line.chars().enumerate() {
            grid[i][j] = char.to_digit(10).unwrap();
        }
    }
    grid
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
