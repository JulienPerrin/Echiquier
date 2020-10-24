// use rand::Rng;
// use std::cmp::Ordering;

// #[derive(Copy, Clone)]
// pub struct Game {
//     last_guess: Option<u32>,
//     secret_number: u32,
//     min: u32,
//     max: u32,
//     result: GameResult,
// }

// #[derive(Copy, Clone)]
// pub enum GameResult {
//     Ongoing,
//     Win,
// }

// impl Game {
//     pub fn new() -> Game {
//         println!("===== Game =====");
//         let min = 1;
//         let max = 100;
//         let secret_number = rand::thread_rng().gen_range(min, max + 1);
//         let last_guess = None;
//         let result = GameResult::Ongoing;

//         println!("Guess the number!");
//         println!("Please input a number between {} and {}", min, max);
//         Game {
//             last_guess,
//             secret_number,
//             min,
//             max,
//             result,
//         }
//     }
//     pub fn play_turn(&mut self, guess: u32) {
//         self.last_guess = Some(guess);

//         if guess < self.min || guess > self.max {
//             println!(
//                 "Please input a number between {} and {}!",
//                 self.min, self.max
//             );
//         }

//         match guess.cmp(&self.secret_number) {
//             Ordering::Less => println!("Too small!"),
//             Ordering::Greater => println!("Too big!"),
//             Ordering::Equal => {
//                 self.result = GameResult::Win;
//                 println!("You win!");
//             }
//         }
//     }
//     #[allow(dead_code)]
//     pub fn is_ongoing(self) -> bool {
//         match self.result {
//             GameResult::Ongoing => true,
//             _ => false,
//         }
//     }
//     pub fn is_won(self) -> bool {
//         match self.result {
//             GameResult::Win => true,
//             _ => false,
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn start_game() {
//         let game: Game = Game::new();
//         assert_eq!(1, game.min);
//         assert_eq!(100, game.max);

//         assert_eq!(None, game.last_guess);
//         assert!(1 <= game.secret_number);
//         assert!(100 >= game.secret_number);
//         assert!(game.is_ongoing());
//     }
//     #[test]
//     fn play_and_win() {
//         let mut game = Game::new();
//         assert!(game.is_ongoing());
//         game.play_turn(game.secret_number);
//         assert!(game.is_won());
//     }
// }
