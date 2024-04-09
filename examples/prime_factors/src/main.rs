use std::env;

fn main() {
    let mut args = env::args();
    args.next().expect("too few arguments provided");
    let x: u32 = args
        .next()
        .expect("too few arguments provided")
        .parse()
        .expect("no positive integer provided");
    let result = prime_factors(x);
    let output = result
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    println!("{x}: {output}");
}

fn prime_sieve(n: u32) -> Vec<u32> {
    let n_sqrt: u32 = (n as f32).floor() as u32;
    let mut primes: Vec<u32> = Vec::new();
    for i in 2..=n {
        if !primes.iter().filter(|&x| x <= &n_sqrt).any(|x| i % x == 0) {
            primes.push(i);
        }
    }
    primes
}

fn prime_factors(n: u32) -> Vec<u32> {
    let n_sqrt: u32 = (n as f32).floor() as u32;
    let primes = prime_sieve(n_sqrt);
    let mut x = n;
    let mut i = 0;
    let mut factors: Vec<u32> = Vec::new();
    while x > 1 {
        if x % primes[i] == 0 {
            x = x / primes[i];
            factors.push(primes[i]);
        } else {
            i += 1;
        }
    }
    factors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primes_up_to_10() {
        assert_eq!(prime_sieve(20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
    }

    #[test]
    fn prime_factors_136() {
        assert_eq!(prime_factors(136), vec![2, 2, 2, 17]);
    }
}
