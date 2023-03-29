// errors4.rs
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a hint.



#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// impl PositiveNonzeroInteger {
//     fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
//         // Hmm...? Why is this only returning an Ok value?
//         Ok::<PositiveNonzeroInteger, E>(PositiveNonzeroInteger(value as u64));
//         match value {
//             x if x < 0 => Err(CreationError::Negative),
//         }
//     }
// }


impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            value => Ok(PositiveNonzeroInteger(value as u64)),
        }
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
