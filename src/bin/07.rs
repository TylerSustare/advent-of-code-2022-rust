use std::collections::HashMap;

use advent_of_code::helpers::get_lines_without_empty;

#[derive(Debug, PartialEq, Eq)]
struct Command {
    raw: String,
    kind: CommandKind,
}

#[derive(Debug, PartialEq, Eq)]
enum CommandKind {
    Input,
    Dir,
    File,
}

fn output_to_command(output: &&str) -> Command {
    let mut parts = output.split(" ");
    let kind = match parts.next() {
        Some("$") => CommandKind::Input,
        Some("dir") => CommandKind::Dir,
        // trust file input for AoC
        _ => CommandKind::File,
    };
    let raw = output.to_string();
    Command { raw, kind }
}

fn group_input_and_output(commands: Vec<Command>) -> Vec<Vec<Command>> {
    let mut groups = Vec::new();
    let mut group = Vec::new();
    for command in commands {
        match command.kind {
            CommandKind::Input => {
                groups.push(group);
                group = Vec::from([command]);
            }
            _ => group.push(command),
        }
    }
    groups.push(group);
    groups.into_iter().filter(|g| !g.is_empty()).collect()
}

fn process_input(
    input_group: &Vec<Command>,
    path: &mut Vec<String>,
    size_map: &mut HashMap<String, u32>,
) {
    for command in input_group {
        match command.kind {
            CommandKind::Input => {
                let mut parts = command.raw.split(" ");
                parts.next(); // get past "$"
                let cmd = parts.next().unwrap();
                match cmd {
                    "cd" => {
                        let dir = parts.next().unwrap();
                        match dir {
                            ".." => {
                                path.pop();
                            }
                            "/" => {
                                path.truncate(0);
                                path.push(dir.to_string());
                            }
                            _ => {
                                // can't declare full as `String` and assign multiple places
                                path.push(path.join("/") + dir);
                                size_map.entry(path.join("/")).or_insert(0);
                            }
                        }
                    }
                    "ls" => {}
                    _ => panic!("Unknown command: {}", cmd),
                }
            }
            CommandKind::File => {
                let mut parts = command.raw.split(" ");
                let size = parts.next().unwrap().parse::<u32>().unwrap();
                // `to_owned` to avoid `borrow of moved value: `path` error `
                for dir in path.to_owned() {
                    let entry = size_map.entry(dir.to_string()).or_insert(0);
                    *entry += size;
                }
            }
            CommandKind::Dir => {}
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let terminal_output = get_lines_without_empty(input);
    let commands = terminal_output
        .iter()
        .map(output_to_command)
        .collect::<Vec<Command>>();
    let groups = group_input_and_output(commands);
    // mutable state through the commands
    // use String as key, not &str for lifetime memory
    let mut path: Vec<String> = Vec::new();
    let mut size_map: HashMap<String, u32> = HashMap::new();
    for group in groups {
        process_input(&group, &mut path, &mut size_map);
    }
    let sum: u32 = size_map
        .values()
        .fold(0, |acc, x| if x < &100000 { acc + x } else { acc });

    Some(sum)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
    fn test_ls_adds_to_path() {
        let input = Vec::from([
            Command {
                raw: "$ cd /".to_string(),
                kind: CommandKind::Input,
            },
            Command {
                raw: "$ ls".to_string(),
                kind: CommandKind::Input,
            },
            Command {
                raw: "dir a".to_string(),
                kind: CommandKind::Dir,
            },
            Command {
                raw: "14848514 b.txt".to_string(),
                kind: CommandKind::File,
            },
            Command {
                raw: "8504156 c.dat".to_string(),
                kind: CommandKind::File,
            },
            Command {
                raw: "dir d".to_string(),
                kind: CommandKind::Dir,
            },
        ]);

        let expected: Vec<Vec<Command>> = Vec::from([
            Vec::from([Command {
                raw: "$ cd /".to_string(),
                kind: CommandKind::Input,
            }]),
            Vec::from([
                Command {
                    raw: "$ ls".to_string(),
                    kind: CommandKind::Input,
                },
                Command {
                    raw: "dir a".to_string(),
                    kind: CommandKind::Dir,
                },
                Command {
                    raw: "14848514 b.txt".to_string(),
                    kind: CommandKind::File,
                },
                Command {
                    raw: "8504156 c.dat".to_string(),
                    kind: CommandKind::File,
                },
                Command {
                    raw: "dir d".to_string(),
                    kind: CommandKind::Dir,
                },
            ]),
        ]);

        assert_eq!(group_input_and_output(input), (expected));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}

// Requirements

// `ls` should gather the file sizes and add them
//   to the current and parent directory size

// `dir` should add an entry to the map for a directory

// path should be a vector of strings.
//   when we add file sizes we can add them to all entries in the map
//   contained in the `path` vector
// e.g.
//   path = ["a", "b"]
//   map = {
//     "a": 0,
//     "b": 0,
//   }
//   file.jj = 10
//   map = {
//     "a": 10,
//     "b": 10,
//   }
//   path = ["a", "b", "c"]
//   file.css = 10
//   map = {
//     "a": 20,
//     "b": 20,
//     "c": 10,
//    }

// `cd ..` should just pop the last entry off the path vector

// `cd <dir>` should push the dir onto the path vector

// `cd /` should clear the path vector
