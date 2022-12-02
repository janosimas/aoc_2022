use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut elves: Vec<i32> = vec![];
    for elf in contents.split("\n\n") {
        elves.push(elf.split('\n').map(|c| c.parse::<i32>().unwrap()).sum());
    }
    elves.sort();

    let sum: i32 = elves.iter().rev().take(3).sum();
    dbg!(&sum);
}
