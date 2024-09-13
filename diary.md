# Day 173 (2024-05-13)

I started implementing a deque as a doubly linked list. The implementation
relies on a lot of clones, but I just want to make it work somehow.

# Day 172 (2024-05-12)

I implemented the remaining operations for the linked list (access at a given
index, search index by a value, insert an element after a given index, delete an
element at a given index).

Next up: doubly linked list, which probably will require the interior mutability
pattern.

# Day 171 (2024-05-11)

Having read the first part of chapter 14 about linked lists, I started to
implement a linked list. I only managed to write a proper `append` method in the
evening, also thanks to the [Rust
Forum](https://users.rust-lang.org/t/append-to-linked-list/117419/3).

# Day 170 (2024-05-10)

I re-read the next part of chapter 15 about `Rc` and `RefCell` and made some
examples.

In the evening, I worked through the rest of the chapter.

Today, I decided to finish CSGDSA in September. Then I'll stop my daily Rust
routine to focus on web frontend technologies for the rest of the year. Maybe
I'll pick up Rust as a daily routine again in January, or maybe something else.
But I'd like to use Rust for my own programs from now on where it comes in
handy.

# Day 169 (2024-05-09)

I started re-reading chapter 15 of the Rust Book on smart pointers. I just read
the easy first part about `Box`, the `Deref`, and the `Drop` trait.

# Day 168 (2024-05-08)

I had a few more things to tweak an `rowcalc`, but I also discovered a few
flaws. The next chapter in CSGDSA will be about node-based data structures, e.g.
linked lists. I should re-examine smart pointers in the Rust book before
tackling this.

I fixed the one big remaining bug in `rowcalc` later in the afternoon and was
able to simplify the code. For my purposes, `rowcalc` is now feature complete.

# Day 167 (2024-05-07)

I refactored `rowcalc`, so that most code is now in `lib.rs` instead of
`main.rs`. I managed to release a new version in the evening. Now back to
CSGDSA.

# Day 166 (2024-05-06)

I once again programmed in the morning and managed to get the phase
computations right for the `rowcalc` program. It needs refactoring, and hasn't
been tested for distances that don't align nicely up to 500m splits, but the
logic works.

# Day 165 (2024-09-05)

Recovering today; I had a little time to work on `rowcalc` in the train. The
data types are getting quite messy; a refactoring will be needed after getting
the functionality together.

# Day 164 (2024-09-04)

I was still sick today, probably worse than yesterday, but managed to compute
the rowing/resting phases for `rowcalc`. Now I have to make the output more
useful by grouping the rowing phases into slices of, say, 500 meters, then I can
work on the output and refactor the code.

# Day 163 (2024-09-03)

I'm still sick, but at least I went through the motions and calculated the 500m
split time for `rowcalc`. I'd like to really work at it tomorrow.

# Day 162 (2024-09-02)

I caught a cold and slept through the day, but I resurrected `rowcalc` in the
evening. I'm now using `duration_str` for the command line arguments (total
time, pause time). However, I'm not sure yet what kind of output I should
produce. But that can wait for tomorrow.

My cold also breaks up my 21 days streak of rowing every morning; so I need to
get the program ready to row again towards the end of the week.

# Day 161 (2024-09-01)

I worked through the exercises of chapter 13, which were rather easy. I learned
about the `to_vec()` function of slices, which is quite handy.

# Day 160 (2024-08-31)

I wrote an optimized version of a function to find duplicates, which uses Quick
Sort. In the process, I found a nasty bug in the partition function, which
caused a stack overflow. I managed to fixed this bug.

# Day 159 (2024-08-30)

I managed to implement the Quick Select algorithm using the `partition` function
I wrote yesterday.

# Day 158 (2024-08-29)

I refactored the Quick Sort algorithm in terms of the `partition` function,
which first remained some fixing.

# Day 157 (2024-08-28)

I continued reading chapter 13 in CSGDSA and started implementing the Quick
Select algorithm; however I got it slightly conceputally wrong.

# Day 156 (2024-08-27)

I started reading chapter 13 in CSGDSA and implemented Quick Sort in a way that
was quite new to me.

# Day 155 (2024-08-26)

I read chapter 12 in CSGDSA, which mostly is about optimizing recursive
functions using memoization. I first was afraid that memoization could be
troublesome due to Rust's borrowing rules, but it works perfectly.

I worked through the exercises during my stay at the hospital; programming can
be a bit tedious when connected to an infusion and when the blood pressure is
measured every 5 minutes automatically. I also had my left thumb in a
contraption to measure my heart rate, so the right thumb had to press all the
spaces for once.

# Day 154 (2024-08-25)

