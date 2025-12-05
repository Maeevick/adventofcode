pub fn part1(content: Vec<String>) -> usize {
    let parts: Vec<_> = content.split(|line| line.is_empty()).collect();
    parts[1]
        .iter()
        .map(|item| item.parse::<i64>().unwrap())
        .filter(|&item| {
            parts[0]
                .iter()
                .map(|range| range.split_once("-").unwrap())
                .any(|(start, end)| {
                    item >= start.parse::<i64>().unwrap() && item <= end.parse::<i64>().unwrap()
                })
        })
        .count()
}

pub fn part2(content: Vec<String>) -> usize {
    42
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader;

    #[test]
    fn part1solution() {
        assert_eq!(part1(reader::read_input("day5sample")), 3);
    }

    #[test]
    fn part2solution() {
        assert_eq!(part2(reader::read_input("day5sample")), 666);
    }
}
