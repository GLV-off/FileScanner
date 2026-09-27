pub type Position = i32;
pub type Size = i32;

#[derive(Default)]
pub struct Dimensions {
    start: Position,
    stop: Position,
    size: Size
}

impl Dimensions {
    pub fn new(start: Position, stop: Position, size: Size) -> Self {
        Self { start, stop, size }
    }

    pub fn start(&self) -> Position {
        self.start
    }

    pub fn stop(&self) -> Position {
        self.stop
    } 

    pub fn size(&self) -> Size {
        self.size
    }
}