use regex::Regex;

pub fn part1(content: Vec<String>) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Invalid regex pattern");

    re.captures_iter(&content.join(""))
        .map(|cap| {
            let a = cap[1].parse::<i32>().expect("Failed to parse first number");
            let b = cap[2].parse::<i32>().expect("Failed to parse second number");
            a * b
        })
        .sum()
}

pub fn part2(content: Vec<String>) -> i32 {
    let re = Regex::new(r"(don't|do)\(\)|mul\((\d{1,3}),(\d{1,3})\)").expect("Invalid regex pattern");
    
    re.captures_iter(&content.join(""))
        .fold((true, 0), |(processing, sum), cap| {
            if let Some(instruction) = cap.get(1) {
                (instruction.as_str() == "do", sum)
            } else {
                if processing {
                    let a = cap[2].parse::<i32>().expect("Failed to parse first number");
                    let b = cap[3].parse::<i32>().expect("Failed to parse second number");
                    (processing, sum + a * b)
                } else {
                    (processing, sum)
                }
            }
        })
        .1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader;

    #[test]
    fn part1solution() {
        assert_eq!(part1(reader::read_input("day3part1sample")), 161);
    }

    #[test]
    fn part2solution() {
        assert_eq!(part2(reader::read_input("day3part2sample")), 48);
    }
}
