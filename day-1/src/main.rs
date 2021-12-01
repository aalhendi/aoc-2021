use std::fs;

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Unable to read file");
    let numbers_int: Vec<u16> = data.lines().map(|e| e.parse::<u16>().unwrap()).collect();
    let mut counter = 0;
    let mut prev = numbers_int[0];
    for i in 1..numbers_int.len() {
        if numbers_int[i] > prev {
            counter += 1;
        }
        prev = numbers_int[i];
    }
    println!("Part One: {}", counter);

    // Part 2 - sliding window
    // A+B+C > B+C+D ---> A > D ? aka offset by 2
    prev = numbers_int[0];
    counter = 0;
    for i in 3..numbers_int.len() {
        if numbers_int[i] > prev {
            counter += 1
        }
        prev = numbers_int[i - 2]
    }
    println!("Part Two: {}", counter)
}
