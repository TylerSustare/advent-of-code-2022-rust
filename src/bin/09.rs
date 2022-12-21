use advent_of_code::helpers::get_lines_without_empty;
use std::collections::HashSet;

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug)]
struct Rope {
    // faux x and y coordinates
    tail: (i32, i32),
    head: (i32, i32),
    tail_visited: HashSet<(i32, i32)>,
}

impl Rope {
    fn new() -> Rope {
        Rope {
            tail: (0, 0),
            head: (0, 0),
            tail_visited: HashSet::from([(0, 0)]),
        }
    }

    // for testing
    fn _from(head: (i32, i32), tail: (i32, i32)) -> Rope {
        Rope {
            tail,
            head,
            tail_visited: HashSet::from([tail]),
        }
    }

    // `move` is a reserved word
    fn traverse(&mut self, direction: Direction, distance: i32) {
        match direction {
            Direction::Up | Direction::Down => self.move_vertical(direction, distance),
            Direction::Right | Direction::Left => self.move_horizontal(direction, distance),
        }
    }

    fn move_vertical(&mut self, direction: Direction, distance: i32) {
        for _ in 0..distance {
            if direction == Direction::Up {
                self.head.1 += 1;
            } else {
                self.head.1 -= 1;
            }
            // if the tail is 2 below the head, move it one up
            if self.tail.1 == self.head.1 - 2 && self.tail.0 == self.head.0 {
                self.tail.1 += 1;
            }
            // if the tail is 2 above the head, move it one up
            if self.tail.1 == self.head.1 + 2 && self.tail.0 == self.head.0 {
                self.tail.1 -= 1;
            }

            let lower_right = (self.tail.0 == self.head.0 + 1) && (self.tail.1 == self.head.1 - 2);
            let lower_left = (self.tail.0 == self.head.0 - 1) && (self.tail.1 == self.head.1 - 2);
            let upper_right = (self.tail.0 == self.head.0 + 1) && (self.tail.1 == self.head.1 + 2);
            let upper_left = (self.tail.0 == self.head.0 - 1) && (self.tail.1 == self.head.1 + 2);
            if lower_right {
                self.tail.0 -= 1;
                self.tail.1 += 1;
            }
            if lower_left {
                self.tail.0 += 1;
                self.tail.1 += 1;
            }
            if upper_right {
                self.tail.0 -= 1;
                self.tail.1 -= 1;
            }
            if upper_left {
                self.tail.0 += 1;
                self.tail.1 -= 1;
            }

            self.tail_visited.insert(self.tail);
        }
    }

    fn move_horizontal(&mut self, direction: Direction, distance: i32) {
        for _ in 0..distance {
            if direction == Direction::Right {
                self.head.0 += 1;
            } else {
                self.head.0 -= 1;
            }

            // diagonal tail movement
            let upper_right = (self.tail.0 == self.head.0 - 2) && (self.tail.1 == self.head.1 - 1);
            let lower_right = (self.tail.0 == self.head.0 - 2) && (self.tail.1 == self.head.1 + 1);
            let lower_left = (self.tail.0 == self.head.0 + 2) && (self.tail.1 == self.head.1 + 1);
            let upper_left = (self.tail.0 == self.head.0 + 2) && (self.tail.1 == self.head.1 - 1);
            if upper_right {
                self.tail.0 += 1;
                self.tail.1 += 1;
            }
            if lower_right {
                self.tail.0 += 1;
                self.tail.1 -= 1;
            }
            if upper_left {
                self.tail.0 -= 1;
                self.tail.1 += 1;
            }
            if lower_left {
                self.tail.0 -= 1;
                self.tail.1 -= 1;
            }
            // if the tail is 2 behind the head to the left, move it one right
            if self.tail.0 == self.head.0 - 2 && self.tail.1 == self.head.1 {
                self.tail.0 += 1;
            }
            // if the tail is 2 behind the head to the right, move it one left
            if self.tail.0 == self.head.0 + 2 && self.tail.1 == self.head.1 {
                self.tail.0 -= 1;
            }
            self.tail_visited.insert(self.tail);
        }
    }
}

