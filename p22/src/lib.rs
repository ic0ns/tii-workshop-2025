//Define modules
pub mod figures;
pub mod tictac;
pub mod borrowing;
pub mod calc {

    /// Returns the Fahrenheit equivalent of the given Celsius temperature.
    /// # Arguments
    /// * `celsius` - The temperature in Celsius.
    ///
    /// # Returns
    /// * The temperature in Fahrenheit.
    ///
    /// # Examples
    ///
    /// ```
    /// use p22::calc::celsius2fahrenheit;
    /// let temp = celsius2fahrenheit(0);
    /// assert_eq!(temp, 32);
    /// ```
    pub fn celsius2fahrenheit(celsius: i32) -> i32 {
        (celsius * 9 / 5) + 32
    }

    /// Returns the Celsius equivalent of the given Fahrenheit temperature.
    /// # Arguments
    /// * `farenheit` - The temperature in Fahrenheit.
    ///
    /// # Returns
    /// * The temperature in Celsius.
    ///
    /// # Examples
    ///
    /// ```
    /// use p22::calc::fahrenheit2celsius;
    /// let temp = fahrenheit2celsius(32);
    /// assert_eq!(temp, 0);
    /// ```
    pub fn fahrenheit2celsius(farenheit: i32) -> i32 {
        (farenheit - 32) * 5 / 9
    }

    /// Returns the nth Fibonacci number using a loop.
    /// # Arguments
    /// * `n` - The position in the Fibonacci sequence.
    ///     
    /// # Returns
    /// * The nth Fibonacci number.
    ///
    /// # Examples
    ///
    /// ```
    /// use p22::calc::fibonacci_loop;
    /// let fib = fibonacci_loop(5);
    /// assert_eq!(fib, 5);
    /// ```
    pub fn fibonacci_loop(n: u32) -> u32 {
        let mut a = 0;
        let mut b = 1;
        for _ in 0..n {
            let temp = a;
            a = b;
            b += temp;
        }
        a
    }

    /// Returns the nth Fibonacci number using recursion.
    /// # Arguments
    /// * `n` - The position in the Fibonacci sequence.
    ///
    /// # Returns
    /// * The nth Fibonacci number.
    ///
    /// # Examples
    ///     
    /// ```
    /// use p22::calc::fibonacci_rec;
    /// let fib = fibonacci_rec(5);
    /// assert_eq!(fib, 5);
    /// ```
    pub fn fibonacci_rec(n: u32) -> u32 {
        if n == 0 {
            0
        } else if n == 1 {
            1
        } else {
            fibonacci_rec(n - 1) + fibonacci_rec(n - 2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::calc::*;

    #[test]
    fn test_celsisus2farenheit() {
        assert_eq!(celsius2fahrenheit(0), 32);
        assert_eq!(celsius2fahrenheit(100), 212);
        assert_eq!(celsius2fahrenheit(-40), -40);
    }

    #[test]
    fn test_farenheit2celsisus() {
        assert_eq!(fahrenheit2celsius(32), 0);
        assert_eq!(fahrenheit2celsius(212), 100);
        assert_eq!(fahrenheit2celsius(-40), -40);
    }

    #[test]
    fn test_fibonacci_loop() {
        assert_eq!(fibonacci_loop(0), 0);
        assert_eq!(fibonacci_loop(1), 1);
        assert_eq!(fibonacci_loop(2), 1);
        assert_eq!(fibonacci_loop(3), 2);
        assert_eq!(fibonacci_loop(4), 3);
        assert_eq!(fibonacci_loop(5), 5);
        assert_eq!(fibonacci_loop(6), 8);
    }

    #[test]
    fn test_fibonacci_rec() {
        assert_eq!(fibonacci_rec(0), 0);
        assert_eq!(fibonacci_rec(1), 1);
        assert_eq!(fibonacci_rec(2), 1);
        assert_eq!(fibonacci_rec(3), 2);
        assert_eq!(fibonacci_rec(4), 3);
        assert_eq!(fibonacci_rec(5), 5);
        assert_eq!(fibonacci_rec(6), 8);
    }

    #[test]
    fn test_fibonacci_rec_large() {
        assert_eq!(fibonacci_rec(10), 55);
        assert_eq!(fibonacci_rec(20), 6765);
        assert_eq!(fibonacci_rec(30), 832040);
    }
}
