mod day1;
mod day2;
mod day3;
mod day4;
mod hello;
mod reader;

fn main() {
    println!("----------------");
    let hello = hello::hello();
    println!("\n{hello}\n\n");
    println!("----------------");
    let day1part1 = day1::part1(reader::read_input("day1").clone());
    println!("D1P1: {day1part1}");
    let day1part2 = day1::part2(reader::read_input("day1").clone());
    println!("D1P2: {day1part2}");
    println!("----------------");
    let day2part1 = day2::part1(reader::read_input("day2").clone());
    println!("D2P1: {day2part1}");
    let day2part2 = day2::part2(reader::read_input("day2").clone());
    println!("D2P2: {day2part2}");
    println!("----------------");
    let day3part1 = day3::part1(reader::read_input("day3").clone());
    println!("D3P1: {day3part1}");
    let day3part2 = day3::part2(reader::read_input("day3").clone());
    println!("D3P2: {day3part2}");
    println!("----------------");
    let day4part1 = day4::part1(reader::read_input("day4").clone());
    println!("D4P1: {day4part1}");
    let day4part2 = day4::part2(reader::read_input("day4").clone());
    println!("D4P2: {day4part2}");
    println!("----------------");
}
