use rustc_hash::FxHashSet;

#[derive(Debug)]
struct Id {
    start: u64,
    end: u64,
}

fn parse_input(input: &str) -> Vec<Id> {
    input
        .split(',')
        .map(|r| {
            let (start, end) = r.split_once('-').expect("Invalid range");
            Id {
                start: start.parse().expect("integer"),
                end: end.trim().parse().expect("integer"),
            }
        })
        .collect()
}

fn split_range(mut start: u64, end: u64) -> Vec<(u64, u64)> {
    let mut rs: Vec<(u64, u64)> = vec![];

    while start <= end {
        if start.ilog10() < end.ilog10() {
            let new_end = 10_u64.pow(start.ilog10() + 1) - 1;
            rs.push((start, new_end));
            start = new_end + 1;
        } else {
            rs.push((start, end));
            break;
        }
    }

    rs
}

pub fn day2(input: &str) -> (u64, u64) {
    let mut p1: FxHashSet<u64> = FxHashSet::default();
    let mut p2: FxHashSet<u64> = FxHashSet::default();

    let ids = parse_input(input);

    for id in ids {
        for (start, end) in split_range(id.start, id.end) {
            let len: u32 = start.ilog10();

            for i in 0..=(len / 2) {
                let mut n: u64 = 1;
                let div = 10_u64.pow(len - i);

                let start_pfx = start / div;
                let end_pfx = end / div;

                'outer: for exp in 1.. {
                    n += 10_u64.pow(exp).pow(i + 1);

                    for v in start_pfx..=end_pfx {
                        let product = n * v;

                        if product > end {
                            break 'outer;
                        }

                        if product >= start {
                            //part1
                            if i == len / 2 {
                                p1.insert(product);
                            }

                            p2.insert(product);
                        }
                    }
                }
            }
        }
    }

    (p1.into_iter().sum(), p2.into_iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_range() {
        let expected = vec![(1, 9), (10, 10)];
        let result = split_range(1, 10);
        assert_eq!(expected, result);

        let expected = vec![(1, 9), (10, 99), (100, 200)];
        let result = split_range(1, 200);
        assert_eq!(expected, result);

        let expected = vec![(10, 99), (100, 999), (1000, 9999), (10000, 10000)];
        let result = split_range(10, 10000);
        assert_eq!(expected, result);

        let expected = vec![(3, 9), (10, 22)];
        let result = split_range(3, 22);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_day2() {
        let e1 = 1227775554;
        let e2 = 4174379265;
        let input: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        let (r1, r2) = day2(input);
        assert_eq!(r1, e1);
        assert_eq!(r2, e2);
    }
}
