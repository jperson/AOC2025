fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|l| {
            l.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>()
}

pub fn day3(input: &str, digits: usize) -> u64 {
    parse_input(input).into_iter().fold(0, |mut joltage, b| {
        let mut bi: usize = 0;

        for d in 0..digits {
            let mut bv: u64 = 0;
            let window = (b.len() - bi) - (digits - d);

            for i in bi..=(bi + window) {
                if b[i] > bv {
                    (bv, bi) = (b[i], i);
                }
            }

            bi += 1;
            joltage += 10u64.pow((digits - d - 1) as u32) * bv
        }
        joltage
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = 357;
        let input: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

        let result = day3(input, 2);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_part2() {
        let expected = 3121910778619;
        let input: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

        let result = day3(input, 12);
        assert_eq!(expected, result);
    }
}
