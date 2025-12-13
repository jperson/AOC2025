#[derive(Debug, Eq, PartialEq)]
enum Dir {
    Left,
    Right,
}

#[derive(Debug)]
struct Rot {
    d: Dir,
    m: i32,
}

fn parse_input(input: &str) -> Vec<Rot> {
    input
        .lines()
        .map(|l| {
            let d = l.chars().next().unwrap();
            let m: i32 = l[1..].parse().unwrap();

            Rot {
                d: if d == 'L' { Dir::Left } else { Dir::Right },
                m,
            }
        })
        .collect()
}

pub fn part1(input: &str) -> i32 {
    let mut pos: i32 = 50;
    let mut count = 0;
    let moves = parse_input(input);

    for m in moves {
        if m.d == Dir::Left {
            pos = (pos - m.m).rem_euclid(100);
        } else {
            pos = (pos + m.m).rem_euclid(100);
        }
        if pos == 0 {
            count += 1;
        }
    }

    count
}

pub fn part2(input: &str) -> i32 {
    let mut pos: i32 = 50;
    let mut count = 0;
    let moves = parse_input(input);

    for m in moves {
        count += m.m / 100;
        let mv = m.m % 100;

        if m.d == Dir::Left {
            let new_pos = (pos - mv).rem_euclid(100);
            if (new_pos > pos || new_pos == 0) && pos != 0 {
                count += 1;
            }
            pos = new_pos;
        } else {
            let new_pos = (pos + mv).rem_euclid(100);

            if (new_pos < pos || new_pos == 0) && pos != 0 {
                count += 1;
            }
            pos = new_pos;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let expected: i32 = 3;
        let input: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

        let result = part1(&input);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_part2() {
        let expected: i32 = 6;
        let input: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

        let result = part2(&input);
        assert_eq!(expected, result);
    }
}
