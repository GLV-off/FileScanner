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
        let Some(description) = self.desc.as_ref() else {
            return Err("name not ready".into());
        };
        let Some(start) = self.start else {
            return Err("start not ready".into());
        };
        let Some(stop) = self.stop else {
            return Err("stop not set".into());
        };
        let Some(size) = self.size else {
            return Err("size not set".into());
        };

        if start > stop {
            return Err(format!("start ({start}) must not exceed stop ({stop})"));
        }

        let expected = i64::from(stop) - i64::from(start);
        if expected != i64::from(size) {
            return Err(format!(
                "size mismatch: stop ({stop}) - start ({start}) = {expected}, but size is {size}"
            ));
        }

        Ok(Section {
            id: SectionID::default(),
            description: description.clone(),
            dimensions: Dimensions::new(start, stop, size),
        })
    }
}

impl Section {
    pub fn builder() -> Builder {
        Builder::default()
    }
}

