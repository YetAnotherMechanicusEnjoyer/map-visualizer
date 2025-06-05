use std::fmt;

pub enum CreationError {
    Negative,
    Zero,
    Empty,
}

impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let desc = match *self {
            CreationError::Negative => "Value must be greater than zero",
            CreationError::Zero => "Value must be greater than zero",
            CreationError::Empty => "Value mustn't be empty",
        };
        f.write_str(desc)
    }
}

fn check_u(n: i32) -> Result<(), CreationError> {
    match n {
        x if x < 0 => Err(CreationError::Negative),
        0 => Err(CreationError::Zero),
        _ => Ok(()),
    }
}

pub struct Map<'a> {
    walls: &'a str,
    paths: &'a str,
}

impl<'a> Map<'a> {
    pub fn new(walls: &'a str, paths: &'a str) -> Result<Self, CreationError> {
        if walls.is_empty() || paths.is_empty() {
            Err(CreationError::Empty)
        } else {
            Ok(Self { walls, paths })
        }
    }
    pub fn get_walls(self) -> &'a str {
        self.walls
    }
    pub fn get_paths(self) -> &'a str {
        self.paths
    }
}

impl<'a> fmt::Display for Map<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let desc = format!("({:?}, {:?})", self.walls, self.paths);
        f.write_str(&desc)
    }
}
