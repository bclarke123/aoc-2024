const INPUT: &str = include_str!("../include/day2/input.txt");

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn is_safe(levels: &[i32]) -> bool {
    let mut start = levels[0];
    let sign = (start - levels[1]).signum();

    for i in 1..levels.len() {
        let next = levels[i];
        let d = start.abs_diff(next);
        let s = (start - next).signum();

        let out_range = d < 1 || d > 3;
        let bad_sign = s != sign;

        if !(out_range || bad_sign) {
            start = next;
            continue;
        }

        return false;
    }

    true
}

fn count(input: &str) -> usize {
    parse_input(input)
        .iter()
        .map(|report| is_safe(report))
        .filter(|&result| result)
        .count()
}

fn part2(input: &str) -> usize {
    let reports = parse_input(input);
    let mut total = 0;
    for report in reports {
        for i in 0..report.len() {
            let mut tmp = report.clone();
            tmp.remove(i);
            if is_safe(&tmp) {
                total += 1;
                break;
            }
        }
    }

    total
}

fn main() {
    let safe = count(INPUT);
    println!("Part 1: {safe} reports are safe");

    let safe = part2(INPUT);
    println!("Part 2: {safe} reports are safe");
}

#[test]
fn test_part1() {
    let input = include_str!("../include/day2/sample.txt");
    let ret = count(input);
    assert_eq!(2, ret);
}

#[test]
fn test_part2() {
    let input = include_str!("../include/day2/sample.txt");
    let ret = part2(input);
    assert_eq!(4, ret);
}
