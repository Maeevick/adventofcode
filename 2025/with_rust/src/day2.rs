use fancy_regex::Regex;

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

// ~4200ms -> fancy but slow :D
pub fn _part2_re(content: Vec<String>) -> u64 {
    let re_with_any_pattern_repeated_more_than_once_composing_the_whole_string =
        Regex::new(r"^(.+)\1+$").unwrap();
    content[0]
        .split(',')
        .filter_map(|range| {
            let parts: Vec<&str> = range.split('-').collect();
            let start = parts[0].parse::<u64>().ok()?;
            let end = parts[1].parse::<u64>().ok()?;
            Some(
                (start..=end)
                    .filter(|&num| {
                        re_with_any_pattern_repeated_more_than_once_composing_the_whole_string
                            .is_match(&num.to_string())
                            .expect("Doesn't match the fancy regexp with backtracking")
                    })
                    .sum::<u64>(),
            )
        })
        .sum()
}

// ~900ms -> just slow ;)
pub fn part2(content: Vec<String>) -> u64 {
    /// Check if repeatable pattern equals original number representation
    /// ## Arguments
    /// - s: &str: the number representation
    /// ## Examples:
    /// - s = "123123", len = 3: "123".repeat(6/3) = "123123" ✅
    /// - s = "123456", len = 3: "123".repeat(6/2) = "123123" != "123456" ❌
    fn is_repeating_pattern(s: &str) -> bool {
        (1..=s.len() / 2).any(|len| s == s[0..len].repeat(s.len() / len))
    }

    content[0]
        .split(',')
        .filter_map(|range| {
            let parts: Vec<&str> = range.split('-').collect();
            let start = parts[0].parse::<u64>().ok()?;
            let end = parts[1].parse::<u64>().ok()?;
            Some(
                (start..=end)
                    .filter(|&num| is_repeating_pattern(&num.to_string()))
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

    #[test]
    fn part2solution() {
        assert_eq!(_part2_re(reader::read_input("day2sample")), 4174379265);
        assert_eq!(part2(reader::read_input("day2sample")), 4174379265);
    }
}
