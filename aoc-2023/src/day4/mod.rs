
struct CardContainer {
    cards: [Card; 193],
    current_depth: usize,
}

impl CardContainer {
    fn compute_wins(&mut self, card: Card) -> usize {
        self.current_depth += 1;

        //print the depth and cards
        print!("{}", self.current_depth);
        for i in 0 .. self.current_depth {
            print!("-");
        }

        let calculated_wins = card.calculate_wins();

        println!("Card: {}, wins: {}", card.id, calculated_wins);

        let range = card.id .. card.id + calculated_wins; 
        let mut result = calculated_wins;

        for i in range {
            result += self.compute_wins(self.cards[i]);
        }
        self.current_depth -= 1;
        result
    }
}

#[derive(Copy, Clone)]
struct Card {
    id: usize,
    win_numbers: [usize; 10],
    own_numbers: [usize; 25] 
}

impl Card { 
    fn from_string(input : String) -> Card {
        let list_split: Vec<_> = input.split(":").enumerate().collect();
        let list_split_numbers = list_split[1].1;
        let numbers_split: Vec<_> = list_split_numbers.split("|").enumerate().collect();

        let own_numbers_str: Vec<&str> = numbers_split[1].1.trim().split(" ").enumerate().map(|(_, s)| s.trim()).collect();
        let win_numbers_str: Vec<&str> = numbers_split[0].1.trim().split(" ").enumerate().map(|(_, s)| s.trim()).collect();

        let mut parse_own_numbers = vec![];
        let mut parse_win_numbers = vec![];

        own_numbers_str.iter().for_each(|s| {
            match s.parse::<i32>() {
                Ok(n) => parse_own_numbers.push(n),
                Err(_) => {},
            }
        });

        win_numbers_str.iter().for_each(|s| {
            match s.parse::<i32>() {
                Ok(n) => parse_win_numbers.push(n),
                Err(_) => {},
            }
        });

        let id = list_split[0].1.split(" ").enumerate().collect::<Vec<_>>().last().unwrap().1.parse::<usize>().unwrap();

        
        let mut win_numbers = [0; 10];
        let mut own_number = [0; 25];

        parse_win_numbers.iter().enumerate().for_each(|(i, n)| {
            win_numbers[i] = *n as usize;
        });

        parse_own_numbers.iter().enumerate().for_each(|(i, n)| {
            own_number[i] = *n as usize;
        });

        Card { id: id, win_numbers: win_numbers, own_numbers: own_number} 
    }

    fn calculate_wins(&self) -> usize {
        let mut win_count : u32 = 0;
        self.win_numbers.iter().for_each(|n| {
            if self.own_numbers.contains(&n) && *n != 0 {
                win_count += 1;
            }
        });
        win_count as usize
    }

}

pub fn run(contents: String) {
    let mut cards: [Card; 193] = [Card { id: 0, win_numbers: [0; 10], own_numbers: [0; 25] }; 193];
    let lines = contents.split("\n");
    let mut result = 0;
    for line in lines.enumerate() {
        let card = Card::from_string(line.1.to_string());
        cards[line.0] = card;
    }

    let mut card_container = CardContainer {
        cards,
        current_depth: 0
    };

    let mut result = 0;
    for card in cards {
        result += card_container.compute_wins(card) + 1;
    }

    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::{Card, CardContainer};

    #[test]
    fn test() {
        let input  = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string();
        let card = Card::from_string(input.to_string());
        let wins = card.calculate_wins();
        assert_eq!(wins, 4);
    }

    #[test]
    fn test_from_aoc() {
        let mut index = 0; 

        let mut game = CardContainer {
            cards: [Card { id: 0, win_numbers: [0; 10], own_numbers: [0; 25] }; 193],
            current_depth: 0
        };

        let card = Card::from_string("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string());
        game.cards[index] = card;
        index += 1;
        assert_eq!(card.calculate_wins(), 4);

        let card = Card::from_string("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string());
        game.cards[index] = card;
        index += 1;
        assert_eq!(card.calculate_wins(), 2);

        let card = Card::from_string("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string());
        game.cards[index] = card;
        index += 1;
        assert_eq!(card.calculate_wins(), 2);

        let card = Card::from_string("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string());
        game.cards[index] = card;
        index += 1;
        assert_eq!(card.calculate_wins(), 1);

        let card = Card::from_string("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string());
        game.cards[index] = card;
        index += 1;
        assert_eq!(card.calculate_wins(), 0);

        let card = Card::from_string("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string());
        game.cards[index] = card;
        index += 1;
        assert_eq!(card.calculate_wins(), 0);

        let mut result = 0;
        for i in 0 .. index {
            result += game.compute_wins(game.cards[i]) + 1;
        }
        // card 1 : 1
        // card 2 : 2
        // card 3 : 4
        // card 4 : 8
        // card 5 : 14
        // card 6 : 1
        println!("Result: {}", result);
        assert_eq!(result, 30);

    }
}