I continued reading chapter 11 in CSGDSA and realized that the Rust type system
was very helpful when implementing the _Staircase Problem_: Since I'm using
`usize`, I don't have to deal with negative numbers; and even the expression `n
- 3` is safe, because it's used in a pattern matching expression where `n` can
  only be at least 4.

I finished the exercises for chapter 11 during the day; the last one was quite
challenging. When my test failed, I figured out that my implementation was
correct, and I just failed to see all the possible solutions to the problem.

# Day 153 (2024-08-24)

I started reading chapter 11 in CSGDSA and worked through the _Staircase
Problem_.

# Day 152 (2024-08-23)

I worked through chapter 10 in CSGDSA, but didn't write any Rust code for it.
(The code for completing the exercises was a better fit for a weakly-typed
programming language such as Python.)

I also asked for help in the Rust forum for the `sumtime` regex matching code,
which can be written much easier using the `?` operator, which I have to use
more often together with `Option<T>`.

Later on, I extended `sumtime` so that I can now also enter decimal time
indications (e.g. 8.5 for 8:30).

# Day 151 (2024-08-22)

I worked through chapter 9 in CSGDSA, for which I both implemented a stack and a
queue based on a vector.

# Day 150 (2024-08-21)

I finished reading chapter 8 in CSGDSA and worked through all the exercises,
mostly dealing with maps and sets.

# Day 149 (2024-08-20)

I refactored and tested yesterday's code of my `sumtime` program.

# Day 148 (2024-08-19)

I wrote a small utility program to sum up time indications of the form hh:mm,
which I use a lot at work. (The AWK version I wrote yesterday is considerably
shorter.)

# Day 147 (2024-08-18)

I worked through chapter 7 in CSGDSA, but only implemented the palindrome
checker. The other "algorithms" were quite arbitrary and not really reusable.
The book is soon going to be a bit more interesting and more challenging.

# Day 146 (2024-08-17)

I worked through chapter 6 in CSGDSA and implemented Insertion Sort (and a
function to find the intersection of two vectors).

# Day 145 (2024-08-16)

I worked through chapter 5 in CSGDSA and implemented Selection Sort.

# Day 144 (2024-08-15)

I finished working through chapter 4 and wrote a function to find the maximum
value in an array (or vector), which was the final exercise. Now I hope for more
interesting work to do.

# Day 143 (2024-08-14)

I wrote a function that draws a random value of a collection based on its item
count, i.e. the higher the item count of a value is, the likelier it will be
drawn.

# Day 142 (2024-08-13)

I read the first part of chapter 4 in CSGDSA and implemented an optimized
version of Bubble Sort.

# Day 141 (2024-08-12)

I read chapter 3 of CSGDSA and worked through the exercises, which didn't
require writing any code. I also implemented a set based on a vector, which was
a residue task from the first chapter.

# Day 140 (2024-08-11)

I finished reading chapter 2 of CSGDSA, for which I implemented both linear and
binary search in an ordered array.

# Day 139 (2024-08-10)

I started reading chapter 2 of CSGDSA and implemented an ordered array.

# Day 138 (2024-08-09)

I started reading _A Common-Sense Guide to Data Structures and Algorithms_,
which I'll refer to as _CSGDSA_ from now on. I read chapter 1 and worked through
the exercises.

In the afternoon, I worked on beerxio by introducing the reqwest library and
querying the contacts from Bexio.

# Day 137 (2024-08-08)

I took up error handling again and documented the `Option.ok_or()` API.

# Day 136 (2024-08-07)

I wrote two integration tests in beerxio to GET and POST the order form.

# Day 135 (2024-08-06)

I implemented proper error handling for beerxio, which allows me to use the `?`
operator for Tera template rendering when handling errors. I also implemented an
order confirmation page using Tera. Next up: accessing the upstream API using
reqwest.

# Day 134 (2024-08-05)

Today, I worked on the beerxio application for my company. I managed to both
integrate dotenv and tera, so that the order form is now rendered dynamically.
Next up: proper error handling.

# Day 133 (2024-08-04)

Having finished reading _Grokking Algorithms_, the planned next step was to work
through _A Common Sense-Guide to Data Structures and Algorithms_ by Jay Wengrow.
However, since I'm rather busy with learning Ansible and preparing for the
second LPIC-1 exam, I should rather try to apply existing knowledge in Rust
rather than learning new things. So I'd like to start with said book only in
roughly two weeks. In the mean time, I'd like to work on a web application
prototype for my company using actix, tera, and reqwest.

I never completely worked through _The Practice of Programming_ by Brian W.
Kernighan and Rob Pike, which is a shame. (I worked through the first two
chapters and used the chapter on Debugging in a lesson I gave last fall.)
Chapter 3 _Design and Implementation_ is about the Markov Chain algorithm, which
is implemented in different programming languages (e.g. C, C++, Java). So I
picked it as my Sunday exercise to implement this algorithm in Rust.

I managed to implement the algorithm and applied it to various texts. _Lorem
Ipsum_ works great, of course, but the output isn't too impressive. The rather
formulaic prose of the book _Genesis_ from the King James Bible was more
convincing.

Later on, I realized that the prefix size for the Markov Chain wasn't properly
used when generating the text. I fixed this using a `VecDeque` in place of the
two temporary word variables.

# Day 132 (2024-08-03)

I wrote some utility function to create all possible subsets of a set, which
allows for an exhaustive solution to the Knapsack Problem. Implementing this
solution was rather easy.

I also thought about what doing next. When I'm done with _Grokking Algorithms_,
I initially wanted to pick up the next book on algorithms and data structures.
However, since I'm rather busy the next two weeks, I'd rather spend my daily
Rust time with implementing a web application for my company. This allows me to
practice actix in a real-world scenario. Then I'll step back into algorithms and
data structures afterwards. This interleaving makes sense from a learning
perspective.

Later, I read through chapter 12 in _Grokking Algorithms_ and implemented the
_K-Nearest Neighbors_ algorithm to classify athletes into sports by their height
and weight. (I'm a cyclist, apparently…)

I finished reading the book in the evening; chapter 13 is only an outlook on
additional algorithms with nothing really to do. The book was a good primer into
some new territories (e.g. dynamic programming), but wasn't really challenging.

# Day 131 (2024-08-02)

I worked through chapter 11 in _Grokking Algorithms_. Then I implemented the
algorithm for solving the _Knapsack Problem_. I only got it to work properly in
the evening.

I'd like to implement an alternative solution working through all possible
sub-sets.

# Day 130 (2024-08-01)

I got the set coverage algorithm to work and was able to slightly simplify the
code. The solution is very intuitive. I also made the code somewhat more concise
to be used by accepting data structures that make use of string references
rather than owned strings, which requires some conversion on the validation of
the result. But the trade-off looks acceptable to me.

# Day 129 (2024-07-31)

I continued implementing the set coverage problem, but the code doesn't find a
solution yet.

# Day 128 (2024-07-30)

I continued working on chapter 10.

# Day 127 (2024-07-29)

I started reading chapter 10 in _Grokking Algorithms_ and started scaffolding
the covering set problem.

# Day 126 (2024-07-28)

I worked through the exercises in chapter 9, after which I finally understood
Dijktra's shortest path algorithm. This allowed me to implement the algorithm.
There's a couple of things I'd like to improve in the code, but I can move on
for now.

# Day 125 (2024-07-27)

I continued implementing the shortest path algorithm.

# Day 124 (2024-07-26)

I finished the refactoring of the graph data structure, which now allows me to
continue implementing the shortest path algorithm.

# Day 123 (2024-07-25)

I started re-reading chapter 9 in _Grokking Algorithm_ and realized that I have
to refactor my data structures in order to implement the shortest path
algorithm.

# Day 122 (2024-07-24)

I read once through chapter 9 in _Grokking Algorithms_, but not focused enough
in order to understand it. I have to re-read it tomorrow early. Graphs can be
modelled using maps and therefore don't require any smart pointers such as `Rc`,
which will make the implementation of Dijkstra's shortest path algorithm much
easier.

I started implementing some boiler-plate code for graphs.

# Day 121 (2024-07-23)

I limited tree rotation to cases where the inbalanced node has a free child
position (left or right), which now seems to work. I leave it there for the
moment and consider tree balancing again with a proper book on the subject. For
now, I'll continue with the other topics in _Grokking Algorithms_.

In the afternoon, I started writing a prototype for a web application that
processes a form. The application should later use the Bexio API, which we plan
to use in our company for a project.

# Day 120 (2024-07-22)

I implemented tree balancing, but there is still a mistake that I was able to
demonstrate using a test case. I'm probably pruning the tree, i.e. omitting
nodes upon balancing.

# Day 119 (2024-07-21)

My `rowcalc` refactoring yesterday screwed up rounding, which I fixed today.

I implemented a method `insert_inplace` for my tree implementation, which should
make it easier to re-balance the tree upon insertion.

# Day 118 (2024-07-20)

Today was a travelling day again, but I managed to unify the computations in
`rowcalc` a bit. I should finally tackle the breaks feature, so that the program
becomes useful for me (again). On the other hand, I have access to _Grokking
Algorithms_ again, so I probably should continue with that.

# Day 117 (2024-07-19)

I refactored the `rowcalc` program. The code is now ready for additional
features (break computation, formatted duration output).

# Day 116 (2024-07-18)

I started writing a _consuming_ `insert` operation, but I'm not sure how to pull
it off.

For a change, I worked again on `warmup` and `rowcalc`, which I use to plan my
exercises. For those tools, I should be able to muster enough motivation.
Especially `rowcalc` I didn't ever use, because it's just unfinished for my
purposes.

# Day 115 (2024-07-17)

I implemented in-order traversal for the tree, which should be helpful to test
tree rotation (balancing). However, I figured out that my initial approach won't
work. I probably should step away from this tree example and re-visit it
together with a more detailed resource for such algorithms, _Grokking
Algorithms_ only gives a very rough idea on the mechanism.

I'll pick up error handling again until I'm back at home, where the physical
book is located.

# Day 114 (2024-07-16)

I figured out that the interior mutability pattern probably is the wrong
approach for the problem at hand. Nonetheless, I now understand it better,
especially when to _not_ apply it. I'll continue with a `Box`.

Later in the evening, I got to know the `take()` method on `Option`, which
probably exactly solves my problem. Ownership goes through all of the APIs in
Rust.

# Day 113 (2024-07-15)

I managed to re-write the existing unit test for the code using the interior
mutability pattern. Since `Rc` implements the `Deref` trait, its use is quite
transparent, which puzzled me at first. However, the compiler warnings guided me
quite well. When in doubt, using pattern matching provided the most useful
compiler messages. I also used the `unreachable!` macro to let a unit test fail.

# Day 112 (2024-07-14)

I finally made some progress with the Interior Mutability Pattern; at least the
original tree implementation works now. However, I had to comment out one part
of the existing unit test, because I still don't know how to access the nested
fields in a tree correctly.

# Day 111 (2024-07-13)

I tried to implement the rotation of an unbalanced tree node, but got entangled
in ownership issues. I first need to study smart pointers again before I can
make any progress here.

In the evening, I read on the interior mutability pattern, but didn't figure out
a solution for my tree problem. But I found a good hint: maybe I schould try
with `RefCell<Option<T>>` instead of `Option<RefCell<T>>`, which I'd like to try
out tomorrow.

# Day 110 (2024-07-12)

I implemented the balance mechanism for binary trees, which is a prerequisite
for rotating unbalanced tree nodes.

# Day 109 (2024-07-11)

I read chapter 8 in _Grokking Algorithms_. Again: no exercises, but also no
code, which is a bit of a letdown. However, I implemented a binary tree with the
operations `insert` and `contains`. The next step will be to transform this tree
into an AVL tree, which balances itself, which might be a challenge in Rust.

In the evening, I did another _Drink First Programming_ session, in which I
programmed the game _Reversi_ in Rust. It was quite a struggle, but everything
worked as intended at the end, even though I didn't implement everything needed
for a good user experience.

# Day 108 (2024-07-10)

I refactored the test cases for breadth-/depth-first search, but since the order
in which directory elements are listed is non-deterministic, the test cases
still rely on a sorted result. However, both work as expected, which is why I'll
continue with the next chapter.

# Day 107 (2024-07-09)

I read chapter 7 in _Grokking Algorithms_ on depth-first search and Huffman
coding. The chapter contains no exercises, but I'd like to implement a directory
walk both as breadth- and depth-first search.

I implemented both breadth- and depth-first search in a struggle session in the
evening. However, the example I built up doesn't really point out the difference
between the two. I'd like to sit down for a rework session of this examples.

# Day 106 (2024-07-08)

I read chapter 6 in _Grokking Algorithms_ on breadth-first search (and graphs).
The examples can be done with a hash table, a queue, and a set; so I don't need
to recap smart pointers for now.

In the evening, I implemented a proper example for a hash table and the
breadth-first search from chapter 6.

# Day 105 (2024-07-07)

I used Clippy to improve the code for my _Grokkig Algorithms_ examples. I should
use more slices instead of vectors. Clippy is a good teacher.

I worked through chapter 5 in _Grokking Algorithms_, which left little to
program, except some hash table prototype.

For chapter 6, I should consider re-reading the chapters on smart pointers in
the Rust book. Otherwise, it won't be possible to work with graphs in a
reasonable way.

# Day 104 (2024-07-06)

I worked through chapter 4 in _Grokking Algorithms_, for which I implemented
several recursice functions and the Quicksort algorithm with test cases.

# Day 103 (2024-07-05)

I worked through chapter 3 in _Grokking Algorithms_, which was about recursion
and had to real programming exercises to do. I implemented the factorial
function with some doc tests, but also struggled through implementing a linked
list, which I finally got to run!

# Day 102 (2024-07-04)

I worked through chapter 2 in _Grokking Algorithms_ and implemented Selection
Sort. Linked lists were also a subject of the chapter, but no implementation was
discussed. However, this would be an interesting exercise for later.

# Day 101 (2024-07-03)

I started reading _Grokking Algorithms_ and implemented Binary Search.

# Day 100 (2024-07-02)

I worked through most of chapter 12, but then finished readig the book without
doing the exercises. I also archived the repository. Conclusion from _Rust
Servers, Services, and Apps_ (RSSA): The books shows how to build a basic web
application using the actix framework with sqlx, but the same would have been
possible on 100 pages instead of 300.

On towards more challenging topics: Algorithms.

# Day 99 (2024-07-01)

I worked through chapter 11 of the RSSA book, which just opens a new loose end:
peer-to-peer communication. This chapter could be a complete stand-alone
tutorial, I have no idea how it made it into this book.

# Day 98 (2024-06-30)

I worked through chapter 10 of the RSSA book, which is about async Rust. The
introduction was decent, however, a practical example would have been more
illustrative.

# Day 97 (2024-06-29)

I worked through chapter 9 of the RSSA book, which hardly introduced anything
new besides checking a hashed password. I modified my review on Amazon and
reduced the rating from 3 to 2 stars. Now I hope that at least the rest of the
book is a bit more englightening.

# Day 96 (2024-06-28)

I continued working on chapter 8 in the RSSA book. I also wrote a short review
about the book on Amazon to warn other readers. It was really executed poorly.
Afterwards, I finished chapter 8 nonetheless.

# Day 95 (2024-06-27)

I started working through chapter 8 in the RSSA book, which again looks rather
tedious.

# Day 94 (2024-06-26)

I managed to finish chapter 7 in the RSSA book, which required some adjustments
(the actix client is now part of the awc crate). I'd like to go as quickly as
possible through chapters 8 and 9 so that I can get started with Grokking
Algorithms next week.

In the evening, I worked through my first session of [Drink First
Programming](https://www.youtube.com/playlist?list=PLux6j39XOCC7T_3h1MI86CkOc1qqrzxGR),
for which I implemented _Connect Four_ in Rust.

# Day 93 (2024-06-25)

I started working through chapter 7, which introduces Tera templates that are
similar to Jinja templates.

# Day 92 (2024-06-24)

I almost finished chapter 6 in the RSSA book, but unfortunately, the code
provided is incomplete. Fortunately, the upcoming chapters look promising
(templates, forms, async), so I'll struggle through it.

In the evening, I finally finished chapter 6.

# Day 91 (2024-06-23)

I managed to get the first half of the modifications in chapter 6 (RSSA book)
running. There was quite some drudgery involved, and the book's quality is below
the usual Manning standards. However, there's useful content in the rest of the
book, so I'll continue to work it through.

# Day 90 (2024-06-22)

Not much progress on the RSSA front; the code doesn't build after all the
modifications. I have to step through the recent ~12 pages again.

# Day 89 (2024-06-21)

Little time today again, but I continued writing the unit tests for chapter 6 in
the RSSA book.

# Day 88 (2024-06-20)

I continued working through chapter 6 of the RSSA book. I spotted more
repetitions and incoherences. The book could be considerably shorter—and better.
But I'll finish it anyway.

# Day 87 (2024-06-19)

I finally picked up the RSSA book again after 2-3 weeks of other activities. I
enhanced the course data model as described.

# Day 86 (2024-06-18)

I worked through exercise 57 and, thus, finished the book.

I also wrote a little utility called [warmup](https://crates.io/crates/warmup)
for my own use.

# Day 85 (2024-06-17)

I did exercise 56 in the evening. One more to go.

# Day 84 (2024-06-16)

I worked through exercise 55, which is a text sharing/editing tool. The
requirements were technically quite similar to exercise 54 from yesterday, so I
could make the code slightly nicer and was able to do everything roughly 80
minutes. I also used a poor man's templating engine for the task, because
learning a proper one would be overkill. (But I should look into those, too.)

I deleted the `grokking-algorithms` folder. I'd like to start from scratch, both
structurally and with the actual book.

# Day 83 (2024-06-15)

I worked on exercise 54, which is an URL shortener. I got it to work using actix
with Redis as a storage backend, however, I still need to implement the
statistics function.

I finished exercise 54 in the evening, which was quite straightforward.

# Day 82 (2024-06-14)

I glanced at exercises 53-57 and decided to use Redis for _all_ of them, besides
contradicting constraints. Getting acquainted with a Redis library for Rust is
the first step for my little monitoring tool called peasant, which I'd like to
develop this year.

I only managed to work through exercise 53 late in the evening.

# Day 81 (2024-06-13)

I worked through exercise 51, for which I used quite a couple of libraries:
dotenv, serde, reqwest. For exercise 52, I used the actix framework. This
concludes the penultimate section of _Exercises for Programmers_. There are five
more exercises, which are supposedly quite big. I'd like to go through them this
week, so that I can finish the RSSA book in the second half of June.

# Day 80 (2024-06-12)

I worked through exercise 50, for which I figured out how to map fields with an
arbitrary names with serde.

Later, it took me some time to figure out how to deal with Firebase for exercise
51, for which I am supposed to use the REST API instead of the Firebase API.

# Day 79 (2024-06-11)

I worked through exercise 48, which took a while to test, because the API key
was only accepted roughly 15 minutes after registration.

Exercise 49 I only managed to finished in the evening. I didn't care about
proper error handling, but it's more important to deal with HTTP requests and
JSON deserialization in those exercises.

For exercise 50, I already found an alternative API to be used, because the
proposed API from Rotten Tomatoes is no longer freely available for educational
purposes.

# Day 78 (2024-06-10)

I worked through exercises 41, 42, 43, 44, 45, and 46 on file handling; and on
exercise 47 on API access. I think it's the best to finish one book at a time;
first _Exercises for Programmers_ and then the RSSA book. Afterwards, I'd like
to get into algorithms again using Rust, probably starting with _Grokking
Algorithms_ or with _A Common-Sense Guide to Data Structures and Algorithms_.

# Day 77 (2024-06-09)

I only managed to do some programming in the evening: exercise 40.

# Day 76 (2024-06-08)

I worked through exercise 39.

I also managed to refactor the `sum-duration` command of `csvtool`, which now u
ses two closures to 1) extract a particular column value of each row, and 2) sum
up the durations for the additional total row. I still don't understand
everything, and the code lacks proper error handling, but it's a progress.

Later, I wrote a little program to compare Formula 1 tyre strategies. I had the
idea for school, but first wanted to implement it in Rust as a proof of concept.

# Day 75 (2024-06-07)

I worked through exercises 37 and 38.

# Day 74 (2024-06-06)

I worked through exercise 36. Unfortunately, the example standard deviation in
the text is wrong.

# Day 73 (2024-06-05)

I worked through exercise 33, 34, and 35.

# Day 72 (2024-06-04)

I worked through exercise 32.

# Day 71 (2024-06-03)

I worked through exercise 31 from _Exercises for Programmers_.

In the evening, I started to refactor `csvtool` in order to provide a common
interface to `rewrite` and `sum-duration` using some kind of hooks, but I first
need to figure out how to work with closures vs. function pointers.

# Day 70 (2024-06-02)

I started reading in _Effective Rust_ again; chapter 3 on `Result`/`Option`
transforms, that is. The chapter is relatively short, but motivated me to do
create my own [Q&A collection](error-handling.md) with code examples and a
[diagram](error-handling.png). I'll extend this collection to my own use.
Publishing a version of it on my personal website might be useful for myself and
others, too.

Concerning the `csvtool`: I finished the `sum-duration` command for now, but
there is a lot to do in terms of testing and documentation. But at least I can
use the tool in production now.

On the planning front, I created a overview of my [ongoing Rust
activities](activities.md). This is a good go-to document when I don't know what
to do next. It also helps me to plan the next steps. Working on projects is
something I can always do. The same is true for my notes, e.g. on [error
handling](error-handling.md).

But for deliberate practice into new territories, I have to plan which books to
work through. _Effective Rust_ is perfectly to read along my other activities.
_Exercises for Programmers_ is something to finish off, and so is the RSSA book.
My current hunch is to finish both the RSSA book and _Exercises for Programmers_
in June, the latter of which requires me to do one exercise a day. Finishing
these two books would also mark the end of my initially planned three month
period with Rust.

The question is: Shall I continue with Rust, or pick up Haskell again or some
other topic? Usually, picking up a fresh topic after a three month period seems
very attractive to me. Not this time: Having worked with Rust every day for 70
days now, Rust just feels _right_. I write slowly, and I write a lot of code.
Even though this contradicts my ideal of easy to write concise code, I'm feeling
very good about working with Rust. It never will compete against Ruby on Rails
or Laravel when it comes to writing web applications very quickly. But another
of my ideals is _going right_ instaed of _going fast_. And unlike Haskell,
nobody can claim that Rust is an ivory tower language, for it has been both
adopted by the Windows and the Linux kernel, which makes it running most of the
world.

I'd like to revisit my experiment of the last summer to learn Algorithms. But
this time, I pick only Rust as my language, and I consider learning some
algorithms in a special field, such as the ones presented in _Algorithms for
Decision Making_. But the CLRS book might also be good to revisit with my
upgraded Rust skills. I need to look at both, and maybe even into _Artificial
Intelligence_, which sits right next to the CLRS book in my shelf. There are
also alternatives such as _Grokking Algorithms_ and _A Common-Sense Guide to
Data Structures and Algorithms_, which would both be a good stepping stone for a
more serious project.

# Day 69 (2024-06-01)

As announced yesterday, I started the major refactoring on my `csvtool`. I also
included a command to `rewrite` a CSV file, which serves as a kind of template
for the re-implemented `sum-duration` functionality. I wonder if there will be a
possibility to merge the two functionalities by passing some additional closures
to it, but first I have to finish the refactoring, until I have a working
version for my own productive company use.

I also tried out some Vim integration for Rust, but at this point it doesn't
seem to be worth the hassle. I'd rather deeply get to know the APIs on my own
the hard way instead of messing around with half a dozen tools that just require
setup, documentation, and maintenance.

# Day 68 (2024-05-31)

After a bad night, I only got to Rust programming in the evening after having
caught up on sleep. I worked on my `csvtool` and added a sum row to the parsed
data structure. The tricky part is now to mangle this vector of hash maps back
into a CSV data structure. I need to re-consider the entire idea of parsing,
maybe I should process the CSV file on-the-fly using a reader/writer pair
instead.

# Day 67 (2024-05-30)

I refactored the parsing code for `csvtool`, which provides a struct containing
the regex pattern, which then can be re-used for multiple parsing operations. (I
recently profiled the code for a similar issue in the soccer-table example, and
compiling regexes is really something you wouldn't want to do within an
iteration.)

# Day 66 (2024-05-29)

I started building the boilerplate for `csvtool`. I took over the parsing code
from this repository and deviced some dispatch mechanism to deal with different
commands/tasks. I also implemented the function to sum up a duration column of
the format `hh:mm`, but wasn't able yet to extend the CSV output by that column.

# Day 65 (2024-05-28)

I planned to work through chapter 6.2 in the RSSA book, but after a bad night
and an early appointment, I wasn't  able to even get started. Instead, I
investigated an [issue](https://stackoverflow.com/q/78542978/6763074) I ran into
the other day, which helped me gain some clarity.

In the evening, I registered two crates, just to make sure the names won't be
taken. The first is `csvtool`, which will be a tool to deal with time sheets in
CSV format. The second is `peasant`, which will consist of two binaries:
`harvest` to collect JSON logs from nginx and system metricts such as CPU and
memory usage to be stored in a Redis time series; and `granary` to archive the
logs of multiple servers centrally. The goal is to develop a minimalistic
logging/monitoring stack for my own use.

# Day 64 (2024-05-27)

I started reading chapter 6 in the RSSA book and did the refactoring steps in
preparation for the upcoming improvements of said chapter.

# Day 63 (2024-05-26)

I first wanted to work with the RSSA book, but then decided to look into
async/await. Preparing an example for that, I re-wrote some regex code for my
soccer-table example, which I then partially incorporated into that crate. I
noticed that the error handling works with `String` as an error type, which I
probably should refactor to use proper error handling with types implementing
the `Error` trait.

I refactored the soccer-table example and made some good progress with both
error handling and iterator processing (e.g. with using `filter_map` and
`flat_map` in appropriate cases).

# Day 62 (2024-05-25)

I worked through chapter 5 in the RSSA book. I learned a lot about error
handling, but I'm getting mixed feelings about the book. It uses different
versions of the Actix in every chapter, and also different conventions for
dealing with path variables. The code isn't formatted consistently and looks a
bit sloppy at times. I'll finish the book and then look forward to _Rust Web
Development_.

# Day 61 (2024-05-24)

I finished the parsing code for `timetable`; now begins the real work.

# Day 60 (2024-05-23)

I started a small utility called `timetable`, which is supposed to convert CSV
files to markdown timetables. I plan to use this tool in production to report my
hours conveniently for customer projects.

# Day 59 (2024-05-22)

I finished working through chapter 4 in the RSSA book.

Today, my copy of _Effective Rust_ arrived. I read the first two items, which
are a good summary on the type system.

# Day 58 (2024-05-21)

I continued reading chapter 4 in the RSSA book and am working on the second
iteration of the data base example there.

# Day 57 (2024-05-20)

I started reading chapter 4 in the RSSA book. I managed to query the PostgreSQL
database, but needed to resolve a privilege issue. (PostgreSQL >=15 is more
restrictive, it turned out.)

# Day 56 (2024-05-19)

I worked through three more _Exercises for Programmers_.

# Day 55 (2024-05-18)

I finished the soccer table implementation today. I struggled with `Ord` and
`PartialOrd`, and therefore needed to resort to `sort_by` with an explicit call
to my custom `cmp` implementation. I need to do some research on this topic. (I
figured out how to get it to work in the mean time, but I still don't unterstand
the two concepts of partial and full order.)

Later, I tried out `flamegraph` to profile the `soccer-table` binary. As I
suspected, re-compiling a regular expression for every file name and contained
line was the bottle neck. I got rid of the `TryFrom` implementation from
`String` to `MatchResult` in order to write a function, for which I can provide
the `Regex` as an argument. This improved performance massively; from ~50ms to
~8ms. Profiling with `flamegraph` is so easy that it's practical to do on a
regular basis. The whole Rust ecosystem just feels _right_ to me. I also have
the impression that I learn something new every day with Rust.

# Day 54 (2024-05-17)

I managed to get the parsing code for the soccer table working, but the code
still needs improvement.

# Day 53 (2024-05-16)

I started writing the parsing code for the soccer table example. I used the
`TryFrom` trait with its `try_into` method, which returns a `Result`, for the
first time.

# Day 52 (2024-05-15)

I managed to finish the file name filtering and refactored the code using
clippy, which showed me a lot of small improvements, which combined helped
reducing a lot of code.

# Day 51 (2024-05-14)

I finished the extraction of days from the file names, als also refactored the
code. However, further refactoring is due.

# Day 50 (2024-05-13)

I started the day with exercise 27, which required digging into the `regex`
crate. This comes in very handy, because I'd like to re-implement the soccer
table example as an exercise.

I struggled a bit with the command-line argument parsing for the soccer-table
example. The reason for this was another optional argument that is parsed from
an optional string to a number. The process involves transitions from `Option`
to `Result`, which takes a while to be done nicely in code. It reminded me about
the importance of the _Separation of Concerns_, which is extremely important
when learning Rust: first get the code to compile, then make it elegant.

I have to take some more time to study all the methods of `Option` and `Result`,
and how they interact to create more succinct code with less `match` and `if
let` statements.

# Day 49 (2024-05-12)

I finished working through chapter 3 in the RSSA book. My first impression on
Actix: it looks quite powerful and reasonable.

# Day 48 (2024-05-11)

I worked through the first half of chapter 3 in the RSSA book. I learned about
the `path` attribute, which allows to customize the paths to module files.

# Day 47 (2024-05-10)

I worked through exercise 26 from _Exercises for Programmers_, for which I used
the logarithm methods of `f64` (`log10`).

# Day 46 (2024-05-09)

I read the second half of chapter 3 in the RSSA book. I also worked through
exercise 25, for which I used an enum and wrote tests; the first I should do
more often, the second always.

# Day 45 (2024-05-08)

I read the first half of chapter 3 in the RSSA book and worked through exercise
24 in _Exercises for Programmers_, for which I learned two new APIs:
`Vec::remove` and `Iterator::position`.

# Day 44 (2024-05-07)

I worked through exercises 22 and 23 in _Exercises for Programmers_. The latter
was quite interesting to solve, because I good make good use of enums. The
implementation was straightforward once I got the data structure in place.

# Day 43 (2024-05-06)

I extended `akshually` to a function that formats a `Duration` "Go-style", e.g.
`1h13m57s123ms`. Afterwards, I extended `ghbu` to make use of this function. I
also improved the output slightly.

# Day 42 (2024-05-05)

I extended `ghbu` to also display timings. Unfortunately, there's no formatting
routines provided by the standard library to output the timings such as
`3m12s654ms`, like the Go standard library does. But `seconds.milliseconds` is a
good compromise in my case.

# Day 41 (2024-05-04)

I worked through two _Exercises for Programmers_. Later, I did a third one. The
exercises are kind of repetetive, but as I glanced at some further exercises,
there seems to be more interesting one coming up.

# Day 40 (2024-05-03)

I started exploring the GitHub API for backing up an organization's repos. I
laid out a plan and will introduce a pair of new mutually exclusive command line
arguments `--org ORG` and `--user USER`, which will be backed up to the folder
`TO/ORG` and `TO/USER`, respectively, with `TO` given by the command line
argument `--to TO`.

In the evening, I managed to implement the new feature. So `ghbu` is
feature-complete for now, at least for GitHub backups.

# Day 39 (2024-05-02)

I finished my refactoring of `ghbu` for now. There are no `unwrap()` calls left
in the code, and the structure is acceptable for now. The next step will be to
extend the functionality, so that I can also backup the repositories of the
organizations I belong to. For that, I have to explore the GitHub API. Securing
one's Gitea Repositories could also be interesting; not only for me, but also
for my students.

# Day 38 (2024-05-01)

I refactored the error handling in `ghbu`. It was really helpful to draw the
object graph to work out dependencies—and to flatten the code structure. The `?`
operator is extremely handy. The lack thereof is exactly what makes error
handling in Go so tedious.

# Day 37 (2024-04-30)

I read through the second chapter in the RSSA book once. During the day, I
worked through all the example code. I learned about the `From<T>` trait, and
how it works together with the `into()` method.

This concludes my first full month of everyday Rust programming.

# Day 36 (2024-04-29)

I continued refactoring the `ghbu` code. There's still a lot to do, but the
important thing is to make steady progress.

# Day 35 (2024-04-28)

I added a `--cleanup` flag to `ghbu`, which causes broken repositories to be
deleted (and cloned later during the backup run).

# Day 34 (2024-04-27)

I managed to finish the MVP for `ghbu`. New repositories are cloned, existing
repositories are fetched. Thanks to working with bare repositories, up to 50% of
space is saved. This is exactly what I need for a backup. There are various
improvements to be done to `ghbu`, which I've already written down. However,
after working a month with Rust, I finally managed to write something useful.

I had a closer look at two books in my shelf: _Rust Web Development_ and _Rust
Server, Services, and Apps_. The former is a bit longer and uses the `warp`
library for HTTP. The latter is shorter and uses the `actix` stack, and I
decided to work through that book first, i.e. in May.

In the evening, I finished the rustlings, some of which were rather mediocre and
annoying. But at least I got a good glimpse into various topics. I also started
reading _Rust Servers, Services, and Apps_, which I'll refer to as _the RSSA
book_ from now on.

# Day 33 (2024-04-26)

I decided to work with bare repositories in `ghbu`, which not only saves up to
50% of storage, but also won't require any merges when updating the backup. I
tested the approach with `clone --bare` and `fetch` manually, and it seems to
work beautifully.

# Day 32 (2024-04-25)

I realized that `git clone` is not built into `libgit2` and, therefore, needs to
be implemented as a `fetch` operation, followed by a fast-forward merge. I now
consider just using `git` as an external process. However, I managed to do a
full backup of all my Git repositories on my FreeBSD backup server, which solves
a practical problem. I can continue from there.

# Day 31 (2024-04-24)

I continued working on `ghbu`, for which I made sure the path specified by the
`--to` argument exists and is a directory. Later, I managed to clone a
repository using my SSH key via the `libgit2` implementation of Rust in the form
of the `git2` library.

# Day 30 (2024-04-23)

I started the day by refactoring the code in `ghbu` that fetches the repository
SSH URLs. It now returns a map of SSH URL by repository name. I also put that
code into the library crate.

# Day 29 (2024-04-22)

Well, there was no commit in _this_ repository yesterday for the first time in
four weeks, but I didn't fail; quite the opposite. I worked through chapter 20,
but didn't write any code. (Now I know where to look up in case I'd like to
implement a thread pool.)

I re-started my GitHub Backup project, this time called `ghbu`, and made some
fair progress. The first prototype will be working after a couple of sessions.

# Day 28 (2024-04-21)

Today, I read chapter 19 without doing any examples. The chapter mostly covers
things that are good to know for a beginner like me, and I'll return to them as
the need arises. I also worked through exercise 18, because I felt guilty not
writing a single line of code today.

# Day 27 (2024-04-20)

I read chapter 18 on patterns and completed some additional rustlings.
Unfortunately, there are no more rustlings on pattern matching, which was the
subject of chapter 18.

# Day 26 (2024-04-19)

I added the prime number APIs from my `factor` re-implementation to my
`akshually` library. I added APIs to compute the GCD and to reduce fractions. I
also re-organized the library into a `io` and a `math` module.

# Day 25 (2024-04-18)

I finished reading chapter 17 and worked through the state pattern example
besides. Now I'd like to write an article about the state pattern (with examples
in Python) and a re-implementation of it using a Rust enum.

# Day 24 (2024-04-17)

I started reading chapter 17 on OOP, and now understand the purpose of the `dyn`
keyword. I first wanted to skim the rest of the chapter and do an alternative
example (implementing the state pattern for text files, then refactoring it to
an enum, which makes for a good article). However, I noticed that some
additional mechanisms are introduced in the book's example, so I'll work through
that carefully tomorrow morning.

# Day 23 (2024-04-16)

I read chapter 16 on concurrency and made a couple of (more or less) convincing
examples.

# Day 22 (2024-04-15)

I re-read the section about `Rc<T>` and made some (working) examples for it.
Later, I finished working through the entire chapter and made some examples for
each kind of smart pointer. However, still didn't manage to implement a double
linked list with useful features. I'll try that out at another occasion. I now
have some useful notes for smart pointers.

In the evening, I messed around with a double linked list. I realized that I
could also implement a sorting list with a single linked list, defeating the
point of the exercise. So I'll continue with chapter 16 tomorrow.

# Day 21 (2024-04-14)

I continued reading chapter 15 while taking notes and writing some example code.
I also worked through exercise 17. Later, I worked through the sections on
`Deref` and `Drop`, for which I implemented a couple of examples.

# Day 20 (2024-04-13)

I'll re-read chapter 15 now and take notes. I'll also create some code snippets
with unit tests to check my understanding of the topic.

# Day 19 (2024-04-12)

I finished reading chapter 15, which left open many questions. I'm wondering if
I should re-read it straight away—or if I first should finish the entire book,
because some error messages I encountered when playing with my own examples
hinted to information that will only be introduced in chapter 19. But I
definitively need to re-read chapter 15.

# Day 18 (2024-04-11)

I finished my article on prime factorization in Rust and published it. Then I
started reading chapter 15 on smart pointers.

# Day 17 (2024-04-10)

I read chapter 14 and added some doc comments to my `akshually` library, which I
then published in a new version. I started writing an article about iterators,
and how they can be used to speed up prime factorization.

# Day 16 (2024-04-09)

I read chapter 13, did some examples on my own, and applied the proposed
improvements to the `minigrep` project from yesterday.

Later in the morning, I implemented prime factorization in Rust, which mixes
loops and iterators/closures. The implementation was rather slow, i.e. two
orders of magnitude slower than `factor(1)`, which is implemented in C.

I had the idea to use some kind of stream to interleave factorization with the
prime number search in order to make it run faster. Only in the evening I had
the insight that an Iterator is exactly what I needed. I implemented  my first
iterator, which allowed me to rewrite the factorization function. Now it works
roughly at the same speed as `factor(1)`. This programming example screams for
an article!

# Day 15 (2024-04-08)

I read chapter 12 and worked through the little project. I also implemented the
binary search algorithm from Grokking Algorithm. (I now covered most of the
topics to implement the basic algorithms from that particular book.)

# Day 14 (2024-04-07)

I re-read the entire chapter 11 and made some examples alongside. From now on,
I'll write tests for the exercises where appropriate. I also did a couple of
exercises, which are quite repetitive on one hand, but help me building muscle
memory on the other hand.

# Day 13 (2024-04-06)

Today was a rather unproductive day. I only read half of chapter 11 on automated
tests. I'll re-read the entire chapter tomorrow and will do some examples
alongside.

# Day 12 (2024-04-05)

I read chapter 10 on generics, traits, and lifetimes. I'll skip the Brown
University version for most chapters from now on, except for some very tough
ones. It seems to me that the Brown University book just belabours the point of
ownership. I see the point of exercises where you must assume that the compiler
passed code it'd actually reject, but having to deal with two mental models at
the same time ("Assuming that the compiler would accept _this_ function, which
of the following invocations of it would cause a compiler error") is rather
frustrating then enlightening. I'd rather get to know the ownership rules by
actual programming experience.

Speaking of which, I published a library crate
[akshually](https://crates.io/crates/akshually) to collect some functions I keep
on re-writing. For the artwork, I could put my trolling skills to good use once
again.

# Day 11 (2024-04-04)

I read chapter 9 on error handling, which introduces the `?` operator I stumbled
upon recently. I have to adopt that one into daily use!

# Day 10 (2024-04-03)

I read chapter 8 on collections. In order to complete the exercises, I figured
out how to structure a project with multiple binary crates. The exercises at the
end of the chapter were rather tricky, but (therefore) also useful.

I also did a couple of exercises and continued with the Rustlings.

# Day 9 (2024-04-02)

I read chapter 7 about packages, modules, and crates. I skip on re-reading it in
the Brown University version, because there's not much to do with (nested)
modules at this point. Now I'm looking forward to collections, which allows me
to continue with Rustlings.

# Day 8 (2024-04-01)

I read chapter 6 about enums, the `match` keyword, and the `if let` construct
and made some examples alongside. I also did a few exercises.

# Day 7 (2024-03-31)

I finished reading chapter 5 and did some examples alongside. I found the
[`duration_str`](https://docs.rs/duration-str/latest/duration_str/) crate, which
I'd like to use for my rowcalc program. I also further experimented with the
`fmt!` macro to pad numbers with zeros.

# Day 6 (2024-03-30)

I finished reading chapter 5 on structs (and methods) in the book. I also worked
through some simple exercises. I used the chrono crate for date handling, and
the `format!` macro for formatted output.

I have to re-read chapter 5 tomorrow, because I failed the quizzes today. (I
didn't try too hard.)

# Day 5 (2024-03-29)

I finished reading chapter 4 of the Brown University book, which still was a bit
overwhelming. Instead of just explaining the borrow checker rules, a destinction
between unsafe programs on one side and safe programs rejected by the borrow
checker nonetheless on the other side is made. The reason for this distinction
is also given: If the borrow checker rejects your program, there are two reasons
with different consequences:

1. If the program is rejected due to actual undefined behaviour, you need to
   modify the logic of your program. Maybe the sequence in which variables are
   accessed needs to be changed, or maybe some values need to be copied instead
   of being referred to.
2. If the program that cannot cause undefined behaviour is rejected, you
   probably just need to do slight changes to the data access paths, e.g. adding
   a `mut` to make a variable mutable.

I now got a clearer picture of ownership, but still fail to get all the quizzes
right. So I need to practice.

Later on, I wrote my first productive Rust code: a utility to calculate 500
meter split times for rowing sessions called
[rowcalc](https://github.com/patrickbucher/rowcalc), for which I used the
[clap](https://docs.rs/clap/latest/clap/) library to parse command line
arguments. I'll improve and extend the tool in the future, but it's a good
start.

In the afternoon, I read the first half of chapter 5 on structs.

# Day 4 (2024-03-28)

I started reading chapter 4 on _Ownership_ in the Brown University version of
the book. The first three chapters are almost identical in the original and in
Brown University's adapted version. However, chapter 4 is _totally_ different.

At the beginning, I struggled with the difference between code that fails to
compile and code that is actually unsafe. However, the concepts are explained
way more in depth in the Brown University version of the book.

# Day 3 (2024-03-27)

I re-read chapter 3 and worked through three additional exercises, which all
involve input and output. Since `println!` always creates a newline before the
input, I wanted to use `print!` instead first, but this output didn't flush. As
I looked the issue up, one of the first results was a [question of
mine](https://stackoverflow.com/q/39174864) on StackOverflow, dating back to my
first encounter with rust eight(!) years ago. Those little issues always
derailed me so far, but this time I will pull through. I also finished a lot of
rustlings; up to vectors.

In the afternoon, I re-read chapter 4 on _Ownership_, which leaves me with the
ambiguous feeling that I understood the rules, but still cannot apply them to
programs.

# Day 2 (2024-03-26)

I re-read chapter 2, both in the physical book and in the online version, which
are identical. I worked through the guessing game example. Afterwards, I solved
a couple of rustlings on variables. Then I was able to do the first of the
_Exercises for Programmers_; however without the constraint of separating string
concatenation from output.

# Day 1 (2024-03-25)

I re-read chapter 1 of _The Rust Programming Language_—both in the paperback and
in the Brown University version, which look identical to me. I also started with
rustlings, of which I did the first two. I wanted to start exercise 1 _Saying
Hello_ in _Exercises for Programmers_, but since this requires user input, I
have to postpone it after having read chapter 2 of the book.

As an aside, I followed the suggestion from the _No Boilerplate_ video on
how to learn Rust and started reading _Ultralearning_ as a side quest. I read
the nine principles and am trying to apply them to Rust as follows:

1. **Metalearning**: first draw a map.
    - Focus on the hard and unfamiliar parts of Rust: ownership, lifetimes,
      smart pointers, async/await.
2. **Focus**: sharpen your knife.
    - Build a habit of reading a lot of hard material.
3. **Directness**: go straight ahead.
    - Don't try to write a complete personal summary about everything you learn;
      spend time programming instead.
    - Only write about hard things in order to understand them (Feynman method).
4. **Drill**: attack your weakest point.
    - Don't get overwhelmed by a lot of new syntax and new concepts.
    - Focus on one single concept at the time; proceed step by step.
5. **Retrieval**: test to learn.
    - Write actual, useful programs in Rust.
6. **Feedback**: don't dodge the punches.
    - The Rust compiler is ruthless—but also helpful! The code must pass its
      checks in order to run.
7. **Retention**: don't fill a leaky bucket.
    - Apply new concepts to actual problems in real programs.
    - Write about things you have to look up frequently.
    - Dont mind forgetting things you never use in practice.
8. **Intuition**: dig deep before building up.
    - Try out new concepts and libraries; write about them to deepen your
      understanding.
9. **Experimentation**: explore outside your comfort zone.
    - Solve practical, real-world problems with Rust.

# Day 0 (2024-03-24)

Having neither found the motivation for Angular/TypeScript on one hand and for
Haskell on the other hand, I'd like to dive into Rust—once more, and once and
for all! Having almost wasted the first quarter of this year by dipping into
Erlang, Haskell, TypeScript and Angular using different material, I'd like to
focus the next quarter on Rust entirely. Working with the Rust type system and
reading the Rust compiler's error messages will bring me a step closer to
Haskell, which I'd like to approach once more in the second half of the year.
But for now Rust seems to be the perfect compromise between difficulty,
practicality, and long-time reward.
