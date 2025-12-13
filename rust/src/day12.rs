use cached::proc_macro::cached;

type Shape = [[bool; 3]; 3];

#[derive(Debug, Clone, Copy)]
struct ShapeId {
    id: u64,
    shape: Shape,
}

#[derive(Debug)]
struct Region {
    dim: (usize, usize),
    quantity: [usize; 6],
}

fn parse_input(input: &str) -> (Vec<ShapeId>, Vec<Region>) {
    let mut shapes: Vec<ShapeId> = vec![];
    let result: Vec<_> = input.split("\n\n").collect();

    for (i, id) in result[..=5].iter().enumerate() {
        let mut shape = id.split("\n");
        let _ = shape.next().unwrap();

        let row1: [bool; 3] = shape
            .next()
            .unwrap()
            .chars()
            .map(|c| c == '#')
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let row2: [bool; 3] = shape
            .next()
            .unwrap()
            .chars()
            .map(|c| c == '#')
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let row3: [bool; 3] = shape
            .next()
            .unwrap()
            .chars()
            .map(|c| c == '#')
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        shapes.push(ShapeId {
            id: i as u64,
            shape: [row1, row2, row3],
        });
    }

    let mut regions: Vec<Region> = vec![];
    let rgns = result[6];

    for rs in rgns.trim().split("\n") {
        let (dim, rest) = rs.split_once(": ").unwrap();
        let (x, y) = dim.split_once("x").unwrap();
        let x = x.parse::<usize>().expect("integer");
        let y = y.parse::<usize>().expect("integer");

        let quantity = rest
            .split(" ")
            .map(|q| q.parse::<usize>().expect("integer"))
            .collect::<Vec<usize>>();

        regions.push(Region {
            dim: (x, y),
            quantity: quantity[..].try_into().unwrap(),
        });
    }

    (shapes, regions)
}

fn rotate(grid: &Shape, count: usize) -> Shape {
    let mut result: [[bool; 3]; 3] = [[false; 3]; 3];
    for _ in 0..count {
        for (y, row) in grid.iter().enumerate() {
            for (x, &v) in row.iter().enumerate() {
                result[x][row.len() - y - 1] = v;
            }
        }
    }
    result
}

fn flip(grid: &Shape) -> Shape {
    let mut result: [[bool; 3]; 3] = [[false; 3]; 3];
    for (y, row) in grid.iter().enumerate() {
        for (x, &v) in row.iter().enumerate() {
            result[grid.len() - y - 1][x] = v;
        }
    }
    result
}

fn check(grid: &[Vec<bool>], s: Shape, loc: (usize, usize)) -> bool {
    let xlen = grid[0].len();
    let ylen = grid.len();
    let (lx, ly) = loc;

    if lx < 1 || ly < 1 || lx > (xlen - 2) || ly > (ylen - 2) {
        return false;
    }

    for y in 0..=2 {
        for x in 0..=2 {
            if grid[y + loc.1 - 1][x + loc.0 - 1] && s[y][x] {
                return false;
            }
        }
    }
    true
}

fn place(grid: &mut [Vec<bool>], s: Shape, loc: (usize, usize)) {
    for (xi, x) in (loc.0 - 1..=loc.0 + 1).enumerate() {
        for (yi, y) in (loc.1 - 1..=loc.1 + 1).enumerate() {
            grid[y][x] = grid[y][x] || s[yi][xi];
        }
    }
}

#[cached(
    key = "(Vec<Vec<bool>>, [usize; 6])",
    convert = r#"{ (grid.to_vec(), *count) }"#
)]
fn dfs(grid: &[Vec<bool>], shapes: &[ShapeId; 6], count: &[usize; 6]) -> bool {
    if count.iter().sum::<usize>() == 0 {
        return true;
    }

    let mut remaining: usize = 0;
    for row in grid {
        for v in row {
            if !v {
                remaining += 1;
            }
        }
    }

    if remaining < count.iter().sum::<usize>() * 7 {
        return false;
    }

    for cur in shapes {
        if count[cur.id as usize] == 0 {
            continue;
        }

        let orientations = (0..=3)
            .flat_map(|i| [rotate(&cur.shape, i), flip(&cur.shape)])
            .collect::<Vec<_>>();

        for y in 1..grid.len() - 1 {
            for x in 1..grid[0].len() - 1 {
                for o in &orientations {
                    if check(grid, *o, (x, y)) {
                        let mut ng = grid.to_owned();

                        place(&mut ng, *o, (x, y));

                        let mut new_count = *count;
                        new_count[cur.id as usize] -= 1;

                        if dfs(&ng, shapes, &new_count) {
                            return true;
                        }
                    }
                }
            }
        }
        return false;
    }

    false
}

pub fn part1(input: &str) -> u64 {
    let (shape_ids, regions) = parse_input(input);
    let mut result: u64 = 0;

    for r in regions {
        if r.dim.0 * r.dim.1 >= r.quantity.iter().sum::<usize>() * 9 {
            result += 1;
        } else {
            let grid: Vec<Vec<bool>> = vec![vec![false; r.dim.0]; r.dim.1];
            if dfs(&grid, &shape_ids[..].try_into().unwrap(), &r.quantity) {
                result += 1;
            }
        }
    }

    result
}

pub fn part2(input: &str) -> u64 {
    let _ = parse_input(input);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[test]
    fn test_part1() {
        let expected = 2;
        let result = part1(INPUT);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_part2() {
        let expected: u64 = 0;
        let result = part2(INPUT);
        assert_eq!(expected, result);
    }
}
