use regex::Regex;

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

const REGEX_TEXT: &str = r"one|two|three|four|five|six|seven|eight|nine|\d";

fn part2(data: &str) -> usize {
    data.lines()
        .map(|line| {
            let re = Regex::new(REGEX_TEXT).unwrap();

            let first = re.find(line).unwrap().as_str();
            let first = decode(first);

            let last = (0..line.len())
                .rev()
                // .inspect(|i| println!("debug: {i}"))
                .find_map(|i| re.find_at(line, i))
                .unwrap()
                .as_str();
            let last = decode(last);

            first * 10 + last
        })
        .sum()
}

fn decode(text: &str) -> usize {
    match text {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        x => x.parse::<usize>().unwrap(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = include_str!("example1.txt");
    const DATA2: &str = include_str!("example2.txt");

    #[test]
    fn part1_test() {
        assert_eq!(part1(DATA1), 142);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(DATA2), 281);
    }
}
