/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

pub fn get_all_lines(input: &str) -> Vec<&str> {
    input.split("\n").collect()
}
