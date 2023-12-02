const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part 1: {:?}", part1(DATA));
    println!("part 2: {:?}", part2(DATA));
}

fn part1(data: &str) -> u32 {
    data.lines()
        .map(|line| {
            let first = line.chars().find_map(|c| c.to_digit(10)).unwrap();
            let last = line.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
            first * 10 + last
        })
        .sum()
}

fn part2(data: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = include_str!("example.txt");

    #[test]
    fn part1_test() {
        assert_eq!(part1(DATA), 142);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(DATA), 45000);
    }
}
