use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", process_part_1(input));

    let input = include_str!("input.txt");
    println!("Part 2: {}", process_part_2(input));
}

#[derive(Debug)]
struct Game {
    number: usize,
    rounds: Vec<Round>,
}

#[derive(Debug)]
struct Round {
    entries: Vec<Entry>,
}

#[derive(Debug)]
struct Entry {
    count: usize,
    color: String,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(":").collect();
        let number = parts[0]
            .trim()
            .strip_prefix("Game")
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();

        let hands: Vec<&str> = parts[1].trim().split(";").collect();

        let rounds: Vec<Round> = hands
            .iter()
            .map(|round| {
                let entries: Vec<Entry> = round
                    .trim()
                    .split(", ")
                    .map(|entry| {
                        let vals: Vec<&str> = entry.split(" ").collect();
                        Entry {
                            count: vals[0].parse::<usize>().unwrap(),
                            color: vals[1].to_string(),
                        }
                    })
                    .collect();

                Round { entries }
            })
            .collect();

        Ok(Game { number, rounds })
    }
}

fn process_part_1(input: &str) -> usize {
    let mut output = 0;
    let games: Result<Vec<Game>, _> = input
        .lines()
        .map(|line| {
            println!("Line: {}", line);
            line.trim().parse()
        })
        .collect();

    let red_limit = 12;
    let green_limit = 13;
    let blue_limit = 14;

    let mut impossible_games: Vec<usize> = Vec::new();

    match games {
        Ok(parsed_games) => {
            for game in parsed_games {
                game.rounds.iter().for_each(|round| {
                    round.entries.iter().for_each(|entry| {
                        if entry.color == "red" {
                            if entry.count > red_limit {
                                impossible_games.push(game.number);
                            }
                        }
                        if entry.color == "green" {
                            if entry.count > green_limit {
                                impossible_games.push(game.number);
                            }
                        }
                        if entry.color == "blue" {
                            if entry.count > blue_limit {
                                impossible_games.push(game.number);
                            }
                        }
                    });
                });

                if !impossible_games.contains(&game.number) {
                    output += game.number;
                }
            }
        }
        Err(_) => println!("Error"),
    }
    output
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn process_part_2(input: &str) -> usize {
    let mut output = 0;

    let games: Result<Vec<Game>, _> = input.lines().map(|line| line.trim().parse()).collect();

    match games {
        Ok(parsed_game) => {
            parsed_game.iter().for_each(|game| {
                let mut min_red = 0;
                let mut min_green = 0;
                let mut min_blue = 0;

                game.rounds.iter().for_each(|round| {
                    round.entries.iter().for_each(|entry| {
                        if entry.color == "red" {
                            if entry.count > min_red {
                                min_red = entry.count;
                            }
                        }
                        if entry.color == "green" {
                            if entry.count > min_green {
                                min_green = entry.count;
                            }
                        }
                        if entry.color == "blue" {
                            if entry.count > min_blue {
                                min_blue = entry.count;
                            }
                        }
                    });
                });

                output += min_red * min_blue * min_green
            });
        }

        Err(_) => println!("Error"),
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("test.txt");
        assert_eq!(process_part_1(input), 8);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("test.txt");
        assert_eq!(process_part_2(input), 2286);
    }
}
