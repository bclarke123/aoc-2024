use regex::Regex;

const INPUT: &str = include_str!("../include/day3/input.txt");
const REGEX: &str = r"mul\((\d{1,3}),(\d{1,3})\)";

fn exec(input: &str, re: &Regex) -> u32 {
    re.captures_iter(input).map(|c| c.extract())
        .map(|(_, [a, b])| a.parse().unwrap_or(0) * b.parse().unwrap_or(0))
        .sum()
}

fn part1(input: &str) -> u32 {
    let re = Regex::new(REGEX).unwrap();
    exec(input, &re)
}

fn part2(input: &str) -> u32 {
    let re = Regex::new(REGEX).unwrap();
    let mut ret = 0;
    let mut remaining = input;
    loop {
        if let Some((next, rem)) = remaining.split_once("don't()") {
            ret += exec(next, &re);

            match rem.split_once("do()") {
                Some((_, rem)) => remaining = rem,
                None => break
            }

            continue;
        }

        ret += part1(remaining);

        break;
    }

    ret
}

fn main() { 
    let ret = part1(INPUT);
    println!("The first result is {ret}");

    let ret = part2(INPUT);
    println!("The second result is {ret}");
}

#[test]
fn test_part1() {
    let input = include_str!("../include/day3/sample.txt");
    let ret = part1(input);
    assert_eq!(161, ret);
}

#[test]
fn test_part2() {
    let input = include_str!("../include/day3/sample2.txt");
    let ret = part2(input);
    assert_eq!(48, ret);
}