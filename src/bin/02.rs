use advent_of_code::helpers::get_lines_without_empty;
use std::collections::HashMap;

fn rps_mapping() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("X", "rock"),
        ("Y", "paper"),
        ("Z", "scissors"),
        ("A", "rock"),
        ("B", "paper"),
        ("C", "scissors"),
    ])
}

fn cheating_mapping() -> HashMap<&'static str, u32> {
    HashMap::from([("X", 0), ("Y", 3), ("Z", 6)])
}

fn rps_outcomes() -> HashMap<[&'static str; 2], u32> {
    HashMap::from([
        (["rock", "rock"], 3),
        (["rock", "paper"], 6),
        (["rock", "scissors"], 0),
        (["paper", "rock"], 0),
        (["paper", "paper"], 3),
        (["paper", "scissors"], 6),
        (["scissors", "rock"], 6),
        (["scissors", "paper"], 0),
        (["scissors", "scissors"], 3),
    ])
}

fn needed_to_win(theirs: &str) -> &str {
    if theirs == "rock" {
        return "paper";
    }
    if theirs == "paper" {
        return "scissors";
    }
    "rock"
}

fn needed_to_lose(theirs: &str) -> &str {
    if theirs == "rock" {
        return "scissors";
    }
    if theirs == "paper" {
        return "rock";
    }
    "paper"
}

fn play_scores() -> HashMap<&'static str, u32> {
    HashMap::from([("rock", 1), ("paper", 2), ("scissors", 3)])
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = get_lines_without_empty(input);
    let rps_mapping = rps_mapping();
    let rps_outcomes = rps_outcomes();
    let play_scores = play_scores();
    let mut score: u32 = 0;
    lines.iter().for_each(|line| {
        let commands: Vec<&str> = line.split(" ").collect();
        score += play_scores[&rps_mapping[commands[1]]];
        score += rps_outcomes[&[rps_mapping[commands[0]], rps_mapping[commands[1]]]];
    });
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = get_lines_without_empty(input);
    let cheating_mapping = cheating_mapping();
    let play_scores = play_scores();
    let rps_mapping = rps_mapping();
    let mut score: u32 = 0;

    lines.iter().for_each(|line| {
        let commands: Vec<&str> = line.split(" ").collect();
        score += cheating_mapping[commands[1]];
        let their_play = rps_mapping[commands[0]];
        // do we need to lose, draw or win?
        let outcome = commands[1];
        if outcome == "Y" {
            score += play_scores[their_play];
        }
        if outcome == "X" {
            score += play_scores[needed_to_lose(their_play)];
        }
        if outcome == "Z" {
            score += play_scores[needed_to_win(their_play)];
        }
    });

    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
