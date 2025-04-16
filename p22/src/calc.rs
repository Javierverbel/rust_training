/// Funtion converts from celsius to farenheit
/// ```
/// use p22::calc::celsius2farenheit;
/// assert_eq!(celsius2farenheit(0), 32);
/// ```

pub fn celsius2farenheit(celsius: i32) -> i32 {
    (celsius * 9) / 5 + 32
}

/// Funtion converts from farenheit to celsius
/// ```
/// use p22::calc::farenheit2celsius;
/// assert_eq!(farenheit2celsius(32), 0);
/// ```
pub fn farenheit2celsius(farenheit: i32) -> i32 {
    ((farenheit - 32) * 5) / 9
}

/// Funtion returnts the n-th term of the Fibonacci sequence using loops
/// ```
/// use::p22::calc::fibonacci_loop;
/// let x  = fibonacci_loop(12);
/// assert_eq!(x, 144);
/// ```
pub fn fibonacci_loop(n: u32) -> u64 {
    let mut i0 = 0;
    let mut i1: u64 = 1;
    let mut temp = 0;
    if n == 0 {
        return i0;
    } else if n == 1 {
        return i1;
    } else {
        let mut count = 2;
        loop {
            let temp = i1 + i0;
            i0 = i1;
            i1 = temp;
            if count == n {
                break;
            }
            println!("count: {count} temp: {temp}");
            count += 1;
        }
    }
    i1
}

/// Funtion returnts the n-th term of the Fibonacci sequence using loops
/// ```
/// use::p22::calc::fibonacci_rec;
/// let x  = fibonacci_rec(12);
/// assert_eq!(x, 144);
/// ```
pub fn fibonacci_rec(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_rec(n - 1) + fibonacci_rec(n - 2),
    }
}
