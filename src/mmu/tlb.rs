#[cfg(test)]
mod tests;

type TlbEntry = Option<(usize, usize)>; // (page_number, frame_number)

#[derive(Debug, PartialEq)]
pub struct Tlb<const T: usize> {
    pub entries: [TlbEntry; T],
    pub idx: usize,
}

impl<const T: usize> Tlb<T> {
    pub fn new() -> Self {
        Self {
            entries: [None; T],
            idx: 0,
        }
    }

    pub fn lookup(&self, page_number: usize) -> Option<usize> {
        self.entries.iter().find_map(|entry| match entry {
            Some((page, frame)) if *page == page_number => Some(*frame),
            _ => None,
        })
    }

    pub fn update(&mut self, page_number: usize, frame_number: usize) {
        self.entries[self.idx] = Some((page_number, frame_number));
        self.idx = (self.idx + 1) % T;
    }

    // pub fn clear(&mut self) {
    //     self.entries = [None; T];
    //     self.idx = 0;
    // }
}
