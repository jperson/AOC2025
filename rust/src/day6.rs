#[derive(Debug)]
enum Op {
    Mul,
    Plus,
    Num(i64),
}

fn parse_input_1(input: &str) -> Vec<Vec<Op>> {
    let ps = input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let len = ps[0].len();
    (0..len)
        .map(|i| {
            ps.iter()
                .map(|v| match v[i] {
                    "*" => Op::Mul,
                    "+" => Op::Plus,
                    n => Op::Num(n.parse::<i64>().unwrap()),
                })
                .collect()
        })
        .collect()
}

fn parse_input_2(input: &str) -> Vec<Vec<Op>> {
    let mut ps = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    //split off operators
    let ops = ps.pop().unwrap();
    let ops: Vec<_> = ops
        .into_iter()
        .filter(|op| *op != ' ')
        .map(|op| String::from(op))
        .collect();

    //transpose num matrix
    let max_len = ps.iter().map(|v| v.len()).max().unwrap_or(0);
    let result: Vec<String> = (0..max_len)
        .map(|i| {
            ps.iter()
                .filter_map(|inner| inner.get(i))
                .collect::<String>()
        })
        .collect();

    //trim results
    let mut result: Vec<Vec<String>> = result
        .split(|s| s.trim().is_empty())
        .map(|slice| slice.iter().map(|s| s.trim().to_string()).collect())
        .collect();

    //append operators
    for i in 0..ops.len() {
        result[i].push(ops[i].clone());
    }

    //convert to Ops
    let result: Vec<Vec<Op>> = result
        .into_iter()
        .map(|r| {
            r.into_iter()
                .map(|op| match op.as_str() {
                    "*" => Op::Mul,
                    "+" => Op::Plus,
                    n => Op::Num(n.parse::<i64>().unwrap()),
                })
                .collect()
        })
        .collect();

    result
}

pub fn part1(input: &str) -> i64 {
    let ps = parse_input_1(input);
    let mut count: i64 = 0;

    for mut p in ps {
        count += match p.pop().unwrap() {
            Op::Mul => p
                .into_iter()
                .map(|v| if let Op::Num(n) = v { n } else { 0 })
                .product::<i64>(),
            Op::Plus => p
                .into_iter()
                .map(|v| if let Op::Num(n) = v { n } else { 0 })
                .sum::<i64>(),
            _ => panic!("ERROR"),
        }
    }

    count
}

pub fn part2(input: &str) -> i64 {
    let ps = parse_input_2(input);
    let mut count = 0;

    for mut p in ps {
        count += match p.pop().unwrap() {
            Op::Mul => p
                .into_iter()
                .map(|v| if let Op::Num(n) = v { n } else { 0 })
                .product::<i64>(),
            Op::Plus => p
                .into_iter()
                .map(|v| if let Op::Num(n) = v { n } else { 0 })
                .sum::<i64>(),
            _ => panic!("ERROR"),
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part1() {
        let expected = 4277556;
        let result = part1(INPUT);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_part2() {
        let expected = 3263827;
        let result = part2(INPUT);
        assert_eq!(expected, result);
    }
}
