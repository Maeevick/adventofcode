pub fn part1() -> &'static str {
    "This is day one, part one"
}

pub fn part2() -> &'static str {
    "This is day one, part two"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1solution() {
        assert_eq!(part1(), "This is day one, part one");
    }

    #[test]
    fn part2solution() {
        assert_eq!(part2(), "This is day one, part two");
    }
}
