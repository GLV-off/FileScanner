#[cfg(test)]
mod test;

use super::dimension::{Position,  Size, Dimensions};

pub type SectionID = String;
pub type SectionDescription = String;

pub struct Section {
    id: SectionID,
    description: SectionDescription,
    dimensions: Dimensions
}

#[derive(Default)]
pub struct Builder {
    desc: Option<SectionDescription>,
    start: Option<Position>,
    stop: Option<Position>,
    size: Option<Size>
}

type BuilderResult = Result<Section, String>;

impl Builder {
    pub fn new() -> Self {
        Self {
            desc: None, 
            start: None,
            stop: None,
            size: None
        }
    }

    pub fn name(&mut self, name: SectionDescription) ->  &mut Self {
        self.desc = Some(name);
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
        if self.desc.is_none() {
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
                    id: SectionID::default(),
                    description: self.desc.clone().unwrap(),
                    dimensions: Dimensions::new(
                        self.start.unwrap(), 
                        self.stop.unwrap(),
                        self.size.unwrap()
                    )
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

