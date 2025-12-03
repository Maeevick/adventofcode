pub fn part1(content: Vec<String>) -> u32 {
    content
        .iter()
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();

            let (fst, fst_idx) = line.chars().enumerate().take(chars.len() - 1).fold(
                (0, 0),
                |(fst, idx), (pos, c)| {
                    let digit = c.to_digit(10).unwrap();
                    if digit > fst {
                        (digit, pos)
                    } else {
                        (fst, idx)
                    }
                },
            );

            let snd = line
                .chars()
                .enumerate()
                .skip(fst_idx + 1)
                .fold(0, |snd, (_, c)| {
                    let digit = c.to_digit(10).unwrap();
                    snd.max(digit)
                });

            fst * 10 + snd
        })
        .sum()
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
        assert_eq!(part1(reader::read_input("day3sample")), 357);
    }

    #[test]
    fn part2solution() {
        assert_eq!(part2(reader::read_input("day3sample")), 666);
    }
}
