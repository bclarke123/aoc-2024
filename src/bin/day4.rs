const SEARCH: &str = "XMAS";

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect()
}

fn part1(input: &str) -> usize {
    let mut ret = 0;

    let chars = parse_input(input);
    let cols = chars[0].len();
    let rows = chars.len();

    let mut buf = "";

    

    ret
}

fn main() {}

#[test]
fn test_part1() {
    let input = include_str!("../include/day4/sample.txt");
    let ret = part1(input);
    assert_eq!(18, ret);
}

#[test]
fn test_part2() {

}