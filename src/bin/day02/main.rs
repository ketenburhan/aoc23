use regex::Regex;

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part 1: {:?}", part1(DATA));
    println!("part 2: {:?}", part2(DATA));
}

const MAX_R: usize = 12;
const MAX_G: usize = 13;
const MAX_B: usize = 14;

fn part1(data: &str) -> usize {
    let re_r = Regex::new(r"(\d+) red").unwrap();
    let re_g = Regex::new(r"(\d+) green").unwrap();
    let re_b = Regex::new(r"(\d+) blue").unwrap();

    data.lines()
        .enumerate()
        .map(|(game_index, line)| {
            let game_data = line.split_once(':').unwrap().1.trim();
            let sets = game_data.split(';');
            let is_valid = sets
                .map(|text| {
                    let mut set_data = [0_usize, 0, 0];
                    if let Some(red) = re_r.captures(text) {
                        set_data[0] = red[1].parse().unwrap();
                    }
                    if let Some(green) = re_g.captures(text) {
                        set_data[1] = green[1].parse().unwrap();
                    }
                    if let Some(blue) = re_b.captures(text) {
                        set_data[2] = blue[1].parse().unwrap();
                    }
                    set_data
                })
                .all(|[r, g, b]| r <= MAX_R && g <= MAX_G && b <= MAX_B);

            if is_valid {
                game_index + 1
            } else {
                0
            }
        })
        .sum()
}

fn part2(data: &str) -> usize {
    let re_r = Regex::new(r"(\d+) red").unwrap();
    let re_g = Regex::new(r"(\d+) green").unwrap();
    let re_b = Regex::new(r"(\d+) blue").unwrap();

    data.lines()
        .map(|line| {
            let game_data = line.split_once(':').unwrap().1.trim();
            let sets = game_data.split(';');
            let max = sets
                .map(|text| {
                    let mut set_data = [0_usize, 0, 0];
                    if let Some(red) = re_r.captures(text) {
                        set_data[0] = red[1].parse().unwrap();
                    }
                    if let Some(green) = re_g.captures(text) {
                        set_data[1] = green[1].parse().unwrap();
                    }
                    if let Some(blue) = re_b.captures(text) {
                        set_data[2] = blue[1].parse().unwrap();
                    }
                    set_data
                })
                .reduce(|[max_r, max_g, max_b], [r, g, b]| {
                    [max_r.max(r), max_g.max(g), max_b.max(b)]
                })
                .unwrap();

            max.into_iter().product::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = include_str!("example.txt");

    #[test]
    fn part1_test() {
        assert_eq!(part1(DATA), 8);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(DATA), 2286);
    }
}
