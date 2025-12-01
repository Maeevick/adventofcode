#[rustfmt::skip]
const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
     (0, -1),          (0, 1),
     (1, -1),  (1, 0), (1, 1)
];

pub fn part1(content: Vec<String>) -> i32 {
    let h = content.len() as i32;
    let w = content[0].len() as i32;

    let is_inbound = |x: i32, y: i32| x >= 0 && x < h && y >= 0 && y < w;

    let check_xmas = |x: i32, y: i32, h_dir: i32, v_dir: i32| -> bool {
        (0..4)
            .map(|i| (x + i * h_dir, y + i * v_dir))
            .collect::<Vec<(i32, i32)>>()
            .iter()
            .enumerate()
            .all(|(i, &(x, y))| {
                is_inbound(x, y)
                    && content[x as usize].chars().nth(y as usize) == "XMAS".chars().nth(i)
            })
    };

    content
        .iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.chars().enumerate().filter_map(move |(y, c)| {
                if c == 'X' {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .flat_map(|(x, y)| {
            DIRECTIONS
                .iter()
                .filter(move |&&(h_dir, v_dir)| check_xmas(x, y, h_dir, v_dir))
        })
        .count() as i32
}

const PATTERNS: [[char; 4]; 4] = [
    ['M', 'M', 'S', 'S'],
    ['S', 'S', 'M', 'M'],
    ['M', 'S', 'M', 'S'],
    ['S', 'M', 'S', 'M'],
];

pub fn part2(content: Vec<String>) -> i32 {
    let h = content.len();
    let w = content[0].len();

    let get_symbols_around = |x: usize, y: usize| -> Option<[char; 4]> {
        if x == 0 || y == 0 || x >= h - 1 || y >= w - 1 {
            return None;
        }

        Some([
            content[x - 1].chars().nth(y - 1).expect("Invalid top-left"),
            content[x - 1]
                .chars()
                .nth(y + 1)
                .expect("Invalid top-right"),
            content[x + 1]
                .chars()
                .nth(y - 1)
                .expect("Invalid bottom-left"),
            content[x + 1]
                .chars()
                .nth(y + 1)
                .expect("Invalid bottom-right"),
        ])
    };

    let check_patterns = |symbols_around: [char; 4]| -> bool {
        PATTERNS.iter().any(|pattern| pattern == &symbols_around)
    };

    content
        .iter()
        .enumerate()
        .skip(1)
        .take(h - 2)
        .flat_map(|(x, row)| {
            row.chars()
                .enumerate()
                .skip(1)
                .take(w - 2)
                .filter_map(move |(y, c)| {
                    if c == 'A' {
                        get_symbols_around(x, y)
                            .map(|symbols_around| check_patterns(symbols_around))
                    } else {
                        None
                    }
                })
        })
        .filter(|&valid| valid)
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader;

    #[test]
    fn part1solution() {
        assert_eq!(part1(reader::read_input("day4sample")), 18);
    }

    #[test]
    fn part2solution() {
        assert_eq!(part2(reader::read_input("day4sample")), 9);
    }
}
