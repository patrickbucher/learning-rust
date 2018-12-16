fn main() {
    for i in 1..45 {
        println!("fib({})={}", i, fib(i));
    }
}

fn fib(n: u32) -> u64 {
    if n == 1 || n == 2 {
        1
    } else {
        fib(n - 2) + fib(n - 1)
    }
}
