pub fn part1(content: Vec<String>) -> u64 {
    content[0]
        .split(',')
        .filter_map(|range| {
            let parts: Vec<&str> = range.split('-').collect();
            let start = parts[0].parse::<u64>().ok()?;
            let end = parts[1].parse::<u64>().ok()?;
            Some(
                (start..=end)
                    .filter(|&num| {
                        let s = num.to_string();
                        let len = s.len();
                        len % 2 == 0 && s[0..len / 2] == s[len / 2..]
                    })
                    .sum::<u64>(),
            )
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader;

    #[test]
    fn part1solution() {
        assert_eq!(part1(reader::read_input("day2sample")), 1227775554);
    }
}
