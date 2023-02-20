#[cfg(test)]
mod tests;

mod tlb;

use rand::seq::index;
use rand::thread_rng;

use tlb::Tlb;

#[derive(Debug)]
pub struct Mmu<const M: usize, const N: usize, const T: usize>
where
    [usize; usize::pow(2, M as u32) / usize::pow(2, N as u32)]: Sized,
{
    page_table: Box<[usize; usize::pow(2, M as u32) / usize::pow(2, N as u32)]>, // a page table with U entries (implies a 2^U page size)
    tlb: Tlb<T>, // a TLB with V entries
}

impl<const M: usize, const N: usize, const T: usize> Mmu<M, N, T>
where
    [usize; usize::pow(2, M as u32) / usize::pow(2, N as u32)]: Sized,
{
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let page_table: Box<[usize; usize::pow(2, M as u32) / usize::pow(2, N as u32)]> =
            index::sample(
                &mut rng,
                usize::pow(2, M as u32),
                usize::pow(2, M as u32) / usize::pow(2, N as u32),
            )
            .into_vec()
            .try_into()
            .unwrap();
        // let page_table: Box<[usize; usize::pow(2, M as u32) / usize::pow(2, N as u32)]> =
        //     box [0; usize::pow(2, M as u32) / usize::pow(2, N as u32)];

        let tlb = Tlb::<T>::new();

        Self { page_table, tlb }
    }

    pub fn translate(&mut self, address: &usize) -> AddressTranslationResult {
        // -----------------------
        // | page_number | offset |
        // -----------------------
        // |      M-N    |    N   |
        // -----------------------
        //        Msb -> Lsb
        let page_number = address >> N;
        let offset = address & (usize::pow(2, N as u32) - 1);

        match self.tlb.lookup(page_number) {
            Some(frame_number) => {
                AddressTranslationResult::Hit(frame_number * usize::pow(2, N as u32) + offset)
            }
            None => {
                let frame_number = self.page_table[page_number];

                self.tlb.update(page_number, frame_number);
                AddressTranslationResult::Miss(frame_number * usize::pow(2, N as u32) + offset)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum AddressTranslationResult {
    Hit(usize),
    Miss(usize),
}
