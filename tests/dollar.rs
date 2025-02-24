use rust_tdd::Dollar;

#[test]
fn multiple_test() {
    let dollar_5 = Dollar::new(5);
    assert_eq!(5, dollar_5.amount);
    let dollar_10 = dollar_5.multiple(2);
    assert_eq!(10, dollar_10.amount);
    let dollar_15 = dollar_5.multiple(3);
    assert_eq!(15, dollar_15.amount);
}

#[test]
fn equal_test() {
    let dollar_5 = Dollar::new(5);
    assert!(dollar_5.equals(Dollar::new(5)));
    assert!(!dollar_5.equals(Dollar::new(7)));
}
