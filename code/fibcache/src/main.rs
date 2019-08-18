use std::collections::HashMap;

struct Cache<T>
where
    T: Fn(u32) -> u32,
{
    operation: T,
    values: HashMap<u32, u32>,
}

impl<T> Cache<T>
where
    T: Fn(u32) -> u32,
{
    fn new(operation: T) -> Cache<T> {
        Cache {
            operation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.values.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.operation)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}

fn main() {
    let fib = |n| {
        println!("called fib closure");
        if n < 2 {
            1
        } else {
            let mut prev1 = 0;
            let mut prev2 = 1;
            for _i in 0..n {
                let tmp = prev1 + prev2;
                prev1 = prev2;
                prev2 = tmp;
            }
            prev2
        }
    };
    let mut cache = Cache::new(fib);
    println!("{}", cache.value(9));
    println!("{}", cache.value(10));
    println!("{}", cache.value(9));
    println!("{}", cache.value(10));
}
