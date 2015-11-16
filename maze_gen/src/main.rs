use std::fmt;

enum Cell {
    Wall,
    Empty,
}

pub struct Maze {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

pub struct Backtracker {
    seed:   i32,
}

impl Maze {

    pub fn new(width: usize, height: usize) -> Maze {
        Maze {cells: vec![Cell::Wall; width*height], width: width, height: height}
    }
}

trait MazeGen {
    fn gen(&self, width: usize, height: usize) -> Maze;
}

impl Clone for Cell {
    fn clone(&self) -> Self {
        match self {
            &Cell::Wall => Cell::Wall,
            &Cell::Empty => Cell::Empty
        }
    }
}

impl MazeGen for Backtracker {
    fn gen(&self, width: usize, height: usize) -> Maze {
        let size = width*height;
        let mut result = Maze::new(width, height);
        unimplemented!()
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = "".to_owned();
        for i in 0..self.height {
            for j in 0..self.width {
                result.push_str(match self.cells[i*self.width + j] {
                    Cell::Wall  => "X",
                    Cell::Empty => " "
                });
            }
            result.push_str("\n");
        }
        write!(f, "{}", result)
    }
}

mod tests {
    use super::*;

    #[test]
    fn display_empty() {
        let maze = Maze::new(1, 1);
        assert_eq!(format!("{}", maze), "X\n");
        let maze = Maze::new(2, 2);
        assert_eq!(format!("{}", maze), "XX\nXX\n");
        let maze = Maze::new(1, 2);
        assert_eq!(format!("{}", maze), "X\nX\n");
        let maze = Maze::new(2, 1);
        assert_eq!(format!("{}", maze), "XX\n");
    }
}