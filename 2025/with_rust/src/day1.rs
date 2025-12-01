pub fn part1(content: Vec<String>) -> i32 {
    content
        .iter()
        .scan(50, |position, line| {
            let direction = &line[0..1];
            let distance: i32 = line[1..].parse().unwrap();
            
            *position = match direction {
                "L" => (*position - distance).rem_euclid(100),
                "R" => (*position + distance) % 100,
                _ => panic!("Invalid direction: {}", direction),
            };
            
            Some(*position)
        })
        .filter(|&pos| pos == 0)
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader;

    #[test]
    fn part1solution() {
        assert_eq!(part1(reader::read_input("day1sample")), 3);
    }
}
