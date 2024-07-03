use std::cmp::Ordering;

/// Searches for needle in the sorted haystack, returns the index, or None, if the needle cannot be
/// found in the haystack.
///
/// ```
/// use grokking_algorithms::binary_search::search;
/// let haystack = vec![2, 4, 6, 8, 10];
/// assert_eq!(search(&haystack, &4), Some(1));
/// assert_eq!(search(&haystack, &5), None);
/// ```
pub fn search<T: Ord>(haystack: &Vec<T>, needle: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = haystack.len() - 1;
    while low <= high {
        let mid = (low + high) / 2;
        match needle.cmp(&haystack[mid]) {
            Ordering::Less => high = mid - 1,
            Ordering::Greater => low = mid + 1,
            Ordering::Equal => return Some(mid),
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_miss() {
        let haystack = vec![6, 12, 18, 24, 30, 36, 42, 48, 54, 60];
        let needle = 25;
        let actual = search(&haystack, &needle);
        assert_eq!(actual, None);
    }

    #[test]
    fn test_search_hit() {
        let haystack: Vec<u8> = (1..100).collect();
        let needle: u8 = 23;
        let actual = search(&haystack, &needle);
        assert_eq!(needle, haystack[actual.unwrap()]);
    }

    #[test]
    fn test_search_big_hit() {
        let haystack: Vec<u32> = (0..1_000_000).collect();
        let needle: u32 = 272341;
        let actual = search(&haystack, &needle);
        assert_eq!(needle, haystack[actual.unwrap()]);
    }
}
