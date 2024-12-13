mod day1;
mod hello;

fn main() {
    let hello = hello::hello();
    println!("\n{hello}\n\n");
    let day1part1 = day1::part1();
    println!("\n{day1part1}\n\n");
    let day1part2 = day1::part2();
    println!("\n{day1part2}\n\n");
}
