use itertools::Itertools;

pub fn part1(content: Vec<String>) -> usize {
    content
        .iter()
        .filter(|report| is_sorted(report) && has_valid_gaps(report))
        .count()
}

pub fn part2(content: Vec<String>) -> usize {
    content
        .iter()
        .filter(|report| is_sorted_and_valid_gaps_with_one_exception(report))
        .count()
}

fn is_sorted(report: &str) -> bool {
    let numbers: Vec<i32> = from_str_to_numbers(report);

    numbers.is_sorted() || numbers.is_sorted_by(|a, b| b < a)
}

fn has_valid_gaps(report: &str) -> bool {
    let numbers: Vec<i32> = from_str_to_numbers(report);

    numbers.windows(2).all(|pair| {
        let delta = (pair[1] - pair[0]).abs();
        delta >= 1 && delta <= 3
    })
}

fn from_str_to_numbers(report: &str) -> Vec<i32> {
    report.split_whitespace().flat_map(str::parse).collect()
}

fn is_sorted_and_valid_gaps_with_one_exception(report: &str) -> bool {
    if is_sorted(report) && has_valid_gaps(report) {
        return true;
    }
    let numbers: Vec<i32> = from_str_to_numbers(report);
    for i in 0..numbers.len() {
        let mut new_numbers = numbers.clone();
        new_numbers.remove(i);

        if is_sorted(&new_numbers.iter().join(" ")) && has_valid_gaps(&new_numbers.iter().join(" "))
        {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader;

    #[test]
    fn part1solution() {
        assert_eq!(part1(reader::read_input("day2sample")), 2);
    }

    #[test]
    fn part2solution() {
        assert_eq!(part2(reader::read_input("day2sample")), 4);
    }
}
