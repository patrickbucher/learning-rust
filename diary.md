# Day 63 (2024-05-26)

I first wanted to work with the RSSA book, but then decided to look into
async/await. Preparing an example for that, I re-wrote some regex code for my
soccer-table example, which I then partially incorporated into that crate. I
noticed that the error handling works with `String` as an error type, which I
probably should refactor to use proper error handling with types implementing
the `Error` trait.

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
