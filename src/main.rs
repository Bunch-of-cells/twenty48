use twenty48::{Game, Direction};

fn main() {
    let mut grid = Game::new();
    println!("{}", grid);
    grid.step(Direction::Right);
    println!("{}", grid);
}
