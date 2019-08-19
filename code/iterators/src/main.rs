struct Counter {
    count: u32,
    limit: u32,
}

impl Counter {
    fn new(limit: u32) -> Counter {
        Counter {
            count: 0,
            limit: limit,
        }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count <= self.limit {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let c1to5 = Counter::new(5);
    let c1to7 = Counter::new(7);
    let sum: u32 = c1to5
        .zip(c1to7.skip(2))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    println!("{}", sum);
}
