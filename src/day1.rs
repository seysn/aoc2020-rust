#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> i32 {
    for i in input.iter() {
        if let Some(_) = input.iter().position(|&x| x == (2020 - i)) {
            return (2020 - i) * i;
        }
    }
    unreachable!();
}

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> i32 {
    for i in input.iter() {
        let candidates: Vec<&i32> = input
            .iter()
            .filter(|&x| x < &(2020 - i) && x != i)
            .collect();

        for &j in candidates.iter() {
            if let Some(_) = input.iter().position(|&x| x == (2020 - i - j)) {
                return (2020 - i - j) * j * i;
            }
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        assert_eq!(part1(&[1721, 979, 366, 299, 675, 1456]), 514579);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(&[1721, 979, 366, 299, 675, 1456]), 241861950);
    }
}
