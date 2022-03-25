use twenty48::{Game, Direction};

fn main() {
    let grid = Game::new();
    println!("{}", grid);
    grid.start(play);
}

fn play(grid: &[usize; 16]) -> Direction {
    println!("{:?}", grid);
    Direction::Up
}
