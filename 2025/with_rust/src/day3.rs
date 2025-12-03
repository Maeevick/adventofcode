pub fn part1(content: Vec<String>) -> u64 {
    42
}

pub fn part2(content: Vec<String>) -> u64 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader;

    #[test]
    fn part1solution() {
        assert_eq!(part1(reader::read_input("day3sample")), 666);
    }

    #[test]
    fn part2solution() {
        assert_eq!(part2(reader::read_input("day3sample")), 666);
    }
}
