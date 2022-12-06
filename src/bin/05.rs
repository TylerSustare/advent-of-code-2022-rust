use advent_of_code::helpers::get_all_lines;

#[derive(Clone, Debug)]
struct Crate {
    contents: String,
}

fn fill_stacks(stack_chars: Vec<&str>) -> Vec<Vec<Crate>> {
    let stack_count = number_of_stacks(stack_chars.last().unwrap());
    let mut stacks = crate_stacks(stack_count.unwrap());
    let only_crate_chars = stack_chars[..stack_chars.len() - 1].to_vec();
    for (_, crate_chars) in only_crate_chars.iter().enumerate() {
        let crate_contents = split_crate_chars_to_stacks(crate_chars);

        // push crates
        for (i, crate_box) in crate_contents.iter().enumerate() {
            if crate_box.contents != " " {
                stacks[i].push(crate_box.clone());
            }
        }
    }
    stacks
        .iter_mut()
        .map(|stack| {
            stack.reverse();
            stack.clone()
        })
        .collect::<Vec<Vec<Crate>>>()
}

fn split_crate_chars_to_stacks(stack: &&str) -> Vec<Crate> {
    let mut placeholder = 0;
    let without_spaces: Vec<char> = stack
        .chars()
        .filter(|_| {
            placeholder += 1;
            placeholder % 4 != 0
        })
        .collect();
    without_spaces
        .chunks(3)
        .map(|c| Crate {
            contents: c[1].to_string(),
        })
        .collect::<Vec<Crate>>()
}

fn crate_stacks(number_of_stacks: u32) -> Vec<Vec<Crate>> {
    let mut stacks = Vec::new();
    for _ in 0..number_of_stacks {
        stacks.push(Vec::new());
    }
    stacks
}

fn number_of_stacks(input: &str) -> Option<u32> {
    input
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<Vec<char>>()
        .last()
        .unwrap()
        .to_digit(10)
}

fn separate_input(lines: Vec<&str>) -> (Vec<&str>, Vec<&str>) {
    let mut parts = lines.split(|line| line.is_empty());
    (
        parts.next().unwrap().to_vec(),
        parts.next().unwrap().to_vec(),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    let parts = separate_input(get_all_lines(input));
    let (stack_chars, commands) = parts;
    let mut stacks = fill_stacks(stack_chars);
    let number_commands: Vec<Vec<u32>> = commands
        .iter()
        .map(|f| -> Vec<u32> {
            let mut number_commands: String;
            number_commands = f.replace("move", "");
            number_commands = number_commands.replace("from", "");
            number_commands = number_commands.replace("to", "");
            number_commands = number_commands.replace(" ", "");
            number_commands
                .chars()
                .map(|s| s.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    for command in number_commands {
        let number_to_move = command[0] as usize;
        let from = command[1] as usize;
        let to = command[2] as usize;
        println!("***************");
        println!("{:?}", stacks);
        println!("***************");
        for _ in 0..number_to_move {
            if stacks[from - 1].len() > 0 {
                let crate_to_move = stacks[from - 1].pop().unwrap();
                stacks[to - 1].push(crate_to_move);
            }
        }
    }
    println!("***************");
    for stack in stacks.iter() {
        println!("{:?}", stack.last().unwrap().contents);
    }
    println!("***************");
    None
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}
