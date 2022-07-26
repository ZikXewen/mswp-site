use std::{
    collections::HashSet,
    fmt::{self, Write},
};

use rand::Rng;

#[derive(Clone, PartialEq)]
pub enum Cell {
    Closed,
    Bomb,
    Empty(u8),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position(pub usize, pub usize);

#[derive(Default)]
pub struct Minesweeper {
    width: usize,
    height: usize,
    mines: HashSet<Position>,
    cells: Vec<Vec<Cell>>,
}

impl Minesweeper {
    pub fn new(height: usize, width: usize, mines_count: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut mines = HashSet::new();
        while mines.len() < mines_count {
            mines.insert(Position(rng.gen_range(0..height), rng.gen_range(0..width)));
        }
        Self {
            width,
            height,
            mines,
            cells: vec![vec![Cell::Closed; width]; height],
        }
    }
    fn neighbors(&self, Position(x, y): Position) -> impl Iterator<Item = Position> {
        let width = self.width;
        let height = self.height;
        (x.saturating_sub(1)..=usize::min(x + 1, height - 1))
            .flat_map(move |i| {
                (y.saturating_sub(1)..=usize::min(y + 1, width - 1)).map(move |j| Position(i, j))
            })
            .filter(move |&pos| pos != Position(x, y))
    }
    fn count_neighbor_mines(&self, pos: Position) -> u8 {
        self.neighbors(pos)
            .filter(|p| self.mines.contains(p))
            .count() as u8
    }
    pub fn open(&mut self, start: Position) {
        if self.mines.contains(&start) {
            let Position(x, y) = start;
            self.cells[x][y] = Cell::Bomb;
            return;
        }
        let mut stk = vec![start];
        while let Some(Position(x, y)) = stk.pop() {
            if self.cells[x][y] == Cell::Closed {
                let neighbor_mines = self.count_neighbor_mines(Position(x, y));
                if neighbor_mines > 0 {
                    self.cells[x][y] = Cell::Empty(neighbor_mines);
                } else {
                    self.cells[x][y] = Cell::Empty(0);
                    for Position(x, y) in self.neighbors(Position(x, y)) {
                        if self.cells[x][y] == Cell::Closed {
                            stk.push(Position(x, y));
                        }
                    }
                }
            }
        }
    }
    pub fn flag(&mut self, pos: Position) {
        todo!()
    }
}

impl fmt::Display for Minesweeper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.cells {
            for c in row {
                f.write_str(match c {
                    Cell::Bomb => "💣",
                    Cell::Closed => "⬛",
                    Cell::Empty(0) => "🟦",
                    Cell::Empty(1) => "1️⃣",
                    Cell::Empty(2) => "2️⃣",
                    Cell::Empty(3) => "3️⃣",
                    Cell::Empty(4) => "4️⃣",
                    Cell::Empty(5) => "5️⃣",
                    Cell::Empty(6) => "6️⃣",
                    Cell::Empty(7) => "7️⃣",
                    Cell::Empty(8) => "8️⃣",
                    _ => return Err(fmt::Error),
                })?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Minesweeper, Position};

    #[test]
    fn game_test() {
        let mut game = Minesweeper::new(5, 5, 3);
        game.open(Position(2, 2));
        println!("{game}");
    }

    #[test]
    fn neighbors_test() {
        let game = Minesweeper::new(5, 8, 3);
        dbg!(game.neighbors(Position(0, 1)).collect::<Vec<_>>());
        dbg!(game.neighbors(Position(3, 7)).collect::<Vec<_>>());
        dbg!(game.neighbors(Position(4, 7)).collect::<Vec<_>>());
    }
}
