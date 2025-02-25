use rust_tdd::{Dollar, Franc, Money};

#[test]
fn multiple_test() {
    let dollar_5 = Dollar::new(5);
    assert_eq!(Dollar::new(10), dollar_5.multiple(2));
    assert_eq!(Dollar::new(15), dollar_5.multiple(3));

    let franc_5 = Franc::new(5);
    assert_eq!(Franc::new(10), franc_5.multiple(2));
    assert_eq!(Franc::new(15), franc_5.multiple(3));
}

#[test]
fn equal_test() {
    let dollar_5 = Dollar::new(5);
    assert!(dollar_5.equals(Dollar::new(5)));
    assert!(!dollar_5.equals(Dollar::new(7)));

    let franc_5 = Franc::new(5);
    assert!(franc_5.equals(Franc::new(5)));
    assert!(!franc_5.equals(Franc::new(7)));
}
