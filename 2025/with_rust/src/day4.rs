pub fn part1(content: Vec<String>) -> usize {
    fn count_adjacent_rolls(grid: &[Vec<char>], row: usize, col: usize) -> usize {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .filter(|(dr, dc)| {
            grid.get((row as isize + dr) as usize)
                .and_then(|r| r.get((col as isize + dc) as usize))
                .map_or(false, |&cell| cell == '@')
        })
        .count()
    }

    let grid: Vec<Vec<char>> = content.iter().map(|line| line.chars().collect()).collect();

    grid.iter()
        .enumerate()
        .map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter(|(col, cell)| **cell == '@' && count_adjacent_rolls(&grid, row, *col) < 4)
                .count()
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
        assert_eq!(part1(reader::read_input("day4sample")), 13);
    }

    #[test]
    fn part2solution() {
        assert_eq!(part2(reader::read_input("day4sample")), 666);
    }
}
