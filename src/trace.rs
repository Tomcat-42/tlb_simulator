use std::str::FromStr;

#[cfg(test)]
mod tests;

pub type Address = usize;

#[derive(Debug, PartialEq)]
pub struct Trace {
    operations: Vec<Address>,
}

impl Trace {
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }
    pub fn operations(&self) -> impl Iterator<Item = &Address> {
        self.operations.iter()
    }
}

impl FromStr for Trace {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let operations = s
            .lines()
            .map(|line| {
                usize::from_str_radix(line.trim(), 16)
                    .expect("Address cannot be parseable to usize")
            })
            .collect::<Vec<usize>>();

        Ok(Trace { operations })
    }
}
