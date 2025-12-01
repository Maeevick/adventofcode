mod day1;
mod hello;
mod reader;

fn main() {
    println!("----------------");
    let hello = hello::hello();
    println!("\n{hello}\n\n");
    println!("----------------");
    let day1part1 = day1::part1(reader::read_input("day1").clone());
    println!("D1P1: {day1part1}");
    println!("----------------");
}
