use super::*;
#[test]
fn mmu_should_be_initialized() {
    let mmu = Mmu::<4, 2, 2>::new(); // a 4 bit architecture with 2^2 page size and 2^2 TLB size
    let tlb = Tlb::<2>::new();

    assert_eq!(mmu.page_table.len(), 4);
    assert_eq!(mmu.tlb, tlb);
}

#[test]
fn mmu_should_translate_address() {
    // examples taken from OS concepts 10th edition @ 9.6
    let mut mmu: Mmu<4, 2, 2> = Mmu {
        page_table: Box::new([5, 6, 1, 2]),
        tlb: Tlb {
            entries: [None; 2],
            idx: 0,
        },
    };

    assert_eq!(mmu.translate(&0b0000), AddressTranslationResult::Miss(20));

    assert_eq!(mmu.translate(&0b0001), AddressTranslationResult::Hit(21));

    assert_eq!(mmu.translate(&0b0110), AddressTranslationResult::Miss(26));

    assert_eq!(mmu.translate(&0b1111), AddressTranslationResult::Miss(11));

    assert_eq!(mmu.translate(&0b0000), AddressTranslationResult::Miss(20));
}
