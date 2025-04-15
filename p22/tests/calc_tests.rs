use p22::calc;

#[test]
fn test_functionality() {
    assert_eq!(calc::celsius2fahrenheit(0), 32);
    assert_eq!(calc::celsius2fahrenheit(100), 212);
    assert_eq!(calc::celsius2fahrenheit(-40), -40);
    assert_eq!(calc::fahrenheit2celsius(32), 0);
    assert_eq!(calc::fahrenheit2celsius(212), 100);
    assert_eq!(calc::fahrenheit2celsius(-40), -40);
    assert_eq!(calc::fibonacci_loop(0), 0);
    assert_eq!(calc::fibonacci_loop(1), 1);
    assert_eq!(calc::fibonacci_loop(2), 1);
    assert_eq!(calc::fibonacci_loop(3), 2);
    assert_eq!(calc::fibonacci_loop(4), 3);
    assert_eq!(calc::fibonacci_loop(5), 5);

    assert_eq!(calc::fibonacci_rec(5), 5);
    assert_eq!(calc::fibonacci_rec(6), 8);
    assert_eq!(calc::fibonacci_rec(7), 13);
    assert_eq!(calc::fibonacci_rec(8), 21);
}
