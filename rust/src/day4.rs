fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>()
}

const ADJACENT: [(i64, i64); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn count_adjacent(grid: &[Vec<char>], pos: (usize, usize)) -> u8 {
    let mut count = 0;
    let px = pos.0 as i64;
    let py = pos.1 as i64;

    let max_x = grid[0].len() as i64;
    let max_y = grid.len() as i64;

    for (nx, ny) in ADJACENT {
        let xx = px + nx;
        let yy = py + ny;

        if xx >= 0 && xx < max_x && yy >= 0 && yy < max_y && grid[xx as usize][yy as usize] == '@' {
            count += 1;
        }
    }
    count
}

fn pass(grid: &mut [Vec<char>], mark: bool) -> u64 {
    let mut rolls: u64 = 0;

    for x in 0..grid[0].len() {
        for y in 0..grid.len() {
            if grid[x][y] == '@' && count_adjacent(grid, (x, y)) < 4 {
                rolls += 1;
                if mark {
                    grid[x][y] = 'x';
                }
            }
        }
    }
    rolls
}

pub fn part1(input: &str) -> u64 {
    let mut grid = parse_input(input);
    pass(&mut grid, false)
}

pub fn part2(input: &str) -> u64 {
    let mut grid = parse_input(input);
    let mut total_rolls: u64 = 0;

    loop {
        let rolls = pass(&mut grid, true);
        total_rolls += rolls;

        if rolls == 0 {
            break;
        }
    }
    total_rolls
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = 13;
        let input: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        let result = part1(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_part2() {
        let expected = 43;
        let input: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        let result = part2(input);
        assert_eq!(expected, result);
    }
}
