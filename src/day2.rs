extern crate regex;

use regex::Regex;

pub struct Password {
    start: usize,
    end: usize,
    letter: char,
    password: String,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Password> {
    input
        .lines()
        .map(|l| {
            let re = Regex::new(r"(\d+)-(\d+) ([[:alpha:]]): ([[:alpha:]]+)").unwrap();
            let cap = re.captures(l).unwrap();
            Password {
                start: cap[1].parse().unwrap(),
                end: cap[2].parse().unwrap(),
                letter: cap[3].chars().nth(0).unwrap(),
                password: cap[4].to_string(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Password]) -> u32 {
    let mut res = 0;
    for p in input {
        let count = p.password.chars().filter(|&c| c == p.letter).count();
        if p.start <= count && count <= p.end {
            res += 1;
        }
    }

    res
}

#[aoc(day2, part2)]
pub fn part2(input: &[Password]) -> u32 {
    let mut res = 0;
    for p in input {
        if (p.password.chars().nth(p.start - 1).unwrap() == p.letter)
            ^ (p.password.chars().nth(p.end - 1).unwrap() == p.letter)
        {
            res += 1;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
        assert_eq!(part1(&input_generator(input)), 2);
    }

    #[test]
    fn example_part2() {
        let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
        assert_eq!(part2(&input_generator(input)), 1);
    }
}
