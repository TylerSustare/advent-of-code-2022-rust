use std::collections::HashSet;

use advent_of_code::helpers::get_lines_without_empty;

fn uniques(beginning: usize, end: usize, chars: &Vec<char>) -> HashSet<char> {
    let set: HashSet<char> = HashSet::new();
    chars[beginning..=end].iter().fold(set, |mut set, &c| {
        set.insert(c);
        set
    })
}

fn first_unique_sequence(size: usize, line: &str) -> u32 {
    let mut beginning: usize = 0;
    let mut end: u32 = (size - 1) as u32;
    let chars: Vec<char> = line.chars().collect();
    while end <= line.len() as u32 {
        beginning += 1;
        end += 1;
        if uniques(beginning, end as usize, &chars).len() == size {
            break;
        }
    }
    end + 1
}

pub fn part_one(input: &str) -> Option<u32> {
    let line = *get_lines_without_empty(input).first().unwrap();
    Some(first_unique_sequence(4, line))
}

pub fn part_two(input: &str) -> Option<u32> {
    let line = *get_lines_without_empty(input).first().unwrap();
    Some(first_unique_sequence(14, line))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(5));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
