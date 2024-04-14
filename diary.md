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
