pub struct Map {
    grid: Vec<String>,
}

impl Map {
    fn is_tree(&self, x: usize, y: usize) -> bool {
        self.grid[y].chars().nth(x % self.grid[y].len()).unwrap() == '#'
    }

    pub fn count(&self, right: usize, down: usize) -> u32 {
        let mut x = 0;
        let mut y = 0;
        let mut res = 0;

        while y < self.grid.len() {
            if self.is_tree(x, y) {
                res += 1;
            }
            x += right;
            y += down;
        }

        res
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Map {
    Map {
        grid: input.split("\n").map(|l| l.to_string()).collect(),
    }
}

#[aoc(day3, part1)]
pub fn part1(input: &Map) -> u32 {
    input.count(3, 1)
}

#[aoc(day3, part2)]
pub fn part2(input: &Map) -> u32 {
    let rules = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    rules.iter().fold(1, |acc, x| acc * input.count(x.0, x.1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let input = "..##.......\n\
        #...#...#..\n\
        .#....#..#.\n\
        ..#.#...#.#\n\
        .#...##..#.\n\
        ..#.##.....\n\
        .#.#.#....#\n\
        .#........#\n\
        #.##...#...\n\
        #...##....#\n\
        .#..#...#.#";
        assert_eq!(part1(&input_generator(input)), 7);
    }

    #[test]
    fn example_part2() {
        let input = "..##.......\n\
        #...#...#..\n\
        .#....#..#.\n\
        ..#.#...#.#\n\
        .#...##..#.\n\
        ..#.##.....\n\
        .#.#.#....#\n\
        .#........#\n\
        #.##...#...\n\
        #...##....#\n\
        .#..#...#.#";
        assert_eq!(part2(&input_generator(input)), 336);
    }
}
