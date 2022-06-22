struct Game {
    rolls: Vec<u32>,
}

impl Game {
    fn new() -> Game {
        Game { rolls: vec![] }
    }

    fn roll(&mut self, pins: u32) {
        self.rolls.push(pins);
    }

    fn score(self) -> u32 {
        let mut score = 0;
        let mut current_roll = 0;
        for _frame in 0..10 {
            if self.is_strike(current_roll) {
                score += self.get_score_for_rolls(current_roll, 3);
                current_roll += 1;
            } else if self.is_spare(current_roll) {
                score += self.get_score_for_rolls(current_roll, 3);
                current_roll += 2;
            } else {
                score += self.get_score_for_rolls(current_roll, 2);
                current_roll += 2;
            }
        }
        score
    }

    fn get_score_for_rolls(&self, start_index: usize, rolls: usize) -> u32 {
        let mut score = 0;
        for i in 0..rolls {
            score += self.rolls[start_index + i];
        }
        score
    }

    fn is_spare(&self, current_roll: usize) -> bool {
        self.rolls[current_roll] + self.rolls[current_roll + 1] == 10
    }

    fn is_strike(&self, current_roll: usize) -> bool {
        self.rolls[current_roll] == 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Game {
        fn roll_many(&mut self, pins: u32, rolls: u32) {
            for _ in 0..rolls {
                self.roll(pins);
            }
        }
    }

    #[test]
    fn test_all_gutters() {
        let mut game = Game::new();
        game.roll_many(0, 20);
        assert_eq!(game.score(), 0);
    }

    #[test]
    fn test_all_ones() {
        let mut game = Game::new();
        game.roll_many(1, 20);
        assert_eq!(game.score(), 20);
    }

    #[test]
    fn test_all_fours() {
        let mut game = Game::new();
        game.roll_many(4, 20);
        assert_eq!(game.score(), 80);
    }

    #[test]
    fn test_one_spare() {
        let mut game = Game::new();
        game.roll(2);
        game.roll(8);
        game.roll(4);
        game.roll_many(0, 17);
        assert_eq!(game.score(), 18);
    }

    #[test]
    fn test_one_strike() {
        let mut game = Game::new();
        game.roll(10);
        game.roll(3);
        game.roll(5);
        game.roll_many(0, 17);
        assert_eq!(game.score(), 26);
    }

    #[test]
    fn test_perfect_game() {
        let mut game = Game::new();
        game.roll_many(10, 12);
        assert_eq!(game.score(), 300);
    }
}
