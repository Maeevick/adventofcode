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
    content
        .iter()
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();

            (1..=12)
                .rev()
                .scan(0, |start, remaining| {
                    let end = chars.len() - remaining + 1;

                    let (best, best_idx) = chars[*start..end].iter().enumerate().fold(
                        (0, *start),
                        |(best, best_idx), (i, &c)| {
                            let digit = c.to_digit(10).unwrap();
                            if digit > best {
                                (digit, *start + i)
                            } else {
                                (best, best_idx)
                            }
                        },
                    );
                    *start = best_idx + 1;

                    Some(char::from_digit(best, 10).unwrap())
                })
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
        })
        .sum()
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
        assert_eq!(part2(reader::read_input("day3sample")), 3121910778619);
    }
}
