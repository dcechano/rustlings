// errors4.rs
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a hint.



#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // Hmm...? Why is this only returning an Ok value?
        let rslt: Result<PositiveNonzeroInteger, CreationError>;
        if value == 0 {
            rslt = Err(CreationError::Zero);
        } else if value < 0 {
            rslt = Err(CreationError::Negative);
        } else {
            rslt = Ok(PositiveNonzeroInteger(value as u64));
        };
        rslt
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
