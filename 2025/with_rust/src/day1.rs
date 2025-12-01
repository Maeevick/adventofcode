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

pub fn part2(content: Vec<String>) -> i32 {
    content
        .iter()
        .scan(50, |position, line| {
            let direction = &line[0..1];
            let distance: i32 = line[1..].parse().unwrap();

            let steps_to_zero = match direction {
                "R" => {
                    if *position == 0 {
                        100
                    } else {
                        100 - *position
                    }
                }
                "L" => {
                    if *position == 0 {
                        100
                    } else {
                        *position
                    }
                }
                _ => panic!("Invalid direction"),
            };

            *position = match direction {
                "L" => (*position - distance).rem_euclid(100),
                "R" => (*position + distance) % 100,
                _ => panic!("Invalid direction"),
            };

            if distance >= steps_to_zero {
                Some(1 + ((distance - steps_to_zero) / 100))
            } else {
                Some(0)
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader;

    #[test]
    fn part1solution() {
        assert_eq!(part1(reader::read_input("day1sample")), 3);
    }

    #[test]
    fn part2solution() {
        assert_eq!(part2(reader::read_input("day1sample")), 6);
    }
}
