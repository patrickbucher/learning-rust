use std::env;

fn main() {
    let mut args = env::args();
    args.next().expect("too few arguments provided");
    let x: u32 = args
        .next()
        .expect("too few arguments provided")
        .parse()
        .expect("no positive integer provided");
    let result = prime_factors_fast(x);
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

fn prime_factors_fast(n: u32) -> Vec<u32> {
    let mut sieve = PrimeSieve::new();
    let mut factors: Vec<u32> = Vec::new();
    let mut x = n;
    let mut prime = sieve.next().expect("running out of prime numbers");
    while x > 1 {
        if x % prime == 0 {
            x /= prime;
            factors.push(prime);
        } else {
            prime = sieve.next().expect("running out of prime numbers");
        }
    }
    factors
}

struct PrimeSieve {
    i: u32,
    cache: Vec<u32>,
}

impl PrimeSieve {
    fn new() -> PrimeSieve {
        PrimeSieve {
            i: 2,
            cache: Vec::new(),
        }
    }
}

impl Iterator for PrimeSieve {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut j = 2;
        loop {
            if self.cache.iter().any(|x| j % x == 0) {
                j += 1;
            } else {
                self.cache.push(j);
                self.i = j;
                return Some(j);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primes_up_to_20() {
        assert_eq!(prime_sieve(20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
    }

    #[test]
    fn prime_sieve_up_to_20() {
        let mut sieve = PrimeSieve::new();
        assert_eq!(sieve.next(), Some(2));
        assert_eq!(sieve.next(), Some(3));
        assert_eq!(sieve.next(), Some(5));
        assert_eq!(sieve.next(), Some(7));
        assert_eq!(sieve.next(), Some(11));
        assert_eq!(sieve.next(), Some(13));
        assert_eq!(sieve.next(), Some(17));
        assert_eq!(sieve.next(), Some(19));
    }

    #[test]
    fn prime_factors_136() {
        assert_eq!(prime_factors(136), vec![2, 2, 2, 17]);
    }

    #[test]
    fn prime_factors_fast_136() {
        assert_eq!(prime_factors_fast(136), vec![2, 2, 2, 17]);
    }
}
