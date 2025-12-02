mod day1;
mod day2;
mod hello;
mod reader;

use std::time::Instant;

fn main() {
    println!("----------------");
    let hello = hello::hello();
    println!("\n{hello}\n\n");
    println!("----------------");

    let start = Instant::now();
    let day1part1 = day1::part1(reader::read_input("day1").clone());
    println!("D1P1: {day1part1} ({}ms)", start.elapsed().as_millis());

    let start = Instant::now();
    let day1part2 = day1::part2(reader::read_input("day1").clone());
    println!("D1P2: {day1part2} ({}ms)", start.elapsed().as_millis());
    println!("----------------");

    let start = Instant::now();
    let day2part1 = day2::part1(reader::read_input("day2").clone());
    println!("D2P1: {day2part1} ({}ms)", start.elapsed().as_millis());

    let start = Instant::now();
    let day2part2 = day2::part2(reader::read_input("day2").clone());
    println!("D2P2: {day2part2} ({}ms)", start.elapsed().as_millis());
    println!("----------------");
}
