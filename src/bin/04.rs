use advent_of_code::helpers::get_lines_without_empty;

fn pairs(input: &str) -> Vec<&str> {
    input.split(',').collect()
}

fn sequence(range: &str) -> Vec<u32> {
    let mut range = range.split('-').map(|x| x.parse::<u32>().unwrap());
    let start: u32 = range.next().unwrap();
    let end: u32 = range.next().unwrap();

    (start..end + 1).collect()
}

fn fully_contains(first: Vec<u32>, second: Vec<u32>) -> bool {
    first.iter().all(|x: &u32| second.contains(x)) || second.iter().all(|x: &u32| first.contains(x))
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut fully_contained_total = 0;
    let lines: Vec<&str> = get_lines_without_empty(input);
    let pairs: Vec<Vec<&str>> = lines.iter().map(|line: &&str| pairs(*line)).collect();
    pairs.iter().for_each(|pair: &Vec<&str>| {
        let first: Vec<u32> = sequence(pair[0]);
        let second: Vec<u32> = sequence(pair[1]);
        if fully_contains(first, second) {
            fully_contained_total += 1;
        }
    });
    Some(fully_contained_total)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), None);
    }
}
