use super::*;

#[test]
fn trace_should_be_parsed() -> Result<(), Box<dyn std::error::Error>> {
    let trace = Trace::from_str("07b243a0\n08b24312")?;

    let expected = Trace {
        operations: vec![(0x07b243a0), (0x08b24312)],
    };

    assert_eq!(trace, expected);

    Ok(())
}

#[test]
#[should_panic]
fn trace_should_panic_on_invalid_line() {
    let _ = Trace::from_str("07b243a0R\n08b24312").unwrap();
}
