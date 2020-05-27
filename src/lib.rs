use js_sys::Array;
use rand::Rng;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    console::log_1(&JsValue::from_str("Rust initialising..."));

    Ok(())
}

#[derive(Debug, PartialEq)]
enum GameState {
    Continue(u32),
    Win,
    Lose,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Game {
    board: Vec<u32>,
    size: u32,
    state: GameState,
}

#[wasm_bindgen]
impl Game {
    pub fn new(size: u32, colours: u32) -> Game {
        let mut rng = rand::thread_rng();
        Game {
            board: (0..(size * size))
                .map(|_| rng.gen_range(0, colours))
                .collect(),
            size,
            state: GameState::Continue(50),
        }
    }

    pub fn get_board(&self) -> Array {
        self.board
            .iter()
            .map(|x| JsValue::from(*x))
            .collect::<Array>()
    }

    pub fn take_turn(&mut self, position: u32) {
        if let GameState::Continue(i) = self.state {
            self.floodfill(0, self.board[0], self.board[position as usize]);

            self.state = match self.is_win() {
                true => GameState::Win,
                false => self.next_state(i - 1),
            };
        }
    }

    pub fn get_state(&self) -> String {
        match self.state {
            GameState::Win => "WIN".to_string(),
            GameState::Continue(x) => x.to_string(),
            GameState::Lose => "LOSE".to_string(),
        }
    }
}

impl Game {
    fn floodfill(&mut self, position: u32, target: u32, replacement: u32) {
        if self.board[position as usize] == target {
            self.board[position as usize] = replacement;

            if let Some(next) = self.right(position) {
                self.floodfill(next, target, replacement);
            }
            if let Some(next) = self.down(position) {
                self.floodfill(next, target, replacement);
            }
            if let Some(next) = self.left(position) {
                self.floodfill(next, target, replacement);
            }
            if let Some(next) = self.up(position) {
                self.floodfill(next, target, replacement);
            }
        }
    }

    fn left(&self, pos: u32) -> Option<u32> {
        if ((pos % self.size) as i32) - 1 > -1 {
            Some(pos - 1)
        } else {
            None
        }
    }

    fn right(&self, pos: u32) -> Option<u32> {
        if (pos % self.size) + 1 < self.size {
            Some(pos + 1)
        } else {
            None
        }
    }

    fn up(&self, pos: u32) -> Option<u32> {
        if (pos as i32) - (self.size as i32) > -1 {
            Some(pos - self.size)
        } else {
            None
        }
    }

    fn down(&self, pos: u32) -> Option<u32> {
        if pos + self.size < (self.size * self.size) {
            Some(pos + self.size)
        } else {
            None
        }
    }

    fn is_win(&self) -> bool {
        self.board.iter().all(|x| x == &self.board[0])
    }

    fn next_state(&mut self, next: u32) -> GameState {
        match next {
            0 => GameState::Lose,
            i => GameState::Continue(i),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn left_some_left() {
        let game = Game {
            board: vec![],
            size: 5,
            state: GameState::Continue(5),
        };
        assert_eq!(game.left(1), Some(0));
    }

    #[test]
    fn left_none() {
        let game = Game {
            board: vec![],
            size: 5,
            state: GameState::Continue(5),
        };
        assert_eq!(game.left(0), None);
    }

    #[test]
    fn right_some_right() {
        let game = Game {
            board: vec![],
            size: 5,
            state: GameState::Continue(5),
        };
        assert_eq!(game.right(0), Some(1));
    }

    #[test]
    fn right_none() {
        let game = Game {
            board: vec![],
            size: 5,
            state: GameState::Continue(5),
        };
        assert_eq!(game.right(4), None);
    }

    #[test]
    fn up_some_up() {
        let game = Game {
            board: vec![],
            size: 5,
            state: GameState::Continue(5),
        };
        assert_eq!(game.up(20), Some(15));
    }

    #[test]
    fn up_none() {
        let game = Game {
            board: vec![],
            size: 5,
            state: GameState::Continue(5),
        };
        assert_eq!(game.up(4), None);
    }

    #[test]
    fn down_some_down() {
        let game = Game {
            board: vec![],
            size: 5,
            state: GameState::Continue(5),
        };
        assert_eq!(game.down(15), Some(20));
    }

    #[test]
    fn down_none() {
        let game = Game {
            board: vec![],
            size: 5,
            state: GameState::Continue(5),
        };
        assert_eq!(game.down(20), None);
    }

    #[test]
    fn take_winning_turn() {
        let mut game = Game {
            board: vec![0; 25],
            size: 5,
            state: GameState::Continue(25),
        };
        game.board[0] = 1;

        game.take_turn(1);
        assert_eq!(game.state, GameState::Win);
    }

    #[test]
    fn take_losing_turn() {
        let mut game = Game {
            board: vec![0; 25],
            size: 5,
            state: GameState::Continue(1),
        };
        game.board[0] = 1;
        game.board[20] = 1;

        game.take_turn(1);
        assert_eq!(GameState::Lose, game.state);
    }
}
