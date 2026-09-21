#[cfg(test)]
mod test;

pub type Position = i32;
pub type SectionName = String;
pub type Size = i32;

pub struct Section {
    name: SectionName,
    start: Position,
    stop: Position,
    size: Size
}

pub type Sections = Vec<Section>;

#[derive(Default)]
pub struct Builder {
    name: Option<SectionName>,
    start: Option<Position>,
    stop: Option<Position>,
    size: Option<Size>
}

type BuilderResult = Result<Section, String>;

impl Builder {
    pub fn new() -> Self {
        Self {
            name: None, 
            start: None,
            stop: None,
            size: None
        }
    }

    pub fn name(&mut self, name: SectionName) ->  &mut Self {
        self.name = Some(name);
        self
    }

    pub fn start(&mut self, start: Position) -> &mut Self {
        self.start = Some(start);
        self
    }

    pub fn stop(&mut self, stop: Position) -> &mut Self {
        self.stop = Some(stop);
        self
    }

    pub fn size(&mut self, size: Size) -> &mut Self {
        self.size = Some(size);
        self
    }

    pub fn build(&mut self) -> BuilderResult {
        if self.name.is_none() {
            Err("name not ready".into())
        } else if self.start.is_none() {
            Err("start not ready".into())
        } else if self.stop.is_none() {
            Err("none not set".into())
        } else if self.size.is_none() {
            Err("size not set".into())
        } else {
            Ok(
                Section {
                    name: self.name.clone().unwrap(),
                    start: self.start.unwrap(),
                    stop: self.stop.unwrap(),
                    size: self.size.unwrap()
                }
            )
        }        
    }
}

impl Section {
    pub fn builder() -> Builder {
        Builder::default()
    }
}

