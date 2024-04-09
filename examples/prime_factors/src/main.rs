fn main() {
    let n = 100;
    println!("primes up to {n}: {:?}", prime_sieve(n));
}

fn prime_sieve(n: u32) -> Vec<u32> {
    let mut primes: Vec<u32> = Vec::new();
    let n_sqrt: u32 = (n as f32).floor() as u32;
    for i in 2..=n {
        if !primes.iter().filter(|&x| x <= &n_sqrt).any(|x| i % x == 0) {
            primes.push(i);
        }
    }
    primes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primes_up_to_10() {
        assert_eq!(prime_sieve(20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
    }
}
