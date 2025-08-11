use game_logic::{GameState, print_game};
use strategy::{GameStrategy, MctsStrategy};

mod game_logic;
mod strategy;

fn main() {
    // let strategy = SimpleRandomStrategy { depth: 1000 };
    let strategy = MctsStrategy {
        tries: 1000,
        c_param: 1.4,
    };
    let mut scores = vec![];

    let mut game = GameState::new();
    while !game.is_game_over() {
        print_game(&game);
        let direction = strategy.next_move(&game);
        println!("{:?}", direction);
        game.move_tiles(direction);
    }

    println!("Game over! Final score: {}", game.score);
    scores.push(game.score as f64);
}