fn parse_line(line: &str) -> (Direction, i32) {
    // let ch = line.chars().nth(0).unwrap();
    let commands = line.split(' ').collect::<Vec<&str>>();
    let direction = match commands[0] {
        "R" => Direction::Right,
        "L" => Direction::Left,
        "U" => Direction::Up,
        "D" => Direction::Down,
        _ => panic!("Invalid direction"),
    };
    let distance = commands[1].parse::<i32>().unwrap();
    (direction, distance)
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = get_lines_without_empty(input);
    let mut rope = Rope::new();
    for line in lines {
        let (direction, distance) = parse_line(line);
        rope.traverse(direction, distance);
    }
    Some(rope.tail_visited.len() as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rope_traverse_one_down() {
        let mut rope = Rope::new();
        rope.traverse(Direction::Down, 1);
        assert_eq!(rope.tail, (0, 0));
        assert_eq!(rope.head, (0, -1));
        assert_eq!(rope.tail_visited.len(), 1);
    }

    #[test]
    fn test_rope_traverse_two_down() {
        let mut rope = Rope::new();
        rope.traverse(Direction::Down, 2);
        assert_eq!(rope.tail, (0, -1));
        assert_eq!(rope.head, (0, -2));
        assert_eq!(rope.tail_visited.len(), 2);
    }

    #[test]
    fn test_rope_traverse_two_down_without_tail_moving() {
        let mut rope = Rope::_from((0, 2), (0, 1));
        // HT = H & T on the same spot
        // * * -> *  * -> *  *
        // H * -> *  * -> *  *
        // T * -> HT * -> T  *
        // * * -> *  * -> H  *
        rope.traverse(Direction::Down, 2);
        assert_eq!(rope.tail, (0, 1));
        assert_eq!(rope.head, (0, 0));
        assert_eq!(rope.tail_visited.len(), 1);
    }

    #[test]
    fn test_rope_traverse_down_diag_upper_right() {
        // if the tail is diagonal
        // head (1, 1) tail (2, 2)
        // * * T * *
        // * H * * *
        // * * * * *
        //
        // H (1, 0) T (2, 2)
        // * * T * *
        // * * * * *
        // * H * * *
        //
        // H (1, 0) T (1, 1)
        // * * * * *
        // * T * * *
        // * H * * *
        let mut rope = Rope::_from((1, 1), (2, 2));
        rope.traverse(Direction::Down, 1);
        assert_eq!(rope.tail, (1, 1));
        assert_eq!(rope.head, (1, 0));
        assert_eq!(rope.tail_visited.len(), 2);
    }

    #[test]
    fn test_rope_traverse_down_diag_upper_left() {
        // if the tail is diagonal
        // head (1, 1) tail (0, 2)
        // T * * * *
        // * H * * *
        // * * * * *
        //
        // H (1, 0) T (0, 2)
        // T * * * *
        // * * * * *
        // * H * * *
        //
        // H (1, 0) T (1, 1)
        // * * * * *
        // * T * * *
        // * H * * *
        let mut rope = Rope::_from((1, 1), (0, 2));
        rope.traverse(Direction::Down, 1);
        assert_eq!(rope.tail, (1, 1));
        assert_eq!(rope.head, (1, 0));
        assert_eq!(rope.tail_visited.len(), 2);
    }

    #[test]
    fn test_rope_traverse_one_up() {
        let mut rope = Rope::new();
        rope.traverse(Direction::Up, 1);
        assert_eq!(rope.tail, (0, 0));
        assert_eq!(rope.head, (0, 1));
        assert_eq!(rope.tail_visited.len(), 1);
    }

    #[test]
    fn test_rope_traverse_two_up() {
        let mut rope = Rope::new();
        rope.traverse(Direction::Up, 2);
        assert_eq!(rope.tail, (0, 1));
        assert_eq!(rope.head, (0, 2));
        assert_eq!(rope.tail_visited.len(), 2);
    }

    #[test]
    fn test_rope_traverse_up_diag_lower_right() {
        // if the tail is diagonal
        // head (1, 1) tail (2, 0)
        // * * * * *
        // * H * * *
        // * * T * *
        //
        // H (1, 2) T (2, 0)
        // * H * * *
        // * * * * *
        // * * T * *
        //
        // H (1, 2) T (1, 1)
        // * H * * *
        // * T * * *
        // * * * * *
        let mut rope = Rope::_from((1, 1), (2, 0));
        rope.traverse(Direction::Up, 1);
        assert_eq!(rope.tail, (1, 1));
        assert_eq!(rope.head, (1, 2));
        assert_eq!(rope.tail_visited.len(), 2);
    }

    #[test]
    fn test_rope_traverse_up_diag_lower_left() {
        // if the tail is diagonal
        // head (2, 1) tail (1, 0)
        // * * * * *
        // * * H * *
        // * T * * *
        //
        // H (2, 2) T (1, 0)
        // * * H * *
        // * * * * *
        // * T * * *
        //
        // H (2, 2) T (2, 1)
        // * * H * *
        // * * T * *
        // * * * * *
        let mut rope = Rope::_from((2, 1), (1, 0));
        rope.traverse(Direction::Up, 1);
        assert_eq!(rope.tail, (2, 1));
        assert_eq!(rope.head, (2, 2));
        assert_eq!(rope.tail_visited.len(), 2);
    }

    #[test]
    fn test_rope_traverse_one_left() {
        let mut rope = Rope::new();
        rope.traverse(Direction::Left, 1);
        assert_eq!(rope.tail, (0, 0));
        assert_eq!(rope.head, (-1, 0));
        assert_eq!(rope.tail_visited.len(), 1);
    }

    #[test]
    fn test_rope_traverse_diag_upper_left() {
        // if the tail is diagonal
        // head (1, 1) tail (2, 0)
        // * H * * *
        // * * T * *
        //
        // H (0, 1) T (2, 0)
        // H * * * *
        // * * T * *
        //
        // H (0, 1) T (1, 1)
        // H T * * *
        // * * * * *
        let mut rope = Rope::_from((1, 1), (2, 0));
        rope.traverse(Direction::Left, 1);
        assert_eq!(rope.head, (0, 1));
        assert_eq!(rope.tail, (1, 1));
        assert_eq!(rope.tail_visited.len(), 2);
    }

    #[test]
    fn test_rope_traverse_diag_lower_left() {
        // if the tail is diagonal
        // head (1, 0) tail (2, 1)
        // * * T * *
        // * H * * *
        //
        // H (0, 0) T (2, 1)
        // * * T * *
        // H * * * *
        //
        // H (0, 0) T (1, 0)
        // * * * * *
        // H T * * *
        let mut rope = Rope::_from((1, 0), (2, 1));
        rope.traverse(Direction::Left, 1);
        assert_eq!(rope.head, (0, 0));
        assert_eq!(rope.tail, (1, 0));
        assert_eq!(rope.tail_visited.len(), 2);
    }

    #[test]
    fn test_rope_new() {
        let rope = Rope::new();
        assert_eq!(rope.tail, (0, 0));
        assert_eq!(rope.head, (0, 0));
        assert_eq!(rope.tail_visited.len(), 1);
    }

    #[test]
    fn test_rope_traverse_one_right() {
        let mut rope = Rope::new();
        rope.traverse(Direction::Right, 1);
        assert_eq!(rope.tail, (0, 0));
        assert_eq!(rope.head, (1, 0));
        assert_eq!(rope.tail_visited.len(), 1);
    }

    #[test]
    fn test_rope_traverse_2_right() {
        let mut rope = Rope::new();
        rope.traverse(Direction::Right, 2);
        assert_eq!(rope.tail, (1, 0));
        assert_eq!(rope.head, (2, 0));
        assert_eq!(rope.tail_visited.len(), 2);
    }

    #[test]
    fn test_rope_traverse_diag_upper_right() {
        // if the tail is diagonal
        // head (1, 1) tail (0, 0)
        // * H * * *
        // T * * * *
        //
        // H (2, 1) T (0, 0)
        // * * H * *
        // T * * * *
        //
        // H (2, 1) T (1, 1)
        // * T H * *
        // * * * * *
        let mut rope = Rope::_from((1, 1), (0, 0));
        rope.traverse(Direction::Right, 1);
        assert_eq!(rope.head, (2, 1));
        assert_eq!(rope.tail, (1, 1));
    }

    #[test]
    fn test_rope_traverse_diag_lower_right() {
        // H (1, 0) T (0, 1)
        // * * * * *
        // T * * * *
        // * H * * *
        //
        // H (2, 0) T (0, 1)
        // * * * * *
        // T * * * *
        // * * H * *
        //
        // H (2, 0) T (1, 0)
        // * * * * *
        // * * * * *
        // * T H * *

        let mut rope = Rope::_from((1, 0), (0, 1));
        rope.traverse(Direction::Right, 1);
        assert_eq!(rope.head, (2, 0));
        assert_eq!(rope.tail, (1, 0));
    }

    #[test]
    fn test_rope_traverse_right_without_diag() {
        // H (1, 2) T (2, 1)
        // * H * * *
        // * * T * *
        // * * * * *
        //
        // H (2, 2) T (2, 1)
        // * * H * *
        // * * T * *
        // * * * * *
        //
        // H (3, 2) T (2, 1)
        // * * * H *
        // * * T * *
        // * * * * *
        let mut rope = Rope::_from((1, 2), (2, 1));
        rope.traverse(Direction::Right, 2);
        assert_eq!(rope.head, (3, 2));
        assert_eq!(rope.tail, (2, 1));
        // tail shouldn't have moved
        assert_eq!(rope.tail_visited.len(), 1);
        // H (4, 2) T (2, 1)
        // * * * * H
        // * * T * *
        // * * * * *
        //
        // H (4, 2) T (3, 2)
        // * * * T H
        // * * * * *
        // * * * * *
        rope.traverse(Direction::Right, 1);
        assert_eq!(rope.head, (4, 2));
        assert_eq!(rope.tail, (3, 2));
        assert_eq!(rope.tail_visited.len(), 2);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }
}
