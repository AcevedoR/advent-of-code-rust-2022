use std::collections::HashMap;

#[derive(Debug)]
pub struct Grid {
    content: HashMap<Position, Cell>,
    pub max_x: u32,
    pub max_y: u32,
}

impl Grid {
    pub fn new() -> Grid{
        Grid {content: HashMap::new(), max_y: 0, max_x: 0}
    }
    pub fn add_new_cell (&mut self, x:u32, y:u32, value: CellValue) {
        if x > self.max_x {
            self.max_x = x;
        }
        if y > self.max_y {
            self.max_y = y;
        }
        self.content.insert(Position{x,y}, Cell{x, y, value});
    }
    pub fn get(&self, x:u32, y:u32) -> Option<&Cell> {
        self.content.get(&Position { x, y })
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}
impl Position {
    pub fn new(x: u32, y: u32) -> Position {
        Position{x,y}
    }
}

#[derive(Debug)]
pub struct Cell {
    pub x: u32,
    pub y: u32,
    pub(crate) value: CellValue
}

#[derive(Debug, PartialEq)]
pub enum CellValue {
    EMPTY,
    PATH,
    WALL
}

impl TryFrom<char> for CellValue {
    type Error = ();

    fn try_from(v: char) -> Result<Self, Self::Error> {
        match v {
            x if x == ' ' => Ok(CellValue::EMPTY),
            x if x == '.'=> Ok(CellValue::PATH),
            x if x == '#' => Ok(CellValue::WALL),
            _ => Err(()),
        }
    }
}
