pub fn part1(content: Vec<String>) -> usize {
    let grid: Vec<Vec<char>> = generate_grid(content);

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

pub fn part2(content: Vec<String>) -> usize {
    fn count_then_remove(mut grid: Vec<Vec<char>>) -> usize {
        let accessible: Vec<_> = (0..grid.len())
            .flat_map(|row| (0..grid[row].len()).map(move |col| (row, col)))
            .filter(|&(row, col)| {
                grid[row][col] == '@' && count_adjacent_rolls(&grid, row, col) < 4
            })
            .collect();
        if accessible.is_empty() {
            return 0;
        }
        for (row, col) in &accessible {
            grid[*row][*col] = 'x';
        }
        accessible.len() + count_then_remove(grid)
    }

    count_then_remove(generate_grid(content))
}

fn generate_grid(content: Vec<String>) -> Vec<Vec<char>> {
    content.iter().map(|line| line.chars().collect()).collect()
}

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
        assert_eq!(part2(reader::read_input("day4sample")), 43);
    }
}
