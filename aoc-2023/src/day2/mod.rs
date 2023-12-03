
#[derive(Debug, PartialEq)]
struct SubSet {
    red_count: usize,
    blue_count: usize,
    green_count: usize
}

impl SubSet {
    fn new() -> SubSet {
        SubSet {
            red_count: 0,
            blue_count: 0,
            green_count: 0
        }
    }

    fn from_string(input: String) -> SubSet {
        let set_string = input.split(",");
        let mut result_subset = SubSet::new();
        for set in set_string.enumerate() {
            let set_trimmed = set.1.trim();
            let set_trimmed_split: Vec<_> = set_trimmed.split(" ").enumerate().collect();

            if set_trimmed_split.len() != 2 {
                panic!("Invalid input: {}", input);
            }

            let color = set_trimmed_split[1].1;
            let amount = set_trimmed_split[0].1;

            match color {
                "red" => result_subset.red_count = amount.parse::<usize>().unwrap(),
                "blue" => result_subset.blue_count = amount.parse::<usize>().unwrap(),
                "green" => result_subset.green_count = amount.parse::<usize>().unwrap(),
                _ => panic!("Invalid color: {}", color)
            }

        }
        result_subset
    }
}

#[derive(Debug, PartialEq)]
struct Game {
    id: usize,
    subsets: Vec<SubSet>
}

impl Game {
    fn new() -> Game {
        Game {
            id: 0,
            subsets: vec![]
        }
    }

    fn from_string(input: String) -> Game {
        let mut result_game = Game::new();
        let game: Vec<_> = input.split(":").enumerate().collect();
        let sets: Vec<_> = game[1].1.split(";").enumerate().collect();
        let sets_trimmed: Vec<_> = sets.iter().map(|s| s.1.trim()).collect();

        let game_id = game[0].1.split(" ").last().unwrap();

        result_game.id = game_id.parse::<usize>().unwrap();

        for set in sets_trimmed {
            result_game.subsets.push(SubSet::from_string(set.to_string()));
        }

        result_game
    }

    fn power(&self) -> usize {
        let min_red = self.subsets.iter().map(|s| s.red_count).max().unwrap();
        let min_blue = self.subsets.iter().map(|s| s.blue_count).max().unwrap();
        let min_green = self.subsets.iter().map(|s| s.green_count).max().unwrap();
        min_red * min_blue * min_green
    }

    fn possible(&self) -> bool {
        self.subsets.iter().all(|s| {
            s.red_count <= 12 && s.blue_count <= 14 && s.green_count <= 13
        })
    }
        

}

pub fn run(contents: String) {
    let lines = contents.split("\n");
    let mut result = 0;
    let mut power_result = 0;
    for line in lines.enumerate() {
        let game = Game::from_string(line.1.to_string());
        power_result += game.power();
        if game.possible() {
            result += game.id;
        }
    }

    println!("Sum Possible: {}", result);
    println!("Sum Power: {}", power_result);
    
}

#[cfg(test)]
mod tests {
    use crate::day2::{Game, SubSet};

    #[test]
    fn test_power() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = Game::from_string(input.to_string());
        let power = game.power();
        assert_eq!(power, 48);
    }

    #[test]
    fn test_aoc_input() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = Game::from_string(input.to_string());
        assert_eq!(game.possible(), true);

        let input = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let game = Game::from_string(input.to_string());
        assert_eq!(game.possible(), true);

        let input = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let game = Game::from_string(input.to_string());
        assert_eq!(game.possible(), false);

        let input = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        let game = Game::from_string(input.to_string());
        assert_eq!(game.possible(), false);

        let input = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let game = Game::from_string(input.to_string());
        assert_eq!(game.possible(), true);
    }

    #[test]
    fn test_game_from_string() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected_game = Game {
            id: 1,
            subsets: vec![
                SubSet {
                    red_count: 4,
                    blue_count: 3,
                    green_count: 0
                },
                SubSet {
                    red_count: 1,
                    blue_count: 6,
                    green_count: 2
                },
                SubSet {
                    red_count: 0,
                    blue_count: 0,
                    green_count: 2
                }
            ]
        };
        let game = Game::from_string(input.to_string());
        assert_eq!(game, expected_game);
    }

    #[test]
    fn test_from_string() {
        let input = "3 blue, 4 red";
        let subset = SubSet::from_string(input.to_string());
        let expected_subset = SubSet {
            red_count: 4,
            blue_count: 3,
            green_count: 0
        };
        assert_eq!(subset, expected_subset);

        let input = "3 blue, 0 green";
        let subset_1 = SubSet::from_string(input.to_string());
        let expected_subset_1 = SubSet {
            red_count: 0,
            blue_count: 3,
            green_count: 0
        };
        assert_eq!(subset_1, expected_subset_1);
    }
}
