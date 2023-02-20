use super::*;

#[test]
fn tlb_lookup() {
    let mut tlb = Tlb::<4>::new();

    tlb.update(0, 0);
    tlb.update(1, 1);
    tlb.update(2, 2);
    tlb.update(3, 3);

    assert_eq!(tlb.lookup(0), Some(0));
    assert_eq!(tlb.lookup(1), Some(1));
    assert_eq!(tlb.lookup(2), Some(2));
    assert_eq!(tlb.lookup(3), Some(3));

    tlb.update(4, 4);

    assert_eq!(tlb.lookup(0), None);
    assert_eq!(tlb.lookup(1), Some(1));
    assert_eq!(tlb.lookup(2), Some(2));
    assert_eq!(tlb.lookup(3), Some(3));
    assert_eq!(tlb.lookup(4), Some(4));
}

