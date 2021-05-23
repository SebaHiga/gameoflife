#[derive(Clone, Copy, Debug)]
pub enum Status {
    On,
    Off,
}

#[derive(Debug)]
pub struct Cell {
    pub row: u16,
    pub col: u16,
    pub content: char,
    pub status: Status,
}

impl Cell {
    pub fn get_status(&self) -> Status {
        self.status
    }

    pub fn set_position(&mut self, row: u16, col: u16) {
        self.row = row;
        self.col = col;
    }

    pub fn get_content(&self) -> char {
        self.content
    }
}
