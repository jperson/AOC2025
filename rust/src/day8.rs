use std::collections::HashSet;

#[derive(Debug)]
struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
    count: usize,
}

impl DisjointSet {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![1; n],
            size: vec![1; n],
            count: n,
        }
    }

    pub fn find(&mut self, n: usize) -> usize {
        if self.parent[n] == n {
            return n;
        }

        self.find(self.parent[n])
    }

    pub fn union(&mut self, a: usize, b: usize) {
        let root_a = self.find(a);
        let root_b = self.find(b);

        if root_a == root_b {
            return;
        }

        if self.rank[root_a] < self.rank[root_b] {
            self.parent[root_a] = root_b;
            self.size[root_b] += self.size[root_a];
        } else if self.rank[root_b] < self.rank[root_a] {
            self.parent[root_b] = root_a;
            self.size[root_a] += self.size[root_b];
        } else {
            self.parent[root_b] = root_a;
            self.rank[root_a] += 1;
            self.size[root_a] += self.size[root_b];
        }

        self.count -= 1;
    }
}

fn parse_input(input: &str) -> Vec<Vec<f64>> {
    let result: Vec<Vec<f64>> = input
        .lines()
        .map(|l| {
            l.trim()
                .split(",")
                .map(|n| n.parse::<f64>().expect("Integer"))
                .collect::<Vec<f64>>()
        })
        .collect();
    result
}

fn all_dist(boxes: &[Vec<f64>]) -> Vec<(usize, usize, f64)> {
    let mut edges: Vec<(usize, usize, f64)> = vec![];
    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    for x in 0..boxes.len() {
        for y in 0..boxes.len() {
            if x == y || seen.contains(&(x.min(y), x.max(y))) {
                continue;
            }

            let a: &[f64] = &boxes[x][..];
            let b: &[f64] = &boxes[y][..];
            let d = (a[0] - b[0]).powf(2.) + (a[1] - b[1]).powf(2.) + (a[2] - b[2]).powf(2.);
            let d = d.sqrt();

            let d = (x.min(y), x.max(y), d);
            seen.insert((d.0, d.1));
            edges.push(d);
        }
    }

    edges.sort_by(|a, b| a.2.total_cmp(&b.2));
    edges
}

pub fn part1(input: &str) -> u64 {
    let boxes = parse_input(input);
    let edges = all_dist(&boxes);

    let mut dj = DisjointSet::new(1000);

    for e in &edges[..1000] {
        dj.union(e.0, e.1);
    }

    dj.size.sort();
    dj.size.reverse();
    dj.size[..3].iter().product::<usize>() as u64
}

pub fn part2(input: &str) -> u64 {
    let boxes = parse_input(input);
    let edges = all_dist(&boxes);

    let mut dj = DisjointSet::new(boxes.len());

    let mut last: (usize, usize, f64) = Default::default();

    for e in &edges {
        dj.union(e.0, e.1);
        if dj.count == 1 {
            last = *e;
            break;
        }
    }

    (boxes[last.0][0] * boxes[last.1][0]) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_part1() {
        let expected = 40;
        let result = part1(INPUT);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_part2() {
        let expected = 25272;
        let result = part2(INPUT);
        assert_eq!(expected, result);
    }
}
