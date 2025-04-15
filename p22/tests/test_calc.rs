#[test]
fn test_celsius2farenheit_2degree_c_to_f() {
    use p22::calc::celsius2farenheit;
    assert_eq!(celsius2farenheit(2), 35);
}

#[test]
fn test_farenheit2celsius_35degree_f_to_c() {
    use p22::calc::farenheit2celsius;
    assert_eq!(farenheit2celsius(35), 1);
}

#[test]
fn test_fibonnaci_loop() {
    use p22::calc::fibonacci_loop;
    assert_eq!(fibonacci_loop(15), 610);
}

#[test]
fn test_fibonnaci_rec() {
    use p22::calc::fibonacci_rec;
    assert_eq!(fibonacci_rec(15), 610);
}
