use advent_of_code::helpers::get_lines_without_empty;

fn get_same_letters_in_line(lines: Vec<&str>) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();
    lines.iter().copied().for_each(|line| {
        let half = line.len() / 2;
        let chars: Vec<char> = line.chars().collect();
        let (first, second) = chars.split_at(half);

        for c in first.iter() {
            if second.contains(c) {
                result.push(*c);
                break;
            }
        }
    });
    result
}

fn get_same_letter_in_lines(lines: Vec<&str>) -> char {
    match lines.as_slice() {
        [first, second, third] => {
            let first_chars: Vec<char> = first.chars().collect();
            let second_chars: Vec<char> = second.chars().collect();
            let third_chars: Vec<char> = third.chars().collect();

            for c in first_chars.iter() {
                if second_chars.contains(c) && third_chars.contains(c) {
                    return c.to_owned();
                }
            }
        }
        _ => panic!("Unexpected number of lines"),
    }
    panic!("no letter found for group");
}

fn get_points(chars: Vec<char>) -> u32 {
    let mut result: u32 = 0;
    chars.iter().for_each(|c| result += points_for_char(*c));
    result
}

fn points_for_char(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 96
    } else {
        c as u32 - 38
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = get_lines_without_empty(input);
    let same_letters = get_same_letters_in_line(lines);
    Some(get_points(same_letters))
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = get_lines_without_empty(input);
    let groups: Vec<&[&str]> = lines.chunks(3).collect();
    let sum = groups
        .iter()
        .map(|group| get_same_letter_in_lines(group.to_vec()))
        .fold(0, |acc, c| acc + points_for_char(c));
    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
