# Introduction to Algorithms

## 1.1

`log2(128) = 7`

## 1.2

For O(log n), the number of operations just increases by one as the input size
doubles.

`log2(2*128) = log2(256) = 8`

## 1.3

O(n) for linear search, O(log n) for binary search

## 1.4

O(n) for linear search

## 1.5

O(n) for linear search

## 1.6

O(n) for linear search; even though 'A' roughly covers 1/26 of the alphabet, the
constant factor 1/26 isn't relevant for the algorithm's order.

# Selection Sort

## 2.1

Since there are a lot of inserts and only a few reads, a linked list with O(1)
for inserting and O(n) for reading is more efficient than an array with O(n) for
inserting and O(1) for reading.

## 2.2

In a queue, items are inserted at one and and deleted at the other end. There is
no random access needed, therefore a linked list with constant time for
insertion and deletion is the better choice than an array.

## 2.3

The list of usernames should be managed in an array, because more random access
than insertion operations are performed on this list.

## 2.4

The array of usernames has to be sorted when a username is inserted, or the
username has to be inserted in the proper order.

## 2.5

The right slot for inserting a username can be found in constant time. The
insertion itself can also happen in constant time (unordered) or in linear time
(ordered).

Finding the right slot for searching a username also happens in constant time.
Finding the username itself then happens in linear time.

Since this hybrid data structure splits the problem in 26 sub-problems of
unequal size, it will probably perform faster than either an array or a linked
list containing all the usernames. However, since the speedup for the first step
by 1/26 is a constant factor, the data structure will perform more or less the
same as a linked list as n gets bigger.

# Recursion

## 3.1

A function `greet` was called with the `name` parameter set to `"maggie"`. This
function called another function `greet2`, also with the `name` parameter set to
`"maggie"`.

## 3.2

The stack will be built up until the process runs out of stack space. The
program will then be terminated by the operating system.

## 3.2

# Quicksort

## 4.1

See the `sum` function in `src/recursion.rs`.

## 4.2

See the `count` function in `src/recursion.rs`.

## 4.3

See the `max` function in `src/recursion.rs`.

## 4.4

See the `binary_search` function in `src/recursion.rs`.

The _first_ base case is an empty list, in which case there is no result.

The _second_ base case is a list consisting of a single element, which either is
the proper value or another value.

The recursive case is the sublist from either low to mid or mid to high,
depending on how the searched value compares to the value in the middle.

## 4.5

O(n)

## 4.6

O(n)

## 4.7

O(1)

## 4.8

O(n²)

# Hash Tables

## 5.1

This is consistent, because the hash for the same value will always be 1.

## 5.2

This is not consistent, because a random value will likely change on the next
invocation.

## 5.3

This is not consistent, because the index for the next empty slot may change
between invocations.

## 5.4

This is consistent, because the length of a string stays the same.

## 5.5

- 1) is always bad.
- 2) is not good, because the names have similar lengths.
- 3) is not so good, because initial letters aren't evenly distributed.
- 4) looks quite decent, because all the letters are considered.

## 5.6

- 1) is always bad.
- 2) is good, because all the keys are distinct.
- 3) is not good, because there is always the same starting letter.
- 4) this should work, because the length of the string will also be considered.

## 5.7

- 1) is always bad.
- 2) is not good, because the titles have similar lenghts.
- 3) is not so good, because initial letters aren't evenly distributed.
- 4) looks quite decent, because all the letters are considered.

# Breadth-First Search

## 6.1

- Start
    - [up]
        - [right]
            - Finish
        - [down]
    - [down]
        - [up]
        - [right]
            - [up]
                - Finish

Start -> up -> right = Finish (two steps)

## 6.2

- CAB [Start]
    - CAT
        - MAT
        - BAT [Finish]
    - CAR
        - CAT
        - BAR

CAB -> CAT -> BAT = Fiish (two steps)

## 6.3

- A) invalid, because "brush teet" must come before "eat breakfast"
- B) valid
- C) invalid, because "wake up" must be the first step

## 6.4

- wake up
- pack lunch
- brush teeth
- exercise
- eat breakfast
- shower
- get dressed

## 6.5

A and C are also trees.

