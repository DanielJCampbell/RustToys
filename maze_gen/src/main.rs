enum Cell {
    Wall,
    Empty,
}

struct BackTracker {
    width:  usize,
    height: usize,
    seed:   i32,
}

trait MazeGen {
    fn gen(&self) -> Vec<Cell>;
}

impl Clone for Cell {
    fn clone(&self) -> Self {
        match self {
            &Cell::Wall => Cell::Wall,
            &Cell::Empty => Cell::Empty
        }
    }
}

impl MazeGen for BackTracker {
    fn gen(&self) -> Vec<Cell> {
        let size = self.width*self.height;
        let mut result = vec![Cell::Wall; size];
        result
    }
}

fn main() {
    println!("Hello, world!");
}
