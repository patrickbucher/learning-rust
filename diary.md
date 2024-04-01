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
