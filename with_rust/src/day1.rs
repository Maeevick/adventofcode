use itertools::Itertools;

pub fn part1(content: Vec<String>) -> i32 {
    let (lefts, rights): (Vec<i32>, Vec<i32>) = from_strings_to_pairs(&content);

    lefts
        .into_iter()
        .sorted()
        .zip(rights.into_iter().sorted())
        .map(|(left, right)| (left - right).abs())
        .sum()
}

pub fn part2(content: Vec<String>) -> i32 {
    let (lefts, rights): (Vec<i32>, Vec<i32>) = from_strings_to_pairs(&content);
    let right_occurrences: std::collections::HashMap<i32, i32> = rights
        .into_iter()
        .counts()
        .into_iter()
        .map(|(k, v)| {
            (
                k,
                v.try_into()
                    .expect("Count should not be greater than i32::MAX"),
            )
        })
        .collect();

    lefts
        .iter()
        .map(|left| left * right_occurrences.get(left).unwrap_or(&0))
        .sum()
}

fn from_strings_to_pairs(content: &[String]) -> (Vec<i32>, Vec<i32>) {
    content
        .iter()
        .map(|line| {
            let nums: Vec<i32> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            (nums[0], nums[1])
        })
        .collect::<Vec<(i32, i32)>>()
        .into_iter()
        .unzip()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader;

    #[test]
    fn part1solution() {
        assert_eq!(part1(reader::read_input("day1part1sample")), 11);
    }

    #[test]
    fn part2solution() {
        assert_eq!(part2(reader::read_input("day1part1sample")), 31);
    }
}
