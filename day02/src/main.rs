fn main() {
    let games = read_input("input.txt");
    part1(&games);
    part2(&games);
}

fn part1(games: &[Game]) {
    let possible_id_sum: u32 = games
        .iter()
        .filter(|game| {
            game.is_possible(&ColorTuple {
                red: 12,
                green: 13,
                blue: 14,
            })
        })
        .map(|game| game.id)
        .sum();
    println!("sum of possible IDs: {possible_id_sum}")
}

fn part2(games: &[Game]) {
    let power_sum: u32 = games
        .iter()
        .map(Game::minimum_set)
        .map(|min_set| min_set.power())
        .sum();
    println!("Sum of powers: {power_sum}")
}

#[derive(Default, Debug)]
struct ColorTuple {
    red: u32,
    green: u32,
    blue: u32,
}

impl ColorTuple {
    fn from_text(text: &str) -> Self {
        let mut poll = ColorTuple::default();
        for color_input in text.trim().split(',') {
            let (count, color) = color_input.trim().split_once(' ').unwrap();
            let count: u32 = count.trim().parse().unwrap();
            match color.trim() {
                "red" => poll.red = count,
                "green" => poll.green = count,
                "blue" => poll.blue = count,
                &_ => {}
            }
        }
        poll
    }

    fn is_possible(&self, bag_config: &ColorTuple) -> bool {
        bag_config.red >= self.red && bag_config.green >= self.green && bag_config.blue >= self.blue
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    polls: Vec<ColorTuple>,
}

impl Game {
    fn from_line(line: &str) -> Self {
        let (game_id, polls) = line.split_once(':').unwrap();
        let game_id: u32 = game_id[5..].parse().expect("game id invalid");
        let polls = polls.split(';').map(ColorTuple::from_text).collect();
        Self { id: game_id, polls }
    }

    fn is_possible(&self, bag_config: &ColorTuple) -> bool {
        self.polls.iter().all(|poll| poll.is_possible(bag_config))
    }

    fn minimum_set(&self) -> ColorTuple {
        self.polls
            .iter()
            .fold(ColorTuple::default(), |acc, x| ColorTuple {
                red: u32::max(acc.red, x.red),
                green: u32::max(acc.green, x.green),
                blue: u32::max(acc.blue, x.blue),
            })
    }
}

fn read_input(path: &str) -> Vec<Game> {
    let test = std::fs::read_to_string(path).expect("failed to read input file!");
    test.lines().map(Game::from_line).collect()
}
