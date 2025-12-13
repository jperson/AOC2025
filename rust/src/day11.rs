use cached::proc_macro::cached;
use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> HashMap<&str, HashSet<&str>> {
    input
        .lines()
        .map(|l| {
            let _ = l.trim();
            let (name, values) = l.split_once(": ").unwrap();
            let links: Vec<&str> = values.split(' ').collect::<Vec<&str>>();
            (name, HashSet::from_iter(links))
        })
        .collect()
}

#[cached(
    key = "(String, bool, bool)",
    convert = r#"{ (cur.to_string(), fft, dac) }"#
)]
fn dfs(graph: &HashMap<&str, HashSet<&str>>, cur: &str, fft: bool, dac: bool) -> u64 {
    if cur == "out" {
        return if dac && fft { 1 } else { 0 };
    }

    let mut count: u64 = 0;
    if let Some(edges) = graph.get(cur) {
        for e in edges {
            count += dfs(graph, e, fft || *e == "fft", dac || *e == "dac");
        }
    }

    return count;
}

pub fn part1(input: &str) -> u64 {
    let graph = parse_input(input);
    dfs(&graph, "you", true, true)
}

pub fn part2(input: &str) -> u64 {
    let graph = parse_input(input);
    dfs(&graph, "svr", false, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_1: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    static INPUT_2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn test_part1() {
        let expected = 5;
        let result = part1(INPUT_1);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_part2() {
        let expected: u64 = 2;
        let result = part2(INPUT_2);
        assert_eq!(expected, result);
    }
}
