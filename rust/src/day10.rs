use cached::proc_macro::cached;
use good_lp::*;
use std::collections::VecDeque;

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<u64>>,
    joltages: Vec<i32>,
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|l| {
            let (lights, rest) = l.trim().split_once(' ').unwrap();

            let lights: Vec<bool> = lights[1..(lights.len() - 1)]
                .chars()
                .map(|c| if c == '.' { false } else { true })
                .collect();

            let mut buttons: Vec<Vec<u64>> = Vec::new();
            let mut joltages: Vec<i32> = Vec::new();

            for r in rest.split(' ') {
                if r.chars().nth(0) == Some('(') {
                    let vs: Vec<u64> = r[1..(r.len() - 1)]
                        .split(',')
                        .map(|b| b.parse::<u64>().unwrap())
                        .collect();
                    buttons.push(vs);
                } else {
                    joltages = r[1..(r.len() - 1)]
                        .split(',')
                        .map(|b| b.parse::<i32>().unwrap())
                        .collect();
                }
            }

            Machine {
                lights,
                buttons,
                joltages,
            }
        })
        .collect()
}

fn press_button(mut cur: Vec<bool>, buttons: &[u64]) -> Vec<bool> {
    for b in buttons {
        cur[*b as usize] = !cur[*b as usize];
    }
    cur
}

fn startup(m: &Machine, cur: Vec<bool>) -> u64 {
    let mut q: VecDeque<(Vec<bool>, u64)> = vec![(cur, 0)].into();

    while !q.is_empty() {
        if let Some((lights, count)) = q.pop_front() {
            if lights == m.lights {
                return count;
            }
            for bs in &m.buttons {
                q.push_back((press_button(lights.clone(), &bs), count + 1));
            }
        } else {
            panic!("ERROR");
        }
    }
    0
}

pub fn part1(input: &str) -> u64 {
    let machines = parse_input(input);
    let mut count: u64 = 0;
    for m in machines {
        let cur = vec![false; m.lights.len()];
        count += startup(&m, cur)
    }
    count
}

pub fn part2(input: &str) -> i32 {
    let machines = parse_input(input);
    let mut count: i32 = 0;

    for m in machines {
        let mut vectors: Vec<Vec<i32>> = vec![];
        let goal = m.joltages.clone();

        for bs in &m.buttons {
            let mut coeff: Vec<i32> = vec![0; goal.len()];
            for b in bs {
                coeff[*b as usize] = 1;
            }
            vectors.push(coeff);
        }

        let n_vectors = vectors.len();
        let mut vars = ProblemVariables::new();

        let coeffs: Vec<Variable> = (0..n_vectors)
            .map(|_| vars.add(variable().min(0).integer()))
            .collect();

        let mut problem = good_lp::microlp(vars.minimise(coeffs.iter().sum::<Expression>()));
        let mut expressions = vec![Expression::with_capacity(m.buttons.len()); m.joltages.len()];

        for (i, bs) in m.buttons.iter().enumerate() {
            for b in bs {
                expressions[*b as usize] += coeffs[i];
            }
        }

        for (e, j) in expressions.into_iter().zip(m.joltages.clone()) {
            problem.add_constraint(e.eq(j as f64));
        }

        let solution = problem.solve().unwrap();
        count += coeffs
            .iter()
            .map(|&var| solution.value(var).round() as i32)
            .sum::<i32>();
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_part1() {
        let expected = 7;
        let result = part1(INPUT);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_part2() {
        let expected: i32 = 33;
        let result = part2(INPUT);
        assert_eq!(expected, result);
    }
}
