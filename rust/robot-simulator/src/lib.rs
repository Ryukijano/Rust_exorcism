// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot;

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Robot {
        Robot { x, y, d }
        
    }

    #[must_use]
    pub fn turn_right(&mut self) {
        self.d = match self.d {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
    }

    #[must_use]
    pub fn turn_left(&mut self) {
        self.d = match self.d {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
    }

    #[must_use]
    pub fn advance(&mut self) {
        match self.d {
            Direction::North => self.y += 1,
            Direction::East => self.x += 1,
            Direction::South => self.y -= 1,
            Direction::West => self.x -= 1,
        };
    }


    #[must_use]
    pub fn instructions(&mut self, instructions: &str) {
        for c in instructions.chars() {
            match c {
                'R' => self.turn_right(),
                'L' => self.turn_left(),
                'A' => self.advance(),
                _ => panic!("Invalid instruction! : {}", c),
            }
        }
    }

    }

    pub fn position(&self) -> (i32, i32) {
        
    }

    pub fn direction(&self) -> &Direction {
        unimplemented!()
    }
}
