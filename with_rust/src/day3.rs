pub fn part1(content: Vec<String>) -> i32 {
    0
}

pub fn part2(content: Vec<String>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader;

    #[test]
    fn part1solution() {
        assert_eq!(part1(reader::read_input("day3sample")), 161);
    }

    #[test]
    fn part2solution() {
        assert_eq!(part2(reader::read_input("day3sample")), -1);
    }
}
