use std::fs::read_to_string;

fn main() {
    let entries = read_to_string("inputs/day_1")
        .expect("Error reading input")
        .split_whitespace()
        .map(|e| e.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    println!(
        "{}",
        part_1(&entries).expect("No two entries that add to 2020 found")
    );
    println!(
        "{}",
        part_2(&entries).expect("No thress entries that add to 2020 found")
    );
}

fn part_1(entries: &[usize]) -> Option<usize> {
    for (i, e_1) in entries.iter().enumerate() {
        for e_2 in entries.iter().skip(i) {
            if e_1 + e_2 == 2020 {
                return Some(e_1 * e_2);
            }
        }
    }
    None
}

fn part_2(entries: &[usize]) -> Option<usize> {
    for (i, e_1) in entries.iter().enumerate() {
        for (j, e_2) in entries.iter().enumerate().skip(i) {
            for e_3 in entries.iter().skip(j) {
                if e_1 + e_2 + e_3 == 2020 {
                    return Some(e_1 * e_2 * e_3);
                }
            }
        }
    }
    None
}
