mod minesweeper;

use std::cell::RefCell;

use minesweeper::*;
use wasm_bindgen::prelude::*;

thread_local! {
    static GAME: RefCell<Minesweeper> = RefCell::new(Minesweeper::default());
}

#[wasm_bindgen]
pub fn start(width: usize, height: usize, mines_count: usize) {
    GAME.with(|game| {
        game.replace(Minesweeper::new(width, height, mines_count));
    });
}

#[wasm_bindgen]
pub fn open(x: usize, y: usize) {
    GAME.with(|game| {
        game.borrow_mut().open(Position(x, y));
    });
}

#[wasm_bindgen]
pub fn flag(x: usize, y: usize) {
    GAME.with(|game| {
        game.borrow_mut().flag(Position(x, y));
    });
}

#[wasm_bindgen]
pub fn fetch() -> String {
    GAME.with(|game| game.borrow().to_string())
}

#[wasm_bindgen(js_name=gameEnded)]
pub fn game_ended() -> bool {
    GAME.with(|game| game.borrow().end)
}
