pub fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut fib_n = 0;
    let mut fib_n1 = 1;
    let mut fib_n2 = 1;

    for _i in 2..=n {
        fib_n2 = fib_n + fib_n1;
        fib_n = fib_n1;
        fib_n1 = fib_n2;
    }

    fib_n2
}