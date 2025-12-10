fn parse_input(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .map(|l| {
            let (a, b) = l.trim().split_once(',').expect("pair");
            (
                a.parse::<i64>().expect("integer"),
                b.parse::<i64>().expect("integer"),
            )
        })
        .collect()
}

fn area(p1: (i64, i64), p2: (i64, i64)) -> i64 {
    (1 + (p1.0 - p2.0).abs()) * (1 + (p1.1 - p2.1).abs())
}

pub fn part1(input: &str) -> i64 {
    let pts = parse_input(input);
    let mut max: i64 = 0;

    for a in 0..pts.len() {
        for b in a + 1..pts.len() {
            max = max.max(area(pts[a], pts[b]));
        }
    }
    max
}

pub fn part2(input: &str) -> i64 {
    let pts = parse_input(input);
    let mut result: i64 = 0;

    let mut edges: Vec<(i64, i64, i64, i64)> = vec![];
    for i in 0..pts.len() - 1 {
        let (ax, ay) = pts[i];
        let (bx, by) = pts[i + 1];

        edges.push((ax.min(bx), ay.min(by), ax.max(bx), ay.max(by)));
    }

    let (ax, ay) = pts[pts.len() - 1];
    let (bx, by) = pts[0];
    edges.push((ax.min(bx), ay.min(by), ax.max(bx), ay.max(by)));

    for i in 0..pts.len() {
        'inner: for j in i..pts.len() {
            let c1_x = pts[i].0.min(pts[j].0);
            let c1_y = pts[i].1.min(pts[j].1);

            let c2_x = pts[i].0.max(pts[j].0);
            let c2_y = pts[i].1.max(pts[j].1);

            for e in &edges {
                let (ax, ay, bx, by) = e;
                if *bx > c1_x && *ax < c2_x && *by > c1_y && *ay < c2_y {
                    continue 'inner;
                }
            }

            result = result.max(area((c2_x, c2_y), (c1_x, c1_y)));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_part1() {
        let expected = 50;
        let result = part1(INPUT);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_part2() {
        let expected = 24;
        let result = part2(INPUT);
        assert_eq!(expected, result);
    }
}
