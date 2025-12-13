#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct Range {
    start: u64,
    end: u64,
}

fn parse_input(input: &str) -> (Vec<Range>, Vec<u64>) {
    let (ranges, ingredients) = input.split_once("\n\n").unwrap();
    let mut range_set = ranges
        .trim()
        .split('\n')
        .map(|r| {
            let (s, e) = r.split_once("-").unwrap();
            let s = s.parse::<u64>().unwrap();
            let e = e.parse::<u64>().unwrap();
            Range { start: s, end: e }
        })
        .collect::<Vec<Range>>();

    let ingredients = ingredients
        .trim()
        .split('\n')
        .map(|i| i.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    range_set.sort_unstable_by_key(|r| (r.start, r.end));
    merge_ranges(&mut range_set);

    (range_set, ingredients)
}

fn merge_ranges(rs: &mut Vec<Range>) {
    let mut i = 1;

    while i < rs.len() {
        if rs[i - 1].end >= rs[i].start {
            rs[i - 1].end = rs[i - 1].end.max(rs[i].end);
            rs.remove(i);
        } else {
            i += 1;
        }
    }
}

pub fn part1(input: &str) -> u64 {
    let mut count = 0;
    let (fresh, ingredients) = parse_input(input);

    for i in ingredients {
        for f in &fresh {
            if i < f.start {
                break;
            }

            if i >= f.start && i <= f.end {
                count += 1;
                break;
            }
        }
    }

    count
}

pub fn part2(input: &str) -> u64 {
    let (fresh, _) = parse_input(input);
    let mut count = 0;

    for f in &fresh {
        count += f.end - f.start + 1;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part1() {
        let expected = 3;
        let result = part1(INPUT);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_part2() {
        let expected = 14;
        let result = part2(INPUT);
        assert_eq!(expected, result);
    }
}
