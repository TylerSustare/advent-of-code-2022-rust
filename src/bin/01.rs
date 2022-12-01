pub fn part_one(input: &str) -> Option<u32> {
    let lines = get_lines(input);
    let elves = get_elves_snacks(lines);
    let total_calories = get_elves_total_calories(elves);

    total_calories.iter().max().to_owned().copied()
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = get_lines(input);
    let elves = get_elves_snacks(lines);
    let total_calories = get_elves_total_calories(elves);

    // just sort them and return the top 3 values
    let mut total_calories: Vec<u32> = total_calories.iter().copied().collect();
    total_calories.sort();
    total_calories.reverse();
    total_calories.truncate(3);
    let top_3: u32 = total_calories.iter().sum();
    Some(top_3)
}

fn get_lines(input: &str) -> Vec<&str> {
    input.split("\n").collect()
}

fn get_elves_snacks(lines: Vec<&str>) -> Vec<Vec<&str>> {
    // make a new vector of vectors by splitting array on empty string
    let mut groups: Vec<Vec<&str>> = Vec::new();
    let mut group: Vec<&str> = Vec::new();
    for line in lines {
        if line == "" {
            groups.push(group);
            group = Vec::new();
        } else {
            group.push(line);
        }
    }
    groups
}

fn get_elves_total_calories(elves: Vec<Vec<&str>>) -> Vec<u32> {
    // find the sum of each vector in `elves`
    let mut total_calories: Vec<u32> = Vec::new();
    for elf in elves {
        let mut sum = 0;
        for line in elf {
            sum += line.parse::<u32>().unwrap();
        }
        total_calories.push(sum)
    }
    total_calories
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        let got = part_one(&input);
        let want = Some(24000);
        assert_eq!(got, want);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        let got = part_two(&input);
        let want = Some(45000);
        assert_eq!(got, want);
    }
}
