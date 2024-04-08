use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");
}

pub fn search<T>(sorted: &Vec<T>, needle: &T) -> Option<usize>
where
    T: PartialEq + Ord,
{
    search_bounds(sorted, needle, 0, sorted.len() - 1)
}

fn search_bounds<T: PartialEq>(xs: &Vec<T>, x: &T, i: usize, j: usize) -> Option<usize>
where
    T: PartialEq + Ord,
{
    if i > j {
        None
    } else {
        let span = j - i;
        let mid = i + span / 2;
        let y = &xs[mid];
        match x.cmp(&y) {
            Ordering::Less => search_bounds(xs, x, i, mid - 1),
            Ordering::Equal => Some(mid),
            Ordering::Greater => search_bounds(xs, x, mid + 1, j),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_small() {
        let numbers = vec![0, 10, 20, 30, 40, 50];
        assert_eq!(Some(0), search(&numbers, &0));
        assert_eq!(Some(4), search(&numbers, &40));
        assert_eq!(Some(5), search(&numbers, &50));
    }

    #[test]
    fn dont_find_small() {
        let numbers = vec![0, 10, 20, 30, 40, 50];
        assert_eq!(None, search(&numbers, &25));
    }

    #[test]
    fn find_big() {
        const SIZE: u32 = 100000;
        let mut numbers: Vec<u32> = Vec::new();
        for i in 0..=SIZE {
            numbers.push(i);
        }
        assert_eq!(Some(0), search(&numbers, &0));
        assert_eq!(Some(SIZE as usize / 3), search(&numbers, &(SIZE / 3)));
        assert_eq!(Some(SIZE as usize), search(&numbers, &SIZE));
    }

    #[test]
    fn dont_find_big() {
        const SIZE: u32 = 100000;
        let mut numbers: Vec<u32> = Vec::new();
        for i in 0..=SIZE {
            numbers.push(i * 2);
        }
        assert_eq!(None, search(&numbers, &(SIZE / 2 + 1)));
    }
}
