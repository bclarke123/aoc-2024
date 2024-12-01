const INPUT: &str = include_str!("../include/day1/input.txt");

fn parse_lists(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut lists: (Vec<usize>, Vec<usize>) = (vec![], vec![]);
    let lines = input.lines();
    lines.for_each(|line| {
        let words = line.split_whitespace().collect::<Vec<_>>();
        lists.0.push(words.first().unwrap().parse().unwrap());
        lists.1.push(words.last().unwrap().parse().unwrap());
    });

    lists.0.sort();
    lists.1.sort();

    lists
}

fn part1(input: &str) -> usize {
    let lists = parse_lists(input);
    let mut total = 0;

    for i in 0..lists.0.len() {
        let &item_a = lists.0.get(i).unwrap();
        let &item_b = lists.1.get(i).unwrap();
        total += item_a.abs_diff(item_b);
    }

    total
}

fn part2(input: &str) -> usize {
    let lists = parse_lists(input);
    let mut total = 0;

    for i in 0..lists.0.len() {
        let n = lists.0[i];
        let count = lists.1.iter().filter(|&x| x == &n).count();
        total += n * count;
    }

    total
}

fn main() {
    let ret = part1(INPUT);
    println!("Part 1: {ret}");

    let ret = part2(INPUT);
    println!("Part 2: {ret}");
}

#[test]
fn test_part1() {
    let ret = part1(include_str!("../include/day1/sample.txt"));
    assert_eq!(11, ret);
}

#[test]
fn test_part2() {
    let ret = part2(include_str!("../include/day1/sample.txt"));
    assert_eq!(31, ret);
}