# Trees

_there are no exercises for this chapter_

# Balanced Trees

_there are no exercises for this chapter_

# Dijkstra's Algorithm

1. Begin at the start node.
2. Add the costs of the outnodes to the costs table.
3. Add the start node to the processed set.
4. Pick the node from the cost table with the lowest weight that is not in the
   processed set.
5. Set the parent of that node as the start node.
6. Continue with step 2, until the finish node is the cheapest unprocessed node.
7. Build the route by following the parents table from finish to start.

## 9.1

Costs:

| Node   | Weight |
|--------|-------:|
| Start  |      0 |
| A      |      5 |
| B      |      2 |
| C      |      9 |
| D      |      7 |
| Finish |      8 |

Parents:

| Node   | Parent |
|--------|--------|
| A      | Start  |
| B      | Start  |
| D      | A      |
| C      | A      |
| Finish | D      |

Processed:

| Node  |
|-------|
| Start |
| A     |
| B     |
| D     |

Solution: Start -> A -> D -> Finish in 8

## 9.2

Costs:

| Node   | Weight |
|--------|-------:|
| Start  |      0 |
| A      |     10 |
| B      |     30 |
| C      |     31 |
| Finish |     60 |

Parents:

| Node   | Parent |
|--------|--------|
| A      | Start  |
| B      | A      |
| C      | B      |
| Finish | B      |

Processed:

| Node  |
|-------|
| Start |
| A     |
| B     |
| C     |

Solution: Start -> A -> B -> Finish in 60

## 9.3

Costs:

| Node   | Weight |
|--------|-------:|
| Start  |      0 |
| A      |      2 |
| B      |      2 |
| C      |      4 |
| Finish |      4 |

Parents:

| Node   | Parent |
|--------|--------|
| A      | Start  |
| B      | Start  |
| C      | B      |
| Finish | B      |

Processed:

| Node  |
|-------|
| Start |
| A     |
| B     |

Solution: Start -> B -> Finish in 4

# Greedy Algorithms

## 10.1

Load the truck with boxes ordered in descending order of their size. If the next
box doesn't fit, try the next smaller one.

This might not give the best possible solution, because a big box could have a
shape such that other boxes with a better combined payoff wouldn't fit.

## 10.2

Visit the places in descending order of their point value until the holiday is
over.

This might be a bad strategy, because the highly valued places are very far
apart, requiring a lot of travelling time in between, whereas relativly highly
valued places could be rather close to one another.

# Dynamic Programming

## 11.1

~~No, for one pound, there are better deals: the guitar, the iPhone.~~

Yes, together with the iPhone and the Guitar, one could steal USD 4500.-

## 11.2

Items:

| Item   | Weight (lb) | Value |
|--------|------------:|------:|
| Water  |           3 |    10 |
| Book   |           1 |     3 |
| Food   |           2 |     9 |
| Jacket |           2 |     5 |
| Camera |           1 |     6 |

Formula:

    cell[i][j] = max of 1) or 2)

    1) the previous max value at cell[i-1][j]
    2) the value of the current item + value of the remaining space: cell[i-1][j-weight]

Grid:

| Item/Weight |   1 |      2 |     3 |      4 |      5 |      6 |
|-------------|----:|-------:|------:|-------:|-------:|-------:|
| Water       |   - |      - |  W=10 |   W=10 |   W=10 |   W=10 |
| Book        | B=3 |    B=3 |  W=10 |  BW=13 |  BW=13 |  BW=13 |
| Food        | B=3 |    F=9 | BF=12 |  BW=13 |  FW=19 | BFW=22 |
| Jacket      | B=3 |    F=9 | BF=12 |  FJ=14 |  FW=19 | BFW=22 |
| Camera      | C=6 | BC/F=9 | CF=15 | BCF=17 | CFJ=20 | CFW=25 |

Take the Camera, the Food, and the Water for a total value of 25.

## 11.3

|   | B | L | U | E |
| C | 0 | 0 | 0 | 0 |
| L | 0 | 1 | 0 | 0 |
| U | 0 | 0 | 2 | 0 |
| E | 0 | 0 | 0 | 3 |
| S | 0 | 0 | 0 | 0 |
