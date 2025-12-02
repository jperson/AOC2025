#[derive(Debug)]
struct Id {
    start: String,
    end: String,
}

fn parse_input(input: &str) -> Vec<Id> {
    input
        .split(',')
        .map(|r| {
            let (start, end) = r.split_once('-').expect("Invalid range");
            Id {
                start: String::from(start),
                end: String::from(end.trim()),
            }
        })
        .collect()
}

fn check_invalid_part1(id: String) -> bool {
    let id_len = id.len() / 2;
    id[..id_len] == id[id_len..]
}

fn sum_invalid_ids_part1(ids: Vec<Id>) -> u64 {
    let mut sum = 0;

    for id in ids {
        let start: usize = id.start.parse().expect("integer");
        let end: usize = id.end.parse().expect("integer");

        for v in start..=end {
            if check_invalid_part1(v.to_string()) {
                let v: u64 = v as u64;
                sum += v;
            }
        }
    }
    sum
}

pub fn part1(input: &str) -> u64 {
    let ids = parse_input(input);
    let sum = sum_invalid_ids_part1(ids);
    sum
}

fn check_invalid_part2(id: String) -> bool {
    let half_width = id.len() / 2;
    let idv: Vec<_> = id.chars().collect();

    for n in 1..=half_width {
        let chunks: Vec<_> = idv.chunks(n).collect();
        if chunks.iter().all(|&v| v == chunks[0]) {
            return true;
        }
    }
    false
}

fn sum_invalid_ids_part2(ids: Vec<Id>) -> u64 {
    let mut sum = 0;
    for id in ids {
        let start: usize = id.start.parse().expect("integer");
        let end: usize = id.end.parse().expect("integer");

        for v in start..=end {
            if check_invalid_part2(v.to_string()) {
                let v: u64 = v as u64;
                sum += v;
            }
        }
    }
    sum
}

pub fn part2(input: &str) -> u64 {
    let ids = parse_input(input);
    let sum = sum_invalid_ids_part2(ids);
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = 1227775554;
        let input: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let result = part1(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2() {
        let expected = 4174379265;
        let input: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let result = part2(input);
        assert_eq!(result, expected);
    }
}
