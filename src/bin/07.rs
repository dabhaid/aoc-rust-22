use std::{collections::HashMap, hash::Hash};

pub fn part_one(input: &str) -> Option<u64> {
    let files = parse_input(input);
    let dir_sizes: HashMap<String, u64> = get_dir_sizes(files);
    let mut accum: u64 = 0;
    let values = dir_sizes.values();
    for value in values {
        if value <= &100000 {
            accum += value;
        }
    }
    return Some(accum);
}

pub fn get_dir_sizes(files: HashMap<String, u64>) -> HashMap<String, u64> {
    let keys: Vec<_> = files.keys().cloned().collect();
    let mut dir_sizes: HashMap<String, u64> = HashMap::new();
    for key in keys {
        let file_size = files.get(&key).unwrap();
        //let mut dir = key.rsplit_once("/").unwrap().0;
        let dirs = get_all_parent_dirs(&key).unwrap();
        // add the files to all the dirs
        for dir in dirs {
            let cur_size = dir_sizes.get(dir);
            if let Some(size) = cur_size {
                dir_sizes.insert(dir.to_owned(), file_size + size);
            } else {
                dir_sizes.insert(dir.to_owned(), *file_size);
            }
        }
    }
    dir_sizes
}

pub fn get_all_parent_dirs(path: &str) -> Option<Vec<&str>> {
    let mut paths: Vec<&str> = Vec::new();
    let mut split = path.rsplit_once("/");
    while split != None {
        let parent_path = split.unwrap().0;
        paths.push(parent_path);
        split = parent_path.rsplit_once("/");
    }
    Some(paths)
}

pub fn part_two(input: &str) -> Option<u64> {
    let total_space = 70000000;
    let needed_free = 30000000;
    let files = parse_input(input);
    let dir_sizes: HashMap<String, u64> = get_dir_sizes(files);
    let used_space = dir_sizes.get("").unwrap();
    let space_to_reclaim = needed_free - (total_space - used_space);
    let mut delta = *used_space;
    let mut size = 0;
    for dir in dir_sizes {
        if dir.1 > space_to_reclaim && dir.1 - space_to_reclaim < delta {
            delta = (dir.1 - space_to_reclaim).clone();
            size = dir.1;
        }
    }
    Some(size)
}

// going to unwisely assume section 2 doesn't allow multiple file roots
pub fn parse_input(input: &str) -> HashMap<String, u64> {
    let mut dirish: HashMap<String, u64> = HashMap::new();
    let mut path = String::new();
    // let suffixes = [".txt", ".dat", ".log", ".lst"];
    for line in input.lines() {
        match line {
            // pwd.contents.push(Box::new(File::new(size_name[1], size_name[0].parse().unwrap())));
            _ if line.starts_with("$ cd") => match line {
                _ if line.ends_with("..") => {
                    let (strip_path, _trail): (&str, &str) = path.rsplit_once("/").unwrap();
                    let (new_path, _popped) = strip_path.rsplit_once("/").unwrap();
                    path = new_path.to_string();
                    path += "/";
                }
                _ => {
                    let (_command, dir) = line.rsplit_once(" ").unwrap();
                    if dir == "/" {
                        path = dir.to_string()
                    } else {
                        path += dir;
                        path += "/";
                    }
                }
            },
            _ if line.starts_with("$ ls") => (),
            _ if line.starts_with("dir") => (),
            _ => {
                let size_name: Vec<&str> = line.split_ascii_whitespace().collect();
                let size = size_name[0].parse().unwrap();
                let file_path = path.clone() + size_name[1];
                dirish.insert(file_path, size);
            }
        }
    }
    dirish
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
