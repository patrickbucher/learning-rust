# Chapter 1

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

# Chapter 2

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
