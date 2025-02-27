use rust_tdd::{Bank, Money};

#[test]
fn multiple_test() {
    let dollar_5 = Money::new("dollar", 5);
    assert_eq!(dollar_5.times(2), Money::new("dollar", 10));
    assert_eq!(dollar_5.times(3), Money::new("dollar", 15));

    let franc_5 = Money::new("franc", 5);
    assert_eq!(franc_5.times(2), Money::new("franc", 10));
    assert_eq!(franc_5.times(3), Money::new("franc", 15));
}

#[test]
fn equal_test() {
    let dollar_5 = Money::new("dollar", 5);
    assert!(dollar_5.equals(Money::new("dollar", 5)));
    assert!(!dollar_5.equals(Money::new("dollar", 7)));
    let franc_5 = Money::new("franc", 5);
    assert_ne!(dollar_5, franc_5);
}

#[test]
fn currency_test() {
    assert_eq!("USD", Money::new("dollar", 1).currency());
    assert_eq!("CHF", Money::new("franc", 1).currency());
}

#[test]
fn simple_addition_test() {
    let dollar_5 = Money::new("dollar", 5);
    let sum = dollar_5.plus(Money::new("dollar", 5));
    let bank = Bank::new();
    let reduced = bank.reduce(sum, "USD");
    assert_eq!(Money::new("dollar", 10), reduced);
}
