use cached::proc_macro::cached;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.trim().chars().collect::<Vec<char>>())
        .collect()
}

pub fn part1(input: &str) -> u64 {
    let mut diagram = parse_input(input);
    let mut count: u64 = 0;

    for i in 0..diagram.len() {
        for n in 0..diagram[i].len() {
            if diagram[i][n] == 'S' {
                diagram[i + 1][n] = '|';
            }

            if diagram[i][n] == '^' {
                if diagram[i - 1][n] == '|' {
                    count += 1;
                }
                if n > 0 && n < diagram[i].len() {
                    diagram[i][n - 1] = '|';
                    diagram[i][n + 1] = '|';
                }
            } else if i > 0 && diagram[i - 1][n] == '|' {
                diagram[i][n] = '|';
            }
        }
    }

    count
}

#[cached(key = "(i64, i64)", convert = r#"{ (col, row) }"#)]
fn timeline(d: &[Vec<char>], col: i64, row: i64, mut count: u64) -> u64 {
    if col < 0 || col >= d[0].len() as i64 {
        return 0;
    }

    if row == d.len() as i64 {
        count += 1;
        return count;
    }

    if d[row as usize][col as usize] == '^' {
        return timeline(d, col - 1, row + 1, count) + timeline(d, col + 1, row + 1, count);
    } else {
        return timeline(d, col, row + 1, count);
    }
}

pub fn part2(input: &str) -> u64 {
    let diagram = parse_input(input);
    let mut col: i64 = 0;

    for i in 0..diagram[0].len() {
        if diagram[0][i] == 'S' {
            col = i as i64;
        }
    }

    timeline(&diagram, col, 0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part1() {
        let expected = 21;
        let result = part1(INPUT);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_part2() {
        let expected = 40;
        let result = part2(INPUT);
        assert_eq!(expected, result);
    }
}
