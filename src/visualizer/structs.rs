use std::fmt;

pub enum CreationError {
    Empty,
}

impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let desc = match *self {
            CreationError::Empty => "Value mustn't be empty",
        };
        f.write_str(desc)
    }
}

#[derive(Debug, Clone)]
pub struct Map<'a> {
    map: &'a str,
    walls: &'a str,
    paths: &'a str,
}

impl<'a> Map<'a> {
    pub fn new(map: &'a str, walls: &'a str, paths: &'a str) -> Result<Self, CreationError> {
        if map.is_empty() || walls.is_empty() || paths.is_empty() {
            Err(CreationError::Empty)
        } else {
            Ok(Self { map, walls, paths })
        }
    }
    pub fn get_map(&self) -> &'a str {
        self.map
    }
    pub fn get_walls(&self) -> &'a str {
        self.walls
    }
    pub fn get_paths(&self) -> &'a str {
        self.paths
    }
}

impl<'a> fmt::Display for Map<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let desc = format!("{:?}\n({:?}, {:?})", self.map, self.walls, self.paths);
        f.write_str(&desc)
    }
}
