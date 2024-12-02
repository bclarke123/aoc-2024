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

fn is_safe(levels: &[i32], skip: Option<usize>) -> bool {
    let mut sign = None;

    for i in 0..levels.len() - 1 {
        let mut inc = 1;

        match skip {
            Some(x) if x == i => {
                continue;
            }
            Some(x) if x == i + 1 => {
                inc = 2;
            }
            _ => {}
        }

        if i + inc >= levels.len() {
            break;
        }

        let prev = levels[i];
        let next = levels[i + inc];

        let d = prev.abs_diff(next);
        let s = (prev - next).signum();

        let out_range = d < 1 || d > 3;
        let bad_sign = match sign {
            Some(x) => x != s,
            None => false,
        };

        sign = Some(s);

        if out_range || bad_sign {
            return false;
        }
    }

    true
}

fn part1(input: &str) -> usize {
    parse_input(input)
        .iter()
        .map(|report| is_safe(report, None))
        .filter(|&result| result)
        .count()
}

fn part2(input: &str) -> usize {
    let reports = parse_input(input);
    let mut total = 0;
    for report in reports {
        for i in 0..report.len() {
            if is_safe(&report, Some(i)) {
                total += 1;
                break;
            }
        }
    }

    total
}

fn main() {
    let safe = part1(INPUT);
    println!("Part 1: {safe} reports are safe");

    let safe = part2(INPUT);
    println!("Part 2: {safe} reports are safe");
}

#[test]
fn test_part1() {
    let input = include_str!("../include/day2/sample.txt");
    let ret = part1(input);
    assert_eq!(2, ret);
}

#[test]
fn test_part1_full() {
    let ret = part1(INPUT);
    assert_eq!(502, ret);
}

#[test]
fn test_part2() {
    let input = include_str!("../include/day2/sample.txt");
    let ret = part2(input);
    assert_eq!(4, ret);
}

#[test]
fn test_part2_full() {
    let ret = part2(INPUT);
    assert_eq!(544, ret);
}
