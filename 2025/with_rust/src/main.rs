mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
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

    let start = Instant::now();
    let day3part1 = day3::part1(reader::read_input("day3").clone());
    println!("D3P1: {day3part1} ({}ms)", start.elapsed().as_millis());

    let start = Instant::now();
    let day3part2 = day3::part2(reader::read_input("day3").clone());
    println!("D3P2: {day3part2} ({}ms)", start.elapsed().as_millis());
    println!("----------------");

    let start = Instant::now();
    let day4part1 = day4::part1(reader::read_input("day4").clone());
    println!("D4P1: {day4part1} ({}ms)", start.elapsed().as_millis());

    let start = Instant::now();
    let day4part2 = day4::part2(reader::read_input("day4").clone());
    println!("D4P2: {day4part2} ({}ms)", start.elapsed().as_millis());
    println!("----------------");

    let start = Instant::now();
    let day5part1 = day5::part1(reader::read_input("day5").clone());
    println!("D5P1: {day5part1} ({}ms)", start.elapsed().as_millis());

    let start = Instant::now();
    let day5part2 = day5::part2(reader::read_input("day5").clone());
    println!("D5P2: {day5part2} ({}ms)", start.elapsed().as_millis());
    println!("----------------");
}
