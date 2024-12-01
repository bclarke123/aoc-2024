fn main() {
    let input = include_str!("../include/day1/input.txt");
    let mut lists: (Vec<u32>, Vec<u32>) = (vec![], vec![]);
    let lines = input.lines();
    lines.for_each(|line| {
        let words = line.split_whitespace().collect::<Vec<_>>();
        lists.0.push(words.first().unwrap().parse::<u32>().unwrap());
        lists.1.push(words.last().unwrap().parse::<u32>().unwrap());
    });

    lists.0.sort();
    lists.1.sort();

    let mut total = 0;
    for i in 0..lists.0.len() {
        let item_a = lists.0.get(i).unwrap();
        let item_b = lists.1.get(i).unwrap();
        total += item_a.abs_diff(*item_b);
    }

    println!("{total}");
}
