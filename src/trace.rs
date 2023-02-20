use std::str::FromStr;

#[cfg(test)]
mod tests;

pub type Address = usize;
pub type MemoryOperation = (Address, MemoryOperationKind);

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MemoryOperationKind {
    Read,
    Write,
}

#[derive(Debug, PartialEq)]
pub struct Trace {
    operations: Vec<MemoryOperation>,
}

impl Trace {
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }
    pub fn operations(&self) -> impl Iterator<Item = &MemoryOperation> {
        self.operations.iter()
    }

}

impl FromStr for Trace {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let operations = s
            .lines()
            .map(|line| {
                // 07b243a0 R for example
                match line.split_whitespace().collect::<Vec<_>>()[..] {
                    [address, ty] => {
                        let address = usize::from_str_radix(address, 16).unwrap();
                        let ty = match ty {
                            "R" => MemoryOperationKind::Read,
                            "W" => MemoryOperationKind::Write,
                            _ => panic!("Unknown operation type"),
                        };

                        (address, ty)
                    }
                    _ => panic!("Invalid trace line"),
                }
            })
            .collect();

        Ok(Trace { operations })
    }
}
