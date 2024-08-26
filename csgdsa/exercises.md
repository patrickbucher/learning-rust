# Chapter 1

## 1)

1. 1
2. N
3. N+1
4. 1, if there is capacity left; N+1, if the array needs to be resized, and,
   thus, copied first.
5. N: 1 for the deletion, N-1 for the shift left
6. 1

## 2)

1. 1
2. N
3. 2N+1: N for searching, N for moving, 1 for inserting
4. N+1: N for searching, 1 for inserting (if there is capacity left), or 2N+1,
   if the array needs to be resized/copied first
5. N: 1 for deletion, N-1 left shifts
6. 1

## 3)

It would take N steps.

# Chapter 2

## 1)

linear search for 8 in [2, 4, 6, 8, 10, 12, 13]

Answer: four steps

## 2)

binary search for 8 in [2, 4, 6, 8, 10, 12, 13]

| step | lower | upper | midpoint | value |
|-----:|------:|------:|---------:|------:|
|    1 |     0 |     6 |        3 |     8 |

Answer: one single step.

## 3)

Answer: log2(100000) = 16.61 => max. 17 steps (actually 16 steps)

# Chapter 3

## 1)

O(1)

## 2)

O(N)

## 3)

O(log N)

## 4)

O(N)

## 5)

O(1)

# Chapter 4

## 1)

Assuming `log` as `log10`:

| N Elements | O(N) | O(log N) | O(N²) |
|-----------:|-----:|---------:|------:|
|        100 |  100 | ~~10~~ 7 | 10000 |
|       2000 | 2000 |      7.6 |   4e6 |

`log2(2000)=~11`

## 2)

The array is of size 16 (the square root of 256).

## 3)

The complexity is O(N²) because of the nested loops, of which each iterates
through the entire array.

## 4)

see `src/misc.rs` (function `find_greatest`)

# Chapter 5

## 1)

4N + 16 => O(N)

## 2)

2N² => O(N²)

## 3)

N + N => O(N)

## 4)

3N => O(N)

## 5)

N/2 * N = N²/2 => O(N²)

# Chapter 6

## 1)

3N² + 2N + 1 => O(N²)

## 2)

N + log N => O(N)

## 3)

- Best Case: The first two elements add up to 10.
- Average Case: A matching pair is roughly found around the middle.
- Worst Case: No matching pair is found after; O(N²).

## 4)

The time complexity is O(N). Instead of setting `foundX` to `true`, the function
could just return with `true`.

# Chapter 7

## 1)

O(N)

## 2)

O(N+M), with N being the length of the first and M being the length of the
second array.

## 3)

O(N * M), with N being the size of the haystack and M being the length of the
needle.

## 4)

O(N³)

## 5)

O(log N)

# Chapter 8

## 1)

see `src/misc.rs` (function `intersect`)

## 2)

see `src/misc.rs` (function `find_first_duplicate`)

## 3)

see `src/misc.rs` (function `find_missing_alphabet_letter`)

## 4)

see `src/misc.rs` (function `find_first_unique_letter`)

# Chapter 9

## 1)

A queue; because "first come, first served" and FIFO are the same principle.

## 2)

The number 4 would be on the top.

## 3)

The number 3 would be at the front.

## 4)

see `src/stack.rs` (function `reverse`)

# Chapter 10

## 1)

Base Case: `low > high`

## 2)

    factorial(10)
    10 * factorial(8)
    10 * 8 * factorial(6)
    10 * 8 * 6 * factorial(4)
    10 * 8 * 6 * 4 * factorial(2)
    10 * 8 * 6 * 4 * 2 * factorial(0)
    …

Since the base case checks for equality of `n` with `1`, the function never
terminates.

## 3)

```ruby
def sum(low, high)
  return 0 if low > high
  return high + sum(low, high - 1)
end
```

## 4)

```python
array = [1, 2, 3, [4, 5, 6], 7, [8, [9, 10, 11, [12, 13, 14]]], [
    15, 16, 17, 18, 19, [20, 21, 22, [23, 24, 25, [26, 27, 29]], 30, 31], 32], 33]


def print_numbers(arr):
    for item in arr:
        if type(item) == type(0):
            print(item)
        else:
            print_numbers(item)


print_numbers(array)
```

# Chapter 11

## 1)

see `src/recursion.rs` (function `charcount`)

## 2)

see `src/recursion.rs` (function `filter_even`)

## 3)

see `src/recursion.rs` (function `triangular_numbers`)

## 4)

see `src/recursion.rs` (function `find_first_index_of`)

## 5)

see `src/recursion` (function `find_unique_paths`)

# Chapter 12

## 1)

see `src/recursion.rs` (function `add_until`)

## 2)

see `src/recursion.rs` (function `golomb`)

## 3)

see `src/recursion.rs` (function `find_unique_paths`)
