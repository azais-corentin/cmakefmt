# Introduction

Proptest is a property testing framework (i.e., the QuickCheck family)
inspired by the [Hypothesis](https://hypothesis.works/) framework for
Python. It allows to test that certain properties of your code hold for
arbitrary inputs, and if a failure is found, automatically finds the
minimal test case to reproduce the problem. Unlike QuickCheck, generation
and shrinking is defined on a per-value basis instead of per-type, which
makes it more flexible and simplifies composition.

## Status of this crate

The crate is fairly close to being feature-complete and has not seen
substantial architectural changes in quite some time. At this point, it mainly
sees passive maintenance.

See the [changelog](https://github.com/proptest-rs/proptest/blob/main/proptest/CHANGELOG.md)
for a full list of substantial historical changes, breaking and otherwise.

## What is property testing?

*Property testing* is a system of testing code by checking that certain
properties of its output or behaviour are fulfilled for all inputs. These
inputs are generated automatically, and, critically, when a failing input
is found, the input is automatically reduced to a *minimal* test case.

Property testing is best used to complement traditional unit testing (i.e.,
using specific inputs chosen by hand). Traditional tests can test specific
known edge cases, simple inputs, and inputs that were known in the past to
reveal bugs, whereas property tests will search for more complicated inputs
that cause problems.

---

# The `proptest` crate

The `proptest` crate provides most of Proptest's functionality, including most
strategies and the testing framework itself.

This part of the book is dedicated to introductory material, such as tutorials,
and general usage suggestions. It does *not* contain reference documentation;
for that, please see the [rustdoc documentation](https://docs.rs/proptest/latest/proptest/).

---

# Getting Started

Let's say we want to make a function that parses dates of the form
`YYYY-MM-DD`. We're not going to worry about *validating* the date, any
triple of integers is fine. So let's bang something out real quick.

```rust
fn parse_date(s: &str) -> Option<(u32, u32, u32)> {
    if 10 != s.len() {
        return None;
    }
    if "-" != &s[4..5] || "-" != &s[7..8] {
        return None;
    }

    let year = &s[0..4];
    let month = &s[6..7];
    let day = &s[8..10];

    year.parse::<u32>().ok().and_then(|y| {
        month
            .parse::<u32>()
            .ok()
            .and_then(|m| day.parse::<u32>().ok().map(|d| (y, m, d)))
    })
}
```

It compiles, that means it works, right? Maybe not, let's add some tests.

```rust,ignore
# fn parse_date(s: &str) -> Option<(u32, u32, u32)> {
#     if 10 != s.len() { return None; }
#     if "-" != &s[4..5] || "-" != &s[7..8] { return None; }
#
#     let year = &s[0..4];
#     let month = &s[6..7];
#     let day = &s[8..10];
#
#     year.parse::<u32>().ok().and_then(
#         |y| month.parse::<u32>().ok().and_then(
#             |m| day.parse::<u32>().ok().map(
#                 |d| (y, m, d))))
# }
#[test]
# fn dummy(0..1) {} // Doctests don't build `#[test]` functions, so we need this
fn test_parse_date() {
    assert_eq!(None, parse_date("2017-06-1"));
    assert_eq!(None, parse_date("2017-06-170"));
    assert_eq!(None, parse_date("2017006-17"));
    assert_eq!(None, parse_date("2017-06017"));
    assert_eq!(Some((2017, 06, 17)), parse_date("2017-06-17"));
}
# fn main() { test_parse_date(); }
```

Tests pass, deploy to production! But now your application starts crashing,
and people are upset that you moved Christmas to February. Maybe we need to
be a bit more thorough.

In `Cargo.toml`, add

```toml
[dev-dependencies]
proptest = "1.10.0"
```

Now we can add some property tests to our date parser. But how do we test
the date parser for arbitrary inputs, without making another date parser in
the test to validate it? We won't need to as long as we choose our inputs
and properties correctly. But before correctness, there's actually an even
simpler property to test: *The function should not crash.* Let's start
there.

```rust,should_panic
# extern crate proptest;
// Bring the macros and other important things into scope.
use proptest::prelude::*;
# fn parse_date(s: &str) -> Option<(u32, u32, u32)> {
#     if 10 != s.len() { return None; }
#     if "-" != &s[4..5] || "-" != &s[7..8] { return None; }
#
#     let year = &s[0..4];
#     let month = &s[6..7];
#     let day = &s[8..10];
#
#     year.parse::<u32>().ok().and_then(
#         |y| month.parse::<u32>().ok().and_then(
#             |m| day.parse::<u32>().ok().map(
#                 |d| (y, m, d))))
# }
proptest! {
    #[test]
    # fn dummy(0..1) {} // Doctests don't build `#[test]` functions, so we need this
    fn doesnt_crash(s in "\\PC*") {
        parse_date(&s);
    }
}
# fn main() { doesnt_crash(); }
```

What this does is take a literally random `&String` (ignore `\\PC*` for the
moment, we'll get back to that — if you've already figured it out, contain
your excitement for a bit) and give it to `parse_date()` and then throw the
output away.

When we run this, we get a bunch of scary-looking output, eventually ending
with

```text
thread 'main' panicked at 'Test failed: byte index 4 is not a char boundary; it is inside 'ௗ' (bytes 2..5) of `aAௗ0㌀0`; minimal failing input: s = "aAௗ0㌀0"
	successes: 102
	local rejects: 0
	global rejects: 0
'
```

If we look at the top directory after the test fails, we'll see a new
`proptest-regressions` directory, which contains some files corresponding to
source files containing failing test cases. These are [*failure persistence*](https://proptest-rs.github.io/proptest/proptest/failure-persistence.html)
files. The first thing we should do is add these to source control.

```text
$ git add proptest-regressions
```

The next thing we should do is copy the failing case to a traditional unit
test since it has exposed a bug not similar to what we've tested in the
past.

```rust,should_panic
# fn parse_date(s: &str) -> Option<(u32, u32, u32)> {
#     if 10 != s.len() { return None; }
#     if "-" != &s[4..5] || "-" != &s[7..8] { return None; }
#
#     let year = &s[0..4];
#     let month = &s[6..7];
#     let day = &s[8..10];
#
#     year.parse::<u32>().ok().and_then(
#         |y| month.parse::<u32>().ok().and_then(
#             |m| day.parse::<u32>().ok().map(
#                 |d| (y, m, d))))
# }
#[test]
# fn dummy() {} // Doctests don't build `#[test]` functions, so we need this
fn test_unicode_gibberish() {
    assert_eq!(None, parse_date("aAௗ0㌀0"));
}
# fn main() { test_unicode_gibberish(); }
```

Now, let's see what happened... we forgot about UTF-8! You can't just
blindly slice strings since you could split a character, in this case that
Tamil diacritic placed atop other characters in the string.

In the interest of making the code changes as small as possible, we'll just
check that the string is ASCII and reject anything that isn't.

```rust
# use std::ascii::AsciiExt;
#
fn parse_date(s: &str) -> Option<(u32, u32, u32)> {
    if 10 != s.len() { return None; }

    // NEW: Ignore non-ASCII strings so we don't need to deal with Unicode.
    if !s.is_ascii() { return None; }

    if "-" != &s[4..5] || "-" != &s[7..8] { return None; }

    let year = &s[0..4];
    let month = &s[6..7];
    let day = &s[8..10];

    year.parse::<u32>().ok().and_then(
        |y| month.parse::<u32>().ok().and_then(
            |m| day.parse::<u32>().ok().map(
                |d| (y, m, d))))
}
```

The tests pass now! But we know there are still more problems, so let's
test more properties.

Another property we want from our code is that it parses every valid date.
We can add another test to the `proptest!` section:

```rust
# extern crate proptest;
# use proptest::prelude::*;
# fn parse_date(s: &str) -> Option<(u32, u32, u32)> {
#     if 10 != s.len() { return None; }
#
#     // NEW: Ignore non-ASCII strings so we don't need to deal with Unicode.
#     if !s.is_ascii() { return None; }
#
#     if "-" != &s[4..5] || "-" != &s[7..8] { return None; }
#
#     let year = &s[0..4];
#     let month = &s[6..7];
#     let day = &s[8..10];
#
#     year.parse::<u32>().ok().and_then(
#         |y| month.parse::<u32>().ok().and_then(
#             |m| day.parse::<u32>().ok().map(
#                 |d| (y, m, d))))
# }
proptest! {
    #[test]
    # fn dummy(0..1) {} // Doctests don't build `#[test]` functions, so we need this
    fn parses_all_valid_dates(s in "[0-9]{4}-[0-9]{2}-[0-9]{2}") {
        parse_date(&s).unwrap();
    }
}
# fn main() { parses_all_valid_dates(); }
```

The thing to the right-hand side of `in` is actually a *regular
expression*, and `s` is chosen from strings which match it. So in our
previous test, `"\\PC*"` was generating arbitrary strings composed of
arbitrary non-control characters. Now, we generate things in the YYYY-MM-DD
format.

The new test passes, so let's move on to something else.

The final property we want to check is that the dates are actually parsed
*correctly*. Now, we can't do this by generating strings — we'd end up just
reimplementing the date parser in the test! Instead, we start from the
expected output, generate the string, and check that it gets parsed back.

```rust
# extern crate proptest;
# use proptest::prelude::*;
# fn parse_date(s: &str) -> Option<(u32, u32, u32)> {
#     if 10 != s.len() { return None; }
#
#     // NEW: Ignore non-ASCII strings so we don't need to deal with Unicode.
#     if !s.is_ascii() { return None; }
#
#     if "-" != &s[4..5] || "-" != &s[7..8] { return None; }
#
#     let year = &s[0..4];
#     let month = &s[6..7];
#     let day = &s[8..10];
#
#     year.parse::<u32>().ok().and_then(
#         |y| month.parse::<u32>().ok().and_then(
#             |m| day.parse::<u32>().ok().map(
#                 |d| (y, m, d))))
# }
proptest! {
    #[test]
    # fn dummy(0..1) {} // Doctests don't build `#[test]` functions, so we need this
    fn parses_date_back_to_original(y in 0u32..10000,
                                    m in 1u32..13, d in 1u32..32) {
        let (y2, m2, d2) = parse_date(
            &format!("{:04}-{:02}-{:02}", y, m, d)).unwrap();
        // prop_assert_eq! is basically the same as assert_eq!, but doesn't
        // cause a bunch of panic messages to be printed on intermediate
        // test failures. Which one to use is largely a matter of taste.
        prop_assert_eq!((y, m, d), (y2, m2, d2));
    }
}
```

Here, we see that besides regexes, we can use any expression which is a
`proptest::strategy::Strategy`, in this case, integer ranges.

The test fails when we run it. Though there's not much output this time.

```text
thread 'main' panicked at 'Test failed: assertion failed: `(left == right)` (left: `(0, 10, 1)`, right: `(0, 0, 1)`) at examples/dateparser_v2.rs:46; minimal failing input: y = 0, m = 10, d = 1
	successes: 2
	local rejects: 0
	global rejects: 0
', examples/dateparser_v2.rs:33
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

The failing input is `(y, m, d) = (0, 10, 1)`, which is a rather specific
output. Before thinking about why this breaks the code, let's look at what
proptest did to arrive at this value. At the start of our test function,
insert

```rust
# let (y, m, d) = (0, 10, 1);
println!("y = {}, m = {}, d = {}", y, m, d);
```

Running the test again, we get something like this:

```text
y = 2497, m = 8, d = 27
y = 9641, m = 8, d = 18
y = 7360, m = 12, d = 20
y = 3680, m = 12, d = 20
y = 1840, m = 12, d = 20
y = 920, m = 12, d = 20
y = 460, m = 12, d = 20
y = 230, m = 12, d = 20
y = 115, m = 12, d = 20
y = 57, m = 12, d = 20
y = 28, m = 12, d = 20
y = 14, m = 12, d = 20
y = 7, m = 12, d = 20
y = 3, m = 12, d = 20
y = 1, m = 12, d = 20
y = 0, m = 12, d = 20
y = 0, m = 6, d = 20
y = 0, m = 9, d = 20
y = 0, m = 11, d = 20
y = 0, m = 10, d = 20
y = 0, m = 10, d = 10
y = 0, m = 10, d = 5
y = 0, m = 10, d = 3
y = 0, m = 10, d = 2
y = 0, m = 10, d = 1
```

The test failure message said there were two successful cases; we see these
at the very top, `2497-08-27` and `9641-08-18`. The next case,
`7360-12-20`, failed. There's nothing immediately obviously special about
this date. Fortunately, proptest reduced it to a much simpler case. First,
it rapidly reduced the `y` input to `0` at the beginning, and similarly
reduced the `d` input to the minimum allowable value of `1` at the end.
Between those two, though, we see something different: it tried to shrink
`12` to `6`, but then ended up raising it back up to `10`. This is because
the `0000-06-20` and `0000-09-20` test cases *passed*.

In the end, we get the date `0000-10-01`, which apparently gets parsed as
`0000-00-01`. Again, this failing case was added to the failure persistence
file, and we should add this as its own unit test:

```text
$ git add proptest-regressions
```

```rust,should_panic
# fn parse_date(s: &str) -> Option<(u32, u32, u32)> {
#     if 10 != s.len() { return None; }
#
#     // NEW: Ignore non-ASCII strings so we don't need to deal with Unicode.
#     if !s.is_ascii() { return None; }
#
#     if "-" != &s[4..5] || "-" != &s[7..8] { return None; }
#
#     let year = &s[0..4];
#     let month = &s[6..7];
#     let day = &s[8..10];
#
#     year.parse::<u32>().ok().and_then(
#         |y| month.parse::<u32>().ok().and_then(
#             |m| day.parse::<u32>().ok().map(
#                 |d| (y, m, d))))
# }
#[test]
# fn dummy() {} // Doctests don't build `#[test]` functions, so we need this
fn test_october_first() {
    assert_eq!(Some((0, 10, 1)), parse_date("0000-10-01"));
}
# fn main() { test_october_first(); }
```

Now to figure out what's broken in the code. Even without the intermediate
input, we can say with reasonable confidence that the year and day parts
don't come into the picture since both were reduced to the minimum
allowable input. The month input was *not*, but was reduced to `10`. This
means we can infer that there's something special about `10` that doesn't
hold for `9`. In this case, that "special something" is being two digits
wide. In our code:

```rust,ignore
let month = &s[6..7];
```

We were off by one, and need to use the range `5..7`. After fixing this,
the test passes.

The `proptest!` macro has some additional syntax, including for setting
configuration for things like the number of test cases to generate. See its
[documentation](https://docs.rs/proptest/latest/proptest/macro.proptest.html)
for more details.

---

# Proptest from the Bottom Up

This tutorial will introduce proptest from the bottom up, starting from the
basic building blocks, in the hopes of making the model as a whole clear.
In particular, we'll start off without using the macros so that the macros
can later be understood in terms of what they expand into rather than
magic. But as a result, the first part is *not* representative of how
proptest is normally used. If bottom-up isn't your style, you may wish to
skim the first few sections.

Also note that the examples here focus on the usage of proptest itself, and
as such generally have trivial test bodies. In real code, you would
obviously have assertions and so forth in the test bodies.

---

# Strategy Basics

Please make sure to read the [introduction to this tutorial](index.md) before
starting this section.

The [*Strategy*](https://docs.rs/proptest/latest/proptest/strategy/trait.Strategy.html) is the most fundamental
concept in proptest. A strategy defines two things:

- How to generate random values of a particular type from a random number
  generator.

- How to "shrink" such values into "simpler" forms.

Proptest ships with a substantial library of strategies. Some of these are
defined in terms of built-in types; for example, `0..100i32` is a strategy
to generate `i32`s between 0, inclusive, and 100, exclusive. As we've
already seen, strings are themselves strategies for generating strings
which match the former as a regular expression.

Generating a value is a two-step process. First, a `TestRunner` is passed
to the `new_tree()` method of the `Strategy`; this returns a `ValueTree`,
which we'll look at in more detail momentarily. Calling the `current()`
method on the `ValueTree` produces the actual value. Knowing that, we can
put the pieces together and generate values. The below is the
`tutorial-strategy-play.rs` example:

```rust
# extern crate proptest;
use proptest::test_runner::TestRunner;
use proptest::strategy::{Strategy, ValueTree};

fn main() {
    let mut runner = TestRunner::default();
    let int_val = (0..100i32).new_tree(&mut runner).unwrap();
    let str_val = "[a-z]{1,4}\\p{Cyrillic}{1,4}\\p{Greek}{1,4}"
        .new_tree(&mut runner).unwrap();
    println!("int_val = {}, str_val = {}",
             int_val.current(), str_val.current());
}
```

If you run this a few times, you'll get output similar to the following:

```text
$ target/debug/examples/tutorial-strategy-play
int_val = 99, str_val = vѨͿἕΌ
$ target/debug/examples/tutorial-strategy-play
int_val = 25, str_val = cwᵸійΉ
$ target/debug/examples/tutorial-strategy-play
int_val = 5, str_val = oegiᴫᵸӈᵸὛΉ
```

This knowledge is sufficient to build an extremely primitive fuzzing test.

```rust
# extern crate proptest;
use proptest::test_runner::TestRunner;
use proptest::strategy::{Strategy, ValueTree};

fn some_function(v: i32) {
    // Do a bunch of stuff, but crash if v > 500
    assert!(v <= 500);
}

#[test]
fn some_function_doesnt_crash() {
    let mut runner = TestRunner::default();
    for _ in 0..256 {
        let val = (0..10000i32).new_tree(&mut runner).unwrap();
        some_function(val.current());
    }
}
```

This *works*, but when the test fails, we don't get much context, and even
if we recover the input, we see some arbitrary-looking value like 1771
rather than the boundary condition of 501. For a function taking just an
integer, this is probably still good enough, but as inputs get more
complex, interpreting completely random values becomes increasingly
difficult.

---

# Shrinking Basics

Finding the "simplest" input that causes a test failure is referred to as
*shrinking*. This is where the intermediate `ValueTree` type comes in.
Besides `current()`, it provides two methods — `simplify()` and
`complicate()` — which together allow binary searching over the input
space. The `tutorial-simplify-play.rs` example shows how repeated calls to
`simplify()` produce incrementally "simpler" outputs, both in terms of size
and in characters used.

```rust
# extern crate proptest;
use proptest::test_runner::TestRunner;
use proptest::strategy::{Strategy, ValueTree};

fn main() {
    let mut runner = TestRunner::default();
    let mut str_val = "[a-z]{1,4}\\p{Cyrillic}{1,4}\\p{Greek}{1,4}"
        .new_tree(&mut runner).unwrap();
    println!("str_val = {}", str_val.current());
    while str_val.simplify() {
        println!("        = {}", str_val.current());
    }
}
```

A couple runs:

```text
$ target/debug/examples/tutorial-simplify-play
str_val = vy꙲ꙈᴫѱΆῨῨ
        = y꙲ꙈᴫѱΆῨῨ
        = y꙲ꙈᴫѱΆῨῨ
        = m꙲ꙈᴫѱΆῨῨ
        = g꙲ꙈᴫѱΆῨῨ
        = d꙲ꙈᴫѱΆῨῨ
        = b꙲ꙈᴫѱΆῨῨ
        = a꙲ꙈᴫѱΆῨῨ
        = aꙈᴫѱΆῨῨ
        = aᴫѱΆῨῨ
        = aѱΆῨῨ
        = aѱΆῨῨ
        = aѱΆῨῨ
        = aиΆῨῨ
        = aМΆῨῨ
        = aЎΆῨῨ
        = aЇΆῨῨ
        = aЃΆῨῨ
        = aЁΆῨῨ
        = aЀΆῨῨ
        = aЀῨῨ
        = aЀῨ
        = aЀῨ
        = aЀῢ
        = aЀ῟
        = aЀ῞
        = aЀ῝
$ target/debug/examples/tutorial-simplify-play
str_val = dyiꙭᾪῇΊ
        = yiꙭᾪῇΊ
        = iꙭᾪῇΊ
        = iꙭᾪῇΊ
        = iꙭᾪῇΊ
        = eꙭᾪῇΊ
        = cꙭᾪῇΊ
        = bꙭᾪῇΊ
        = aꙭᾪῇΊ
        = aꙖᾪῇΊ
        = aꙋᾪῇΊ
        = aꙅᾪῇΊ
        = aꙂᾪῇΊ
        = aꙁᾪῇΊ
        = aꙀᾪῇΊ
        = aꙀῇΊ
        = aꙀΊ
        = aꙀΊ
        = aꙀΊ
        = aꙀΉ
        = aꙀΈ
```

Note that shrinking never shrinks a value to something outside the range
the strategy describes. Notice the strings in the above example still match
the regular expression even in the end. An integer drawn from
`100..1000i32` will shrink towards zero, but will stop at 100 since that is
the minimum value.

`simplify()` and `complicate()` can be used to adapt our primitive fuzz
test to actually find the boundary condition.

```rust
# extern crate proptest;
use proptest::test_runner::TestRunner;
use proptest::strategy::{Strategy, ValueTree};

fn some_function(v: i32) -> bool {
    // Do a bunch of stuff, but crash if v > 500
    // assert!(v <= 500);
    // But return a boolean instead of panicking for simplicity
    v <= 500
}

// We know the function is broken, so use a purpose-built main function to
// find the breaking point.
fn main() {
    let mut runner = TestRunner::default();
    for _ in 0..256 {
        let mut val = (0..10000i32).new_tree(&mut runner).unwrap();
        if some_function(val.current()) {
            // Test case passed
            continue;
        }

        // We found our failing test case, simplify it as much as possible.
        loop {
            if !some_function(val.current()) {
                // Still failing, find a simpler case
                if !val.simplify() {
                    // No more simplification possible; we're done
                    break;
                }
            } else {
                // Passed this input, back up a bit
                if !val.complicate() {
                    break;
                }
            }
        }

        println!("The minimal failing case is {}", val.current());
        assert_eq!(501, val.current());
        return;
    }
    panic!("Didn't find a failing test case");
}
```

This code reliably finds the boundary of the failure, 501.

---

# Using the Test Runner

Rather than manually shrinking, proptest's
[`TestRunner`](https://docs.rs/proptest/latest/proptest/test_runner/struct.TestRunner.html)
provides this functionality for us and additionally handles things like panics.
The method we're interested in is `run`. We simply give it the strategy and a
function to test inputs and it takes care of the rest.

```rust
# extern crate proptest;
use proptest::test_runner::{Config, FileFailurePersistence,
                            TestError, TestRunner};

fn some_function(v: i32) {
    // Do a bunch of stuff, but crash if v > 500.
    // We return to normal `assert!` here since `TestRunner` catches
    // panics.
    assert!(v <= 500);
}

// We know the function is broken, so use a purpose-built main function to
// find the breaking point.
fn main() {
    let mut runner = TestRunner::new(Config {
        // Turn failure persistence off for demonstration
        failure_persistence: Some(Box::new(FileFailurePersistence::Off)),
        .. Config::default()
    });
    let result = runner.run(&(0..10000i32), |v| {
        some_function(v);
        Ok(())
    });
    match result {
        Err(TestError::Fail(_, value)) => {
            println!("Found minimal failing case: {}", value);
            assert_eq!(501, value);
        },
        result => panic!("Unexpected result: {:?}", result),
    }
}
```

That's a lot better! Still a bit boilerplatey; the `proptest!` macro will
help with that, but it does some other stuff we haven't covered yet, so for
the moment we'll keep using `TestRunner` directly.

---

# Compound Strategies

Testing functions that take single arguments of primitive types is nice and
all, but is kind of underwhelming. Back when we were writing the whole
stack by hand, extending the technique to, say, *two* integers was clear,
if verbose. But `TestRunner` only takes a single `Strategy`; how can we
test a function that needs inputs from more than one?

```rust,ignore
# extern crate proptest;
use proptest::test_runner::TestRunner;

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
# fn dummy() {} // Doctests don't build `#[test]` functions, so we need this
fn test_add() {
    let mut runner = TestRunner::default();
    runner.run(/* uhhm... */).unwrap();
}
# fn main() { test_add(); }
```

The key is that strategies are *composable*. The simplest form of
composition is "compound strategies", where we take multiple strategies and
combine their values into one value that holds each input separately. There
are several of these. The simplest is a tuple; a tuple of strategies is
itself a strategy for tuples of the values those strategies produce. For
example, `(0..100i32,100..1000i32)` is a strategy for pairs of integers
where the first value is between 0 and 100 and the second is between 100
and 1000.

So for our two-argument function, our strategy is simply a tuple of ranges.

```rust
# extern crate proptest;
use proptest::test_runner::TestRunner;

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
# fn dummy() {} // Doctests don't build `#[test]` functions, so we need this
fn test_add() {
    let mut runner = TestRunner::default();
    // Combine our two inputs into a strategy for one tuple. Our test
    // function then destructures the generated tuples back into separate
    // `a` and `b` variables to be passed in to `add()`.
    runner.run(&(0..1000i32, 0..1000i32), |(a, b)| {
        let sum = add(a, b);
        assert!(sum >= a);
        assert!(sum >= b);
        Ok(())
    }).unwrap();
}
# fn main() { test_add(); }
```

Other compound strategies include fixed-sizes arrays of strategies and
`Vec`s of strategies (which produce arrays or `Vec`s of values parallel to
the strategy collection), as well as the various strategies provided in the
[collection](https://docs.rs/proptest/latest/proptest/collection/index.html) module.

---

# Syntax Sugar: `proptest!`

Now that we know about compound strategies, we can understand how the
[`proptest!`](https://docs.rs/proptest/latest/proptest/macro.proptest.html)
macro works. Our example from the prior section can be rewritten using that
macro like so:

```rust
# extern crate proptest;
use proptest::prelude::*;

fn add(a: i32, b: i32) -> i32 {
    a + b
}

proptest! {
    #[test]
    # fn dummy(0..1) {} // Doctests don't build `#[test]` functions, so we need this
    fn test_add(a in 0..1000i32, b in 0..1000i32) {
        let sum = add(a, b);
        assert!(sum >= a);
        assert!(sum >= b);
    }
}
#
# fn main() { test_add(); }
```

Conceptually, the desugaring process is fairly simple. At the start of the
test function, a new `TestRunner` is constructed. The input strategies
(after the `in` keyword) are grouped into a tuple. That tuple is passed in
to the `TestRunner` as the input strategy. The test body has `Ok(())` added
to the end, then is put into a lambda that destructures the generated input
tuple back into the named parameters and then runs the body. The end result
is extremely similar to what we wrote by hand in the prior section.

`proptest!` actually does a few other things in order to make failure
output easier to read and to overcome the 10-tuple limit.

---

# Transforming Strategies

Suppose you have a function that takes a string which needs to be the
`Display` format of an arbitrary `u32`. A first attempt to providing this
argument might be to use a regular expression, like so:

```rust
# extern crate proptest;
use proptest::prelude::*;

fn do_stuff(v: String) {
    let i: u32 = v.parse().unwrap();
    let s = i.to_string();
    assert_eq!(s, v);
}

proptest! {
    #[test]
    # fn dummy(0..1) {} // Doctests don't build `#[test]` functions, so we need this
    fn test_do_stuff(v in "[1-9][0-9]{0,8}") {
        do_stuff(v);
    }
}
# fn main() { test_do_stuff(); }
```

This kind of works, but it has problems. For one, it does not explore the
whole `u32` space. It is possible to write a regular expression that does,
but such an expression is rather long, and also results in a pretty odd
distribution of values. The input also doesn't shrink correctly, since
proptest tries to shrink it in terms of a string rather than an integer.

What you really want to do is generate a `u32` and then pass in its string
representation. One way to do this is to just take `u32` as an input to the
test and then transform it to a string within the test code. This approach
works fine, but isn't reusable or composable. Ideally, we could get a
*strategy* that does this.

The thing we're looking for is the first strategy *combinator*, `prop_map`.
We need to ensure `Strategy` is in scope to use it.

```rust
# extern crate proptest;
// Grab `Strategy`, shorter namespace prefix, and the macros
use proptest::prelude::*;

fn do_stuff(v: String) {
    let i: u32 = v.parse().unwrap();
    let s = i.to_string();
    assert_eq!(s, v);
}

proptest! {
    #[test]
    # fn dummy(0..1) {} // Doctests don't build `#[test]` functions, so we need this
    fn test_do_stuff(v in any::<u32>().prop_map(|v| v.to_string())) {
        do_stuff(v);
    }
}
# fn main() { test_do_stuff(); }
```

Calling `prop_map` on a `Strategy` creates a new strategy which transforms
every generated value using the provided function. Proptest retains the
relationship between the original `Strategy` and the transformed one; as a
result, shrinking occurs in terms of `u32`, even though we're generating a
`String`.

`prop_map` is also the principal way to define strategies for new types,
since most types are simply composed of other, simpler values.

Let's update our code so it takes a more interesting structure.

```rust
# extern crate proptest;
use proptest::prelude::*;

#[derive(Clone, Debug)]
struct Order {
  id: String,
  // Some other fields, though the test doesn't do anything with them
  item: String,
  quantity: u32,
}

fn do_stuff(order: Order) {
    let i: u32 = order.id.parse().unwrap();
    let s = i.to_string();
    assert_eq!(s, order.id);
}

proptest! {
    #[test]
    # fn dummy(0..1) {} // Doctests don't build `#[test]` functions, so we need this
    fn test_do_stuff(
        order in
        (any::<u32>().prop_map(|v| v.to_string()),
         "[a-z]*", 1..1000u32).prop_map(
             |(id, item, quantity)| Order { id, item, quantity })
    ) {
        do_stuff(order);
    }
}
# fn main() { test_do_stuff(); }
```

Notice how we were able to take the output from `prop_map` and put it in a
tuple, then call `prop_map` on *that* tuple to produce yet another value.

But that's quite a mouthful in the argument list. Fortunately, strategies
are normal values, so we can extract it to a function.

```rust
# extern crate proptest;
use proptest::prelude::*;

// snip
#
# #[derive(Clone, Debug)]
# struct Order {
#   id: String,
#   // Some other fields, though the test doesn't do anything with them
#   item: String,
#   quantity: u32,
# }
#
# fn do_stuff(order: Order) {
#     let i: u32 = order.id.parse().unwrap();
#     let s = i.to_string();
#     assert_eq!(s, order.id);
# }
#
fn arb_order(max_quantity: u32) -> BoxedStrategy<Order> {
    (any::<u32>().prop_map(|v| v.to_string()),
     "[a-z]*", 1..max_quantity)
    .prop_map(|(id, item, quantity)| Order { id, item, quantity })
    .boxed()
}

proptest! {
    #[test]
    # fn dummy(0..1) {} // Doctests don't build `#[test]` functions, so we need this
    fn test_do_stuff(order in arb_order(1000)) {
        do_stuff(order);
    }
}
# fn main() { test_do_stuff(); }
```

We `boxed()` the strategy in the function since otherwise the type would
not be nameable, and even if it were, it would be very hard to read or
write. Boxing a `Strategy` turns both it and its `ValueTree`s into trait
objects, which both makes the types simpler and can be used to mix
heterogeneous `Strategy` types as long as they produce the same value
types.

The `arb_order()` function is also *parameterised*, which is another
advantage of extracting strategies to separate functions. In this case, if
we have a test that needs an `Order` with no more than a dozen items, we
can simply call `arb_order(12)` rather than needing to write out a whole
new strategy.

We can also use `-> impl Strategy<Value = Order>` instead to avoid the
overhead as in the following example. You should use `-> impl Strategy<..>`
unless you need the dynamic dispatch.

```rust
# extern crate proptest;
use proptest::prelude::*;

// snip
#
# #[derive(Clone, Debug)]
# struct Order {
#   id: String,
#   // Some other fields, though the test doesn't do anything with them
#   item: String,
#   quantity: u32,
# }
#
# fn do_stuff(order: Order) {
#     let i: u32 = order.id.parse().unwrap();
#     let s = i.to_string();
#     assert_eq!(s, order.id);
# }
#
fn arb_order(max_quantity: u32) -> impl Strategy<Value = Order> {
    (any::<u32>().prop_map(|v| v.to_string()),
     "[a-z]*", 1..max_quantity)
    .prop_map(|(id, item, quantity)| Order { id, item, quantity })
}

proptest! {
    #[test]
    # fn dummy(0..1) {} // Doctests don't build `#[test]` functions, so we need this
    fn test_do_stuff(order in arb_order(1000)) {
        do_stuff(order);
    }
}

# fn main() { test_do_stuff(); }
```

---

# Syntax Sugar: `prop_compose!`

Defining strategy-returning functions like this is extremely useful, but
the code above is a bit verbose, as well as hard to read for similar
reasons to writing test functions by hand.

To simplify this task, proptest includes the
[`prop_compose!`](https://docs.rs/proptest/latest/proptest/macro.prop_compose.html)
macro. Before going into details, here's our code from above rewritten to use
it.

```rust
# extern crate proptest;
use proptest::prelude::*;

// snip
#
# #[derive(Clone, Debug)]
# struct Order {
#   id: String,
#   // Some other fields, though the test doesn't do anything with them
#   item: String,
#   quantity: u32,
# }
#
# fn do_stuff(order: Order) {
#     let i: u32 = order.id.parse().unwrap();
#     let s = i.to_string();
#     assert_eq!(s, order.id);
# }
prop_compose! {
    fn arb_order_id()(id in any::<u32>()) -> String {
        id.to_string()
    }
}
prop_compose! {
    fn arb_order(max_quantity: u32)
                (id in arb_order_id(), item in "[a-z]*",
                 quantity in 1..max_quantity)
                -> Order {
        Order { id, item, quantity }
    }
}

proptest! {
    #[test]
    # fn dummy(0..1) {} // Doctests don't build `#[test]` functions, so we need this
    fn test_do_stuff(order in arb_order(1000)) {
        do_stuff(order);
    }
}
# fn main() { test_do_stuff(); }
```

We had to extract `arb_order_id()` out into its own function, but otherwise
this desugars to almost exactly what we wrote in the previous section. The
generated function takes the first parameter list as arguments. These
arguments are used to select the strategies in the second argument list.
Values are then drawn from those strategies and transformed by the function
body. The actual function has a return type of `impl Strategy<Value = T>`
where `T` is the declared return type.

---

# Generating Enums

The syntax sugar for defining strategies for `enum`s is currently somewhat
limited. Creating such strategies with `prop_compose!` is possible but
generally is not very readable, so in most cases defining the function by
hand is preferable.

The core building block is the
[`prop_oneof!`](https://docs.rs/proptest/latest/proptest/macro.prop_oneof.html)
macro, in which you list one case for each case in your `enum`. For `enum`s
which have no data, the strategy for each case is
`Just(YourEnum::TheCase)`. Enum cases with data generally require putting
the data in a tuple and then using `prop_map` to map it into the enum case.

Here is a simple example:

```rust
# extern crate proptest;
use proptest::prelude::*;

#[derive(Debug, Clone)]
enum MyEnum {
    SimpleCase,
    CaseWithSingleDatum(u32),
    CaseWithMultipleData(u32, String),
}

fn my_enum_strategy() -> impl Strategy<Value = MyEnum> {
  prop_oneof![
    // For cases without data, `Just` is all you need
    Just(MyEnum::SimpleCase),

    // For cases with data, write a strategy for the interior data, then
    // map into the actual enum case.
    any::<u32>().prop_map(MyEnum::CaseWithSingleDatum),

    (any::<u32>(), ".*").prop_map(
      |(a, b)| MyEnum::CaseWithMultipleData(a, b)),
  ]
}
```

In general, it is best to list the enum cases in order from "simplest" to
"most complex", since shrinking will shrink down toward items earlier in
the list.

For particularly complex enum cases, it can be helpful to extract the strategy
for that case to a separate strategy. Here,
[`prop_compose!`](https://docs.rs/proptest/latest/proptest/macro.prop_compose.html)
can be of use.

```rust
# extern crate proptest;
use proptest::prelude::*;

#[derive(Debug, Clone)]
enum MyComplexEnum {
    SimpleCase,
    AnotherSimpleCase,
    ComplexCase {
        product_code: String,
        id: u64,
        chapter: String,
    },
}

prop_compose! {
  fn my_complex_enum_complex_case()(
      product_code in "[0-9A-Z]{10,20}",
      id in 1u64..10000u64,
      chapter in "X{0,2}(V?I{1,3}|IV|IX)",
  ) -> MyComplexEnum {
      MyComplexEnum::ComplexCase { product_code, id, chapter }
  }
}

fn my_enum_strategy() -> BoxedStrategy<MyComplexEnum> {
  prop_oneof![
    Just(MyComplexEnum::SimpleCase),
    Just(MyComplexEnum::AnotherSimpleCase),
    my_complex_enum_complex_case(),
  ].boxed()
}
```

---

# Filtering

Sometimes, you have a case where your input values have some sort of
"irregular" constraint on them. For example, an integer needing to be even,
or two values needing to be non-equal.

In general, the ideal solution is to find a way to take a seed value and
then use `prop_map` to transform it into the desired, irregular domain. For
example, to generate even integers, use something like

```rust
# extern crate proptest;
use proptest::prelude::*;
prop_compose! {
    // Generate arbitrary integers up to half the maximum desired value,
    // then multiply them by 2, thus producing only even integers in the
    // desired range.
    fn even_integer(max: i32)(base in 0..max/2) -> i32 { base * 2 }
}
```

For the cases where this is not viable, it is possible to filter
strategies. Proptest actually divides filters into two categories:

- "Local" filters apply to a single strategy. If a value is rejected,
  a new value is drawn from that strategy only.

- "Global" filters apply to the whole test case. If the test case is
  rejected, the whole thing is regenerated.

The distinction is somewhat arbitrary, since something like a "global
filter" could be created by just putting a "local filter" around the whole
input strategy. In practise, the distinction is as to what code performs
the rejection.

A local filter is created with the `prop_filter` combinator. Besides a
function indicating whether to accept the value, it also takes a value of
type `&'static str`, `String`, .., which it uses to record where/why the
rejection happened.

```rust
# extern crate proptest;
use proptest::prelude::*;

proptest! {
    #[test]
    # fn dummy(0..1) {} // Doctests don't build `#[test]` functions, so we need this
    fn some_test(
      v in (0..1000u32)
        .prop_filter("Values must not divisible by 7 xor 11",
                     |v| !((0 == v % 7) ^ (0 == v % 11)))
    ) {
        assert_eq!(0 == v % 7, 0 == v % 11);
    }
}
# fn main() { some_test(); }
```

Global filtering results when a test itself returns
`Err(TestCaseError::Reject)`. The
[`prop_assume!`](https://docs.rs/proptest/latest/proptest/macro.prop_assume.html)
macro provides an easy way to do this.

```rust
# extern crate proptest;
use proptest::prelude::*;

fn frob(a: i32, b: i32) -> (i32, i32) {
    let d = (a - b).abs();
    (a / d, b / d)
}

proptest! {
    #[test]
    # fn dummy(0..1) {} // Doctests don't build `#[test]` functions, so we need this
    fn test_frob(a in -1000..1000, b in -1000..1000) {
        // Input illegal if a==b.
        // Equivalent to
        // if (a == b) { return Err(TestCaseError::Reject(...)); }
        prop_assume!(a != b);

        let (a2, b2) = frob(a, b);
        assert!(a2.abs() <= a.abs());
        assert!(b2.abs() <= b.abs());
    }
}
# fn main() { test_frob(); }
```

While useful, filtering has a lot of disadvantages:

- Since it is simply rejection sampling, it will slow down generation of test
  cases since values need to be generated additional times to satisfy the
  filter. In the case where a filter always returns false, a test could
  theoretically never generate a result.

- Proptest tracks how many local and global rejections have happened, and
  aborts if they exceed a certain number. This prevents a test taking an
  extremely long time due to rejections, but means not all filters are viable
  in the default configuration. The limits for local and global rejections are
  different; by default, proptest allows a large number of local rejections but
  a fairly small number of global rejections, on the premise that the former
  are cheap but potentially common (having been built into the strategy) but
  the latter are expensive but rare (being an edge case in the particular
  test).

- Shrinking and filtering do not play well together. When shrinking, if a value
  winds up being rejected, there is no pass/fail information to continue
  shrinking properly. Instead, proptest treats such a rejection the same way it
  handles a shrink that results in a passing test: by backing away from
  simplification with a call to `complicate()`. Thus encountering a filter
  rejection during shrinking prevents shrinking from continuing to any simpler
  values, even if there are some that would be accepted by the filter.

---

# Generating Recursive Data

Randomly generating recursive data structures is trickier than it sounds. For
example, the below is a naïve attempt at generating a JSON AST by using
recursion.

```rust
# extern crate proptest;
use std::collections::HashMap;
use proptest::prelude::*;

#[derive(Clone, Debug)]
enum Json {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Map(HashMap<String, Json>),
}

fn arb_json() -> impl Strategy<Value = Json> {
    prop_oneof![
        Just(Json::Null),
        any::<bool>().prop_map(Json::Bool),
        any::<f64>().prop_map(Json::Number),
        ".*".prop_map(Json::String),
        prop::collection::vec(arb_json(), 0..10).prop_map(Json::Array),
        prop::collection::hash_map(
          ".*", arb_json(), 0..10).prop_map(Json::Map),
    ].boxed()
}
```

Upon closer consideration, this obviously can't work because `arb_json()`
recurses unconditionally.

A more sophisticated attempt is to define one strategy for each level of
nesting up to some maximum. This doesn't overflow the stack, but as defined
here, even four levels of nesting will produce trees with *thousands* of
nodes; by eight levels, we get to tens of *millions*.

Proptest provides a more reliable solution in the form of the
`prop_recursive` combinator. To use this, we create a strategy for the
non-recursive case, then give the combinator that strategy, some size
parameters, and a function to transform a nested strategy into a recursive
strategy.

```rust
# extern crate proptest;
use std::collections::HashMap;
use proptest::prelude::*;

#[derive(Clone, Debug)]
enum Json {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Map(HashMap<String, Json>),
}

fn arb_json() -> impl Strategy<Value = Json> {
    let leaf = prop_oneof![
        Just(Json::Null),
        any::<bool>().prop_map(Json::Bool),
        any::<f64>().prop_map(Json::Number),
        ".*".prop_map(Json::String),
    ];
    leaf.prop_recursive(
      8, // 8 levels deep
      256, // Shoot for maximum size of 256 nodes
      10, // We put up to 10 items per collection
      |inner| prop_oneof![
          // Take the inner strategy and make the two recursive cases.
          prop::collection::vec(inner.clone(), 0..10)
              .prop_map(Json::Array),
          prop::collection::hash_map(".*", inner, 0..10)
              .prop_map(Json::Map),
      ])
}
```

---

# Higher-Order Strategies

A *higher-order strategy* is a strategy which is generated by another
strategy. That sounds kind of scary, so let's consider an example first.

Say you have a function you want to test that takes a slice and an index
into that slice. If we use a fixed size for the slice, it's easy, but maybe
we need to test with different slice sizes. We could try something with a
filter:

```rust
# extern crate proptest;
use proptest::prelude::*;
fn some_function(stuff: &[String], index: usize) { /* do stuff */ }

proptest! {
    #[test]
    # fn dummy(0..1) {} // Doctests don't build `#[test]` functions, so we need this
    fn test_some_function(
        stuff in prop::collection::vec(".*", 1..100),
        index in 0..100usize
    ) {
        prop_assume!(index < stuff.len());
        some_function(&stuff, index);
    }
}
```

This doesn't work very well. First off, you get a lot of global rejections
since `index` will be outside of `stuff` 50% of the time. But secondly, it
will be rare to actually get a small `stuff` vector, since it would have to
randomly choose a small `index` at the same time.

The solution is the `prop_flat_map` combinator. This is sort of like
`prop_map`, except that the transform returns a *strategy* instead of a
value. This is more easily understood by implementing our example:

```rust
# extern crate proptest;
use proptest::prelude::*;

fn some_function(stuff: Vec<String>, index: usize) {
    let _ = &stuff[index];
    // Do stuff
}

fn vec_and_index() -> impl Strategy<Value = (Vec<String>, usize)> {
    prop::collection::vec(".*", 1..100)
        .prop_flat_map(|vec| {
            let len = vec.len();
            (Just(vec), 0..len)
        })
}

proptest! {
    #[test]
    # fn dummy(0..1) {} // Doctests don't build `#[test]` functions, so we need this
    fn test_some_function((vec, index) in vec_and_index()) {
        some_function(vec, index);
    }
}
# fn main() { test_some_function(); }
```

In `vec_and_index()`, we make a strategy to produce an arbitrary vector.
But then we derive a new strategy based on *values* produced by the first
one. The new strategy produces the generated vector unchanged, but also
adds a valid index into that vector, which we can do by picking the
strategy for that index based on the size of the vector.

Even though the new strategy specifies the singleton `Just(vec)` strategy
for the vector, proptest still understands the connection to the original
strategy and will shrink `vec` as well. All the while, `index` continues to
be a valid index into `vec`.

`prop_compose!` actually allows making second-order strategies like this by
simply providing three argument lists instead of two. The below desugars to
something much like what we wrote by hand above, except that the index and
vector's positions are internally reversed due to borrowing limitations.

```rust
# extern crate proptest;
# use proptest::prelude::*;
prop_compose! {
    fn vec_and_index()(vec in prop::collection::vec(".*", 1..100))
                    (index in 0..vec.len(), vec in Just(vec))
                    -> (Vec<String>, usize) {
       (vec, index)
   }
}
```

---

# Defining a canonical `Strategy` for a type

We previously used the function `any` as in `any::<u32>()` to generate a
strategy for all `u32`s. This function works with the trait `Arbitrary`,
which QuickCheck users may be familiar with. In proptest, this trait
is already implemented for most owned types in the standard library,
but you can of course implement it for your own types.

In some cases, where it makes sense to define a canonical strategy, such as in
the [JSON AST example](recursive.md), it is a good idea to implement
`Arbitrary`.

## Deriving `Arbitrary`

The experimental [`proptest-derive` crate](../../proptest-derive/index.md) can
be used to automate implementing `Arbitrary` in common cases. For example, imagine we have a struct that represents a point in a 2-D coordinate space:

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
```

This struct has the property that any pair of valid `i32`s can make a valid `Point`, so that is perfect for using `#[derive(Arbitrary)]`.

## Manual `Arbitrary` implementations

Sometimes, however, there are extra constraints that your type has, which the derive macro can't understand. In these cases, you'll need to implement `Arbitrary` for your type manually.

For example, consider this struct which represents a range (note, the derive API is can actually represent this case, it's just an example):

```rust
#[derive(Debug)]
struct Range {
    lower: i32,
    upper: i32,
}

impl Range {
    pub fn new(lower: i32, upper: i32) -> Option<Self> {
        if lower <= upper {
            Some(Self { lower, upper })
        } else {
            None
        }
    }
}
```

This struct has an invariant: `lower <= upper`. However, if we derive an `Arbitrary` implementation naively, it might generate `Range { lower: 1, upper: 0 }`.

Instead, we can write a manual implementation:

```rust
impl Arbitrary for Range {
    type Parameters = ();
    type Strategy = FilterMap<StrategyFor<(i32, i32)>, fn((i32, i32)) -> Option<Self>>;

    fn arbitrary_with(_parameters: Self::Parameters) -> Self::Strategy {
        any::<(i32, i32)>() // generate 2 arbitrary i32s
            .prop_map(|(a, b)| {
                let (lower, upper) = if a < b { (a, b) } else { (b, a) };
                Range::new(lower, upper).unwrap()
            })
    }
}
```

Here, there are three items we need to define:

- `type Parameters` - the type of any parameters to `arbitrary_with`. Here (and in many cases), we don't need this, so `()` is used.
- `type Strategy` - the type of the strategy produced
- `fn arbitrary_with` - the code that creates the canonical `Strategy` for this type

It's important to consider what type you want to use for `Strategy`. Here, we explicitly write the type out. This uses static dispatch, which is often faster and easier to optimize, but has a few downsides:

- you need to write out the type of the strategy. Even for this small function, it's a pretty lengthy function signature. In the worst case, it's impossible, since some types are unnameable (e.g. closures which capture their environment)
- it makes the implementation of `arbitrary_with` a part of your public API signature (if you expose `Arbitrary` impls in general from your crate). This means that changes to the implementation may require a breaking change.

There are a couple of ways around this:

- heap-allocate the strategy by:
  - returning `BoxedStrategy<T>`
  - calling `.boxed()` on the strategy before returning it
- use the nightly-only `#![feature(type_alias_impl_trait)]`:

```rust
type RangeStrategy = impl Strategy<Value = Range>;

impl Arbitrary for Range {
    type Parameters = ();
    type Strategy = RangeStrategy;
    // ...
}
```

Using `BoxedStrategy` will incur some performance penalty relating to a heap allocation as well as dynamic dispatch, but it works on stable (as of November 2022).

---

# Configuring the number of tests cases required

The default number of successful test cases that must execute for a test
as a whole to pass is currently 256. If you are not satisfied with this
and want to run more or fewer, there are a few ways to do this.

The first way is to set the environment-variable `PROPTEST_CASES` to a
value that can be successfully parsed as a `u32`. The value you set to this
variable is now the new default. (This only applies when the `std` feature of
proptest is enabled, which it is by default.)

Another way is to use `#![proptest_config(expr)]` inside `proptest!` where
`expr : Config`. To only change the number of test cases, you can simply
write:

```rust
# extern crate proptest;
use proptest::prelude::*;

fn add(a: i32, b: i32) -> i32 { a + b }

proptest! {
    // The next line modifies the number of tests.
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    # fn dummy(a in 0..1) {} // Doctests don't build `#[test]` functions, so we need this
    fn test_add(a in 0..1000i32, b in 0..1000i32) {
        let sum = add(a, b);
        assert!(sum >= a);
        assert!(sum >= b);
    }
}
# fn main() {
#     test_add();
# }
```

Through the same `proptest_config` mechanism you may fine-tune your
configuration through the `Config` type. See its documentation for more
information.

---

# Failure Persistence

By default, when Proptest finds a failing test case, it *persists* that
failing case in a file named after the source containing the failing test,
but in a separate directory tree rooted at `proptest-regressions`. Later
runs of tests will replay those test cases before generating novel cases.
This ensures that the test will not fail on one run and then spuriously
pass on the next, and also exposes similar tests to the same
known-problematic input.

(If you do not have an obvious source directory, you may instead find files
next to the source files, with a different extension.)

It is recommended to check these files in to your source control so that
other test runners (e.g., collaborators or a CI system) also replay these
cases.

Note that, by default, all tests in the same crate will share that one
persistence file. If you have a very large number of tests, it may be
desirable to separate them into smaller groups so the number of extra test
cases that get run is reduced. This can be done by adjusting the
`failure_persistence` flag on `Config`.

There are two ways this persistence could theoretically be done.

The immediately obvious option is to persist a representation of the value
itself, for example by using Serde. While this has some advantages,
particularly being resistant to changes like tweaking the input strategy,
it also has a lot of problems. Most importantly, there is no way to
determine whether any given value is actually within the domain of the
strategy that produces it. Thus, some (likely extremely fragile) mechanism
to ensure that the strategy that produced the value exactly matches the one
in use in a test case would be required.

The other option is to store the *seed* that was used to produce the
failing test case. This approach requires no support from the strategy or
the produced value. If the strategy in use differs from the one used to
produce failing case that was persisted, the seed may or may not produce
the problematic value, but nonetheless produces a valid value. Due to these
advantages, this is the approach Proptest uses.

---

# Forking and Timeouts

By default, proptest tests are run in-process and are allowed to run for
however long it takes them. This is resource-efficient and produces the nicest
test output, and for many use cases is sufficient. However, problems like
overflowing the stack, aborting the process, or getting stuck in an infinite
loop will simply break the entire test process and prevent proptest from
determining a minimal reproducible case.

As of version 0.7.1, proptest has optional "fork" and "timeout" features
(both enabled by default), which make it possible to run your test cases in
a subprocess and limit how long they may run. This is generally slower,
may make using a debugger more difficult, and makes test output harder to
interpret, but allows proptest to find and minimise test cases for these
situations as well.

To use these features, simply set the `fork` and/or `timeout` fields on the
`Config`. (Setting `timeout` implies `fork`.)

Here is a simple example of using both features:

```rust,should_panic
# extern crate proptest;
use proptest::prelude::*;

// The worst possible way to calculate Fibonacci numbers
fn fib(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        // Setting both fork and timeout is redundant since timeout implies
        // fork, but both are shown for clarity.
        fork: true,
        timeout: 100,
        # cases: 1, // Need to set this to 1 to avoid doctest running forever
        .. ProptestConfig::default()
    })]
    #[test]
    # fn dummy(0..1) {} // Doctests don't build `#[test]` functions, so we need this
    fn test_fib(n: u64) {
        // For large n, this will variously run for an extremely long time,
        // overflow the stack, or panic due to integer overflow.
        assert!(fib(n) >= n);
    }
}
# fn main() { test_fib(); }
```

The exact value of the test failure depends heavily on the performance of
the host system, the rust version, and compiler flags, but on the system
where it was originally tested, it found that the maximum value that
`fib()` could handle was 39, despite having dozens of processes dump core
due to stack overflow or time out along the way.

If you just want to run tests in subprocesses or with a timeout every now
and then, you can do that by setting the `PROPTEST_FORK` or
`PROPTEST_TIMEOUT` environment variables to alter the default
configuration. For example, on Unix,

```sh
# Run all the proptest tests in subprocesses with no timeout.
# Individual tests can still opt out by setting `fork: false` in their
# own configuration.
PROPTEST_FORK=true cargo test
# Run all the proptest tests in subprocesses with a 1 second timeout.
# Tests can still opt out or use a different timeout by setting `timeout: 0`
# or another timeout in their own configuration.
PROPTEST_TIMEOUT=1000 cargo test
```

---

# `no_std` Support

Proptest has partial support for being used in `no_std` contexts.

You will need a nightly compiler version. In your `Cargo.toml`, adjust the
Proptest dependency to look something like this:

```toml
[dev-dependencies.proptest]
version = "proptestVersion"

# Opt out of the `std` feature
default-features = false

# alloc: Use the `alloc` crate directly. Proptest has a hard requirement on
# memory allocation, so either this or `std` is needed.
# unstable: Enable use of nightly-only compiler features.
features = ["no_std", "alloc", "unstable"]
```

Some APIs are not available in the `no_std` build. This includes functionality
which necessarily needs `std` such as failure persistence and forking, as well
as features depending on other crates which do not support `no_std` usage, such
as regex support.

The `no_std` build may not have access to an entropy source (one exception are
x86-64 machines that support rdrand, in this case the library can be compiled
with the `hardware-rng` feature to get random numbers). If no entropy source is
available, every `TestRunner` (i.e., every `#[test]` when using the `proptest!`
macro) uses a single hard-coded seed. For complex inputs, it may be a good idea
to increase the number of test cases to compensate. The hard-coded seed is not
contractually guaranteed and may change between Proptest releases without
notice.

---

# Web Assembly support

As of 0.9.2, it is possible to compile proptest on `wasm` targets. Please note
that this is **highly experimental** and has not been subject to any
substantial amount of testing.

In `cargo.toml`, write something like

```toml
[dev-dependencies.proptest]
version = "$proptestVersion"
# The default feature set includes things like process forking which are not
# supported in Web Assembly.
default-features = false
# Enable using the `std` crate.
features = ["std"]
```

A few APIs are unavailable on `wasm` targets (beyond those which are removed by
deselecting certain default features):

- The `Arbitrary` implementation for `std::env::VarError`.

---

# Limitations of Property Testing

Given infinite time, property testing will eventually explore the whole
input space to a test. However, time is not infinite, so only a randomly
sampled portion of the input space can be explored. This means that
property testing is extremely unlikely to find single-value edge cases in a
large space. For example, the following test will virtually always pass:

```rust
# extern crate proptest;
use proptest::prelude::*;

proptest! {
    #[test]
    # fn dummy(0..1) {} // Doctests don't build `#[test]` functions, so we need this
    fn i64_abs_is_never_negative(a: i64) {
        // This actually fails if a == i64::MIN, but randomly picking one
        // specific value out of 2⁶⁴ is overwhelmingly unlikely.
        assert!(a.abs() >= 0);
    }
}
# fn main() { i64_abs_is_never_negative() }
```

Because of this, traditional unit testing with intelligently selected cases
is still necessary for many kinds of problems.

Similarly, in some cases it can be hard or impossible to define a strategy
which actually produces useful inputs. A strategy of `.{1,4096}` may be
great to fuzz a C parser, but is highly unlikely to produce anything that
makes it to a code generator.

---

# Differences between QuickCheck and Proptest

QuickCheck and Proptest are similar in many ways: both generate random
inputs for a function to check certain properties, and automatically shrink
inputs to minimal failing cases.

The one big difference is that QuickCheck generates and shrinks values
based on type alone, whereas Proptest uses explicit `Strategy` objects. The
QuickCheck approach has a lot of disadvantages in comparison:

- QuickCheck can only define one generator and shrinker per type. If you need a
  custom generation strategy, you need to wrap it in a newtype and implement
  traits on that by hand. In Proptest, you can define arbitrarily many
  different strategies for the same type, and there are plenty built-in.

- For the same reason, QuickCheck has a single "size" configuration that tries
  to define the range of values generated. If you need an integer between 0 and
  100 and another between 0 and 1000, you probably need to do another newtype.
  In Proptest, you can directly just express that you want a `0..100` integer
  and a `0..1000` integer.

- Types in QuickCheck are not easily composable. Defining `Arbitrary` and
  `Shrink` for a new struct which is simply produced by the composition of its
  fields requires implementing both by hand, including a bidirectional mapping
  between the struct and a tuple of its fields. In Proptest, you can make a
  tuple of the desired components and then `prop_map` it into the desired form.
  Shrinking happens automatically in terms of the input types.

- Because constraints on values cannot be expressed in QuickCheck, generation
  and shrinking may lead to a lot of input rejections. Strategies in Proptest
  are aware of simple constraints and do not generate or shrink to values that
  violate them.

The author of Hypothesis also has an [article on this topic](http://hypothesis.works/articles/integrated-shrinking/).

Of course, there's also some relative downsides that fall out of what
Proptest does differently:

- Generating complex values in Proptest can be up to an order of magnitude
  slower than in QuickCheck. This is because QuickCheck performs stateless
  shrinking based on the output value, whereas Proptest must hold on to all the
  intermediate states and relationships in order for its richer shrinking model
  to work.

---

# Reference documentation

For the API reference documentation, please see the [rustdoc documentation for
the `proptest`
crate](https://docs.rs/proptest/latest/proptest/).

---

# State Machine testing

The state machine testing support is available in the `proptest-state-machine` crate.

## When to use State Machine testing?

State machine testing automates the checking of properties of a system under test (SUT) against an abstract reference state machine definition. It does this by trying to discover a counter-example that breaks the defined properties of the system and attempts to shrink it to a minimal sequence of transitions that still reproduce the issue.

State machines are a very useful abstraction for reasoning about code. Many things from low-level to high-level logic and anywhere in between can be modelled as a state machine. They are very effective for modelling effectful code, that is code that performs some state changes that can be too hard to test thoroughly with a more manual approach or too complex to verify formally.

Some fitting examples to give you an idea include (by no means exhaustive):

- A data structure with an API that mutates its state
- An API for a database
- Interactions between a client(s) and a server

There is some initial investment needed to set the test up and it usually takes a bit more time to run than simple prop tests, but if correctness is important for your use case, you'll be rewarded with a test that is so effective at discovering bugs it might feel almost magical, but as you'll see, [you could have easily implemented it yourself](#how-does-it-work). Also, once you have the test setup, it is much easier to extend it and add new properties to check.

## How to use it

Before using state machine testing, it is recommended to be at least familiar with the basic concepts of Proptest itself as it's built on its essential foundations. That is:

- Strategies are composed from common proptest constructs and used to generate inputs to a state machine test.
- Because the generated transitions sequence is a strategy itself, a test will attempt to shrink them on a discovery of a case that breaks some properties.
- It will capture regressions file with a seed that can be used to deterministically repeat the found case.

In short, use `ReferenceStateMachine` and `StateMachineTest` to implement your state machine test and `prop_state_machine!` macro to run it.

If you just want to get started quickly, take a look at one of the examples:

- `state_machine_heap.rs` - a simple model to test an API of a heap data structure
- `state_machine_echo_server.rs` - a more advanced model for an echo server with multiple clients talking to it

To see what transitions are being applied in standard output as the state machine test executes, run these with e.g. `PROPTEST_VERBOSE=1 cargo run --example state_machine_heap`.

State machine testing is made up of two parts, an abstract reference state machine definition that drives the inputs to a test and a test definition for a SUT that replicates the same transitions as the reference state machine to find any possible divergence or conditions under which the defined properties (in here post-conditions and invariants) start to break.

### Reference state machine strategy

You can get started with state machine testing by implementing `trait ReferenceStateMachine`, which is used to drive the generation of a sequence of transitions and can also be compared against the state of the SUT. At the minimum, this trait requires two associated types:

- `type State` that represents the state of the reference state machine.
- `type Transition` with possible transitions of the state machine. This is typically an `enum` with its variants containing input parameters for the transitions, if any.

You also have to implement three associated functions:

- To initialize the reference state machine:

  ```rust,ignore
  fn init_state() -> BoxedStrategy<Self::State>
  ```

  You can generate some random state with a strategy or use `Just` strategy for a constant value. Note that you can make a `BoxedStrategy` from any `Strategy` by simply calling `.boxed()` on it.

- To generate transitions:

  ```rust,ignore
  fn transitions(state: &Self::State) -> BoxedStrategy<Self::Transition>
  ```

  Most of the time, you'll use `prop_oneof!` here. If a transition takes some input parameters, you can generate those with a `Strategy` and `.prop_map` it to the `Transition` variant. In more complex state machines, the set of valid transitions may depend on the current state. To that end, you can use the `state` argument, possibly combined with `proptest::sample::select` function that allows you to create a strategy that selects a random value from an array or an array-like collection (be careful not to call `select` on an empty array as that will make it fail in a somewhat obscure way). For example, if you want to remove one of the existing keys from a hash map, you can select one of the keys from the current state and map it into a transition. Note that when you do something like this, you'll also need to override the `fn preconditions`, which are explained in more detail below.

- To apply the given transition on the reference state:

  ```rust,ignore
  fn apply(mut state: Self::State, transition: &Self::Transition) -> Self::State
  ```

Additionally, you may want to override the default implementation of:

```rust,ignore
fn preconditions(state: &Self::State, transition: &Self::Transition) -> bool
```

By default, this simply returns `true`, which implies that there are no pre-conditions. Pre-conditions are a way of restricting what transitions are valid for a given state and you'll *only* need to restrict the transitions whose validity depends on the current state. This ensures that the reference state machine will only produce and shrink to a sequence of valid transitions. It may not be immediately apparent that the current state may be affected by shrinking. With the example of selecting of keys of a hash map for `fn transitions`, you'll need to check that the transition's key is still present in the hash map, which may no longer be true after some shrinking is applied.

You can either implement `ReferenceStateMachine` for:

- A data structure that will represent your reference state machine and set the associated `type State = Self;` or
- An empty `struct`, which may be more convenient than making a wrapper type if you're using a foreign type for the `type State`

### Definition of a state machine test

With that out of the way, you can go ahead and implement `StateMachineTest`. This also requires two associated types:

- `type SystemUnderTest` which is the type that represents the SUT.
- `type Reference` with the type for which you implemented the `ReferenceStateMachine`.

There are also three associated functions to be implemented here (some types are slightly simplified for clarity):

- Initialize the SUT state:

  ```rust,ignore
  fn init_test(ref_state: &Self::Reference::State) -> Self::SystemUnderTest
  ```

  If your `ReferenceStateMachine::init_state` uses a non-constant strategy, you have to use the `ref_state` to initialize this to a corresponding state to ensure that you have consistent initial states.

- Apply the `transition` on the SUT state:

  ```rust,ignore
  fn apply(
    mut state: Self::SystemUnderTest,
    ref_state: &Self::Reference::State,
    transition: Transition
  ) -> Self::SystemUnderTest
  ```

  This is also where you'll want to check any post-conditions that apply to a given transition, so after you apply the transition to the state, you can `assert!` some properties. Alternatively or additionally, you can use the `ref_state` for comparison, which will have the same transition that is given to this function already applied to it.

- Check properties that apply in any state:

  ```rust,ignore
  fn check_invariants(state: &Self::SystemUnderTest, ref_state: &Self::Reference::State)
  ```

  These must always hold and will be checked after every transition. Just like with `apply`, you have the option to use the `ref_state` for comparison.

To add some teardown logic to run at the end of each test case, you can override the `teardown` function, which by default simply drops the state:

```rust,ignore
fn teardown(state: Self::SystemUnderTest, ref_state: Self::Reference::State)
```

### Make the state machine test runnable

Finally, to run the `StateMachineTest`, you can use the `prop_state_machine!` macro. For example:

```rust,ignore
prop_state_machine! {
  #[test]
  fn name_of_the_test(sequential 1..20 => MyStateMachineTest);
}
```

You pick a `name_of_the_test` and a single numerical value or a range after the `sequential` keyword for a number of transitions to be generated for the state machine execution. The `MyStateMachineTest` is whatever you've implemented the `StateMachineTest` for.

And that's it. You can run the test, perhaps with `cargo watch` as you develop it further, and see if it can find some interesting counter-examples to your properties.

### Extra tips

Because a state machine test may be heavier than regular prop tests, if you're running your tests in a CI you may want to override the default `proptest_config`'s `cases` to include more or fewer cases in a single run. You can also use `PROPTEST_CASES` environment variable and during development it is preferable to override this to run many cases to get a better chance of catching those pesky ~~bugs~~, erm, defects.

> Given that there are thought to be in the region of another four million species that we have not yet even named, there is no doubt that scientists will be kept happily occupied studying them for millennia, so long as the insects remain to be studied. Would the world not be less rich, less surprising, less wonderful, if these peculiar creatures did not exist?
>
> -- <cite>Dave Goulson, Silent Earth</cite>

So let's leave bugs alone and only squash defects instead!

Because the output of a failed test case can be a bit hard to read, it is often convenient to print the transitions. You can do that by simply setting the `proptest_config`'s `verbose` to `1` or higher. Again, if you don't want to keep this in your test's config or if you'd prefer to override the config, you could also use the `PROPTEST_VERBOSE` environment variable instead.

Another helpful config option that is good to know about is `timeout` (`PROPTEST_TIMEOUT` via an env var) for tests that may take longer to execute.

## How does it work

This section goes into the inner workings of how the state machine is implemented, omitting some less interesting details. If you're only interested in using it, you can consider this section an optional read.

The `ReferenceStateMachine::sequential_strategy` sets up a `Sequential` strategy that generates a sequence of transitions from the definition of the `ReferenceStateMachine`. The acceptability of each transition in the sequence depends on the current state of the state machine and `ReferenceStateMachine::preconditions`, if any. The state is updated by the transitions with the `ReferenceStateMachine::apply` function.

The `Sequential` strategy is then fed into Proptest like any other strategy via the `prop_state_machine!` macro and it produces a `Vec<Transition>` that gets passed into `StateMachineTest::test_sequential` where it is applied one by one to the SUT. Its post-conditions and invariants are checked during this process and if a failing case is found, the shrinking process kicks in until it can shrink no longer.

The shrinking strategy which is defined by the associated `type Tree = SequentialValueTree` of the `Sequential` strategy is to iteratively apply `Shrink::InitialState`, `Shrink::DeleteTransition` and `Shrink::Transition` (this can be found in `proptest/src/strategy/state_machine.rs`):

1. We start by trying to delete transitions from the back of the list until we can do so no further (the list has reached the `min_size` - that is the variable that gets set from the chosen range for the number of transitions in the `prop_state_machine!` invocation).
2. Then, we again iteratively attempt to shrink the individual transitions, but this time starting from the front of the list from the first transition to be applied.
3. Finally, we try to shrink the initial state until it's not possible to shrink it any further.

The last applied shrink gets stored in the `SequentialValueTree`, so that if the shrinking process ends up in a case that no longer reproduces the discovered issue, the call to `complicate` in the `ValueTree` implementation of the `SequentialValueTree` can attempt to undo it.

## Similar technologies

The state machine testing support for Proptest is heavily inspired by the Erlang's eqc_statem (see the paper [Finding Race Conditions in Erlang with QuickCheck and PULSE](https://smallbone.se/papers/finding-race-conditions.pdf)) with some key differences. Most notably:

- Currently, only sequential strategy is supported, but a concurrent strategy is planned to be added at later point.
- There are no "symbolic" variables like in eqc_statem. The state for the abstract (reference) state machine is separate from the state of the system under test.
- The post-conditions are not defined in their own function. Instead, they are part of the `StateMachineTest::apply` function.

---

# Tips and Best Practices

## Performance

### Setting `opt-level`

Both the proptest crate and the random number generator it uses can be CPU intensive. If you are
generating a lot of cases you may see a significant performance improvement by setting the `opt-level`
to `3` in your `Cargo.toml` file:

```toml
[profile.test.package.proptest]
opt-level = 3

[profile.test.package.rand_chacha]
opt-level = 3
```

### Reusing mutable resources

Sometimes you may want to reuse mutable resources across individual cases. For example, you may want
to reuse a database connection or a file handle to avoid the overhead of opening and closing it for
each case. Because the `proptest!` macro (when used with closure-style invocation) requires a `Fn`, you need to wrap your state in a `RefCell`:

```rust
# extern crate proptest;
use std::cell::RefCell;
use proptest::proptest;

# struct ConnectionPool {};
# struct MyConnection {};
# impl ConnectionPool {
#    fn new() -> Self { Self {} }
#    fn connect(&mut self) -> MyConnection { MyConnection {} }
# }
#[test]
# fn dummy() {}; // This is here to make the doctest work
fn test_with_shared_connection() {
    let mut my_conn = RefCell::new(ConnectionPool::new().connect());
    proptest!(|(x in 0..42)| {
        let mut conn = my_conn.borrow_mut();
        // Use state
    });
}
```

---

# The `proptest-derive` crate

The `proptest-derive` crate provides a procedural macro,
`#[derive(Arbitrary)]`, which can be used to automatically generate simple
`Arbitrary` implementations for user-defined types, allowing them to be used
with `any()` and embedded in other `#[derive(Arbitrary)]` types without fuss.

It is recommended to have a basic working understanding of the [`proptest` crate](/proptest/index.md) before getting into this part of the
documentation.

**This crate is currently somewhat experimental.** Expect rough edges,
particularly in documentation. It is also more likely to see releases with
breaking changes than the main `proptest` crate.

---

# Getting started

## Cargo

To the `[dev-dependencies]` section of your `Cargo.toml`, add

```toml
proptest-derive = "0.8.0"
```

In a Rust 2015 crate, you must add

```
#[cfg(test)] extern crate proptest;
```

to the top of the crate.

### About Versioning

`proptest-derive` is currently experimental and has its own version. Once it is
more stable, it will be versioned in lock-step with the main `proptest` crate.

## Using derive

Inside any of your test modules, you can simply add `#[derive(Arbitrary)]` to a
struct or enum declaration.

```rust
#[cfg(test)]
mod test {
    use proptest::prelude::*;
    use proptest_derive::Arbitrary;

    #[derive(Arbitrary, Debug)]
    struct MyStruct {
        // ...
    }

    proptest! {
        #[test]
        fn test_one(my_struct: MyStruct) {
            // ...
        }

        // Equivalent to the above
        fn test_two(my_struct in any::<MyStruct>()) {
            // ...
        }
    }
}
```

In order to use `proptest-derive` on a type *not* in a test module without also
depending on proptest for your main build, you must currently manually gate off
the related annotations. This is something we plan to [improve in the future](https://github.com/proptest-rs/proptest/pull/106).

```rust
#[cfg(test)]
use proptest_derive::Arbitrary;

#[derive(Debug)]
// derive(Arbitrary) is only available in tests
#[cfg_attr(test, derive(Arbitrary))]
struct MyStruct {
    // Attributes consumed proptest-derive must not be added when the
    // declaration is not being processed by derive(Arbitrary).
    #[cfg_attr(test, proptest(value = 42))]
    answer: u32,
    // ...
}
```

---

# Modifier Reference

All modifiers interpreted by `#[derive(Arbitrary)]` are of the form
`#[proptest(..)]`, where the content between the parentheses follows the normal
Rust attribute syntax.

Each modifier within the parentheses is independent, in that putting two
modifiers in the same attribute is equivalent to having two `#[proptest(..)]`
attributes with one modifier each.

For brevity, modifiers are sometimes referenced by name alone; e.g., "the
`weight` modifier" refers to `#[proptest(weight = nn)]` and not some
freestanding `#[weight]` attribute.

## `filter`

Form: `#[proptest(filter = F)]` or `#[proptest(filter(F))]` where `F` is either
a bare identifier (i.e., naming a function) or a Rust expression in a string.
In either case, the parameter must evaluate to something which is `Fn (&T) ->
bool`, where `T` is the type of the item being filtered.

Usable on: structs, enums, enum variants, fields

The `filter` modifier allows filtering values generated for a field via
rejection sampling. Since rejection sampling is inefficient and interferes with
shrinking, it should only be used for conditions that are very rare or are
unfeasible to express otherwise. In many cases, [`strategy`](#strategy) can be
used to more directly express the desired behaviour without rejection sampling.
See the documentation for [`prop_filter`] for more details.

The argument to the modifier must be a valid argument for the second parameter
of [`prop_filter`].

Example:

```rust
# extern crate proptest_derive;
# extern crate proptest;
# use proptest_derive::Arbitrary;
# use proptest::prelude::*;

#[derive(Debug, Arbitrary)]
#[proptest(filter = "|segment| segment.start != segment.end")]
struct NonEmptySegment {
    start: i32,
    end: i32,
}
```

is equivalent to

```rust
# extern crate proptest_derive;
# extern crate proptest;
# use proptest_derive::Arbitrary;
# use proptest::prelude::*;

fn is_nonempty(segment: &NonEmptySegment) -> bool {
    segment.start != segment.end
}

#[derive(Debug, Arbitrary)]
#[proptest(filter = "is_nonempty")]
struct NonEmptySegment {
    start: i32,
    end: i32,
}
```

As mentioned above, filtering should be avoided when it is reasonably possible
to express a non-filtering strategy that achieves the same effect. For example:

```rust
# extern crate proptest_derive;
# extern crate proptest;
# use proptest_derive::Arbitrary;
# use proptest::{proptest, arbitrary::any, strategy::Strategy};

#[derive(Debug, Arbitrary)]
struct BadExample {
    // Don't do this! Your tests will run more slowly and shrinking won't work
    // properly.
    #[proptest(filter = "|x| x % 2 == 0")]
    even_number: u32,
}

#[derive(Debug, Arbitrary)]
struct GoodExample {
    // Directly generate even numbers only by transforming the set of all
    // `u32`s and then mapping it to the set of even `u32`s.
    #[proptest(strategy = "any::<u32>().prop_map(|x| x / 2 * 2)")]
    even_number: u32,
}
```

[`prop_filter`]: https://docs.rs/proptest/latest/proptest/strategy/trait.Strategy.html#method.prop_filter

## `no_bound`

Form: `#[proptest(no_bound)]`

Usable on: generic type definitions and type parameters

Normally, when `#[derive(Arbitrary)]` is applied to an item with generic type
parameter, every type parameter which is "used" (see below) is required to
`impl Arbitrary`. For example, given a declaration like the following:

```rust
# extern crate proptest_derive;
# use proptest_derive::Arbitrary;

#[derive(Debug, Arbitrary)]
struct MyStruct<T> {
    # t: T
    /* ... */
}
```

Something like this will be generated:

```rust
# extern crate proptest;
# use proptest::arbitrary::Arbitrary;

# #[derive(Debug)]
# struct MyStruct<T> {
# t: T
}

impl<T> Arbitrary for MyStruct<T> where T: Arbitrary {
    # type Parameters = u32;
    # type Strategy = proptest::strategy::BoxedStrategy<Self>;
    # fn arbitrary_with(_params: Self::Parameters) -> Self::Strategy { todo!() }
    /* ... */
}
```

Placing `#[proptest(no_bound)]` on a generic type definition is equivalent to
placing the same attribute on every type parameter.

```rust
# extern crate proptest_derive;
# extern crate proptest;
# use proptest_derive::Arbitrary;
# use proptest::proptest;
# use std::marker::PhantomData;

#[derive(Debug, Arbitrary)]
#[proptest(no_bound)]
struct MyStruct<A, B, C> {
    # a: PhantomData<A>,
    # b: PhantomData<B>,
    # c: PhantomData<C>,
    /* ... */
}
```

This is equivalent to a hypothetical (but not currently supported) syntax like:

```rust,compile_fail
# extern crate proptest_derive;
# use proptest_derive::Arbitrary;
# use std::marker::PhantomData;

#[derive(Debug, Arbitrary)]
struct MyStruct<
  #[proptest(no_bound)] A,
  #[proptest(no_bound)] B,
  #[proptest(no_bound)] C,
> {
    # a: PhantomData<A>,
    # b: PhantomData<B>,
    # c: PhantomData<C>,
    /* ... */
}
```

A type parameter is "used" if the following hold:

- The enum or struct definition references it at least once, and that reference
  is not inside the type argument of a `PhantomData`.

- The item referencing the type parameter does not have any proptest modifiers
  which replace the usual use of `Arbitrary`, such as [`skip`](#skip) or
  [`value`](#value).

Due to the above, `#[proptest(no_bound)]` is generally only needed when the
type parameter is used in another type which does not itself have an
`Arbitrary` bound on the type.

## `no_params`

Form: `#[proptest(no_params)]`

Usable on: structs, enums, enum variants, fields

On a struct or enum, `no_params` causes the `Arbitrary` parameter type to be
`()`. All automatic delegations to `Arbitrary` on members of the item use
`Default::default()` for their parameters.

On an enum variant or field, suppresses the addition of any parameter for the
variant or field to the parameters for the whole struct. If the variant or
field automatically delegates to `Arbitrary` for its value, that `Arbitrary`
call uses `Default::default()` for its own parameter.

See the [`param` modifier](#param) for more information on how parameters work.

## `params`

Form: `#[proptest(params = T)]` or `#[proptest(params(T))]`, where `T` is
either a bare identifier or Rust code inside a string. In either case, the
value must name a concrete Rust type which implements `Default`.

Usable on: structs, enums, enum variants, fields

The [`Arbitrary` trait] specifies a `Parameters` type which is used to control
generation. By default, the `Parameters` type is a tuple of the parameters
which are automatically passed to other `Arbitrary` implementations.

If applied to a struct or enum, `params` completely replaces the `Parameters`
type. Any automatic delegations to other `Arbitrary` implementations then use
`Default::default()` as there is no automatic way to locate an appropriate
value (if there even is any) within the `params` type.

If applied to an enum variant or field, `params` specifies the parameters type
for just that item, as if its type had an `Arbitrary` implementation taking
that type. In this case, either [`value`](#value) or [`strategy`](#strategy)
*must* be specified since the parameter type will not generally be compatible
with the normal `Arbitrary` invocation (and in cases where it is, `params`
would be useless if not used).

Any expressions (such as in the [`value`](#value) and [`strategy`](#strategy)
modifiers) underneath an item with the `params` modifier has access to a
variable named `params` which is of the type passed in
`#[proptest(params = ..)]`.

Examples:

```rust
# extern crate proptest_derive;
# extern crate proptest;
# use proptest_derive::Arbitrary;
# use proptest::prelude::*;

#[derive(Debug)]
struct WidgetRange(usize, usize);

impl Default for WidgetRange {
    fn default() -> Self { Self(0, 100) }
}

#[derive(Debug, Arbitrary)]
#[proptest(params(WidgetRange))]
struct WidgetCollection {
    #[proptest(strategy = "params.0 ..= params.1")]
    desired_widget_count: usize,
    // ...
}

// ...

proptest! {
    #[test]
    fn test_something(wc in any_with::<WidgetCollection>(WidgetRange(10, 20))) {
        assert!(wc.desired_widget_count >= 10 && wc.desired_widget_count <= 20);
    }
}
```

[`Arbitrary` trait]: https://docs.rs/proptest/latest/proptest/arbitrary/trait.Arbitrary.html

## `regex`

Form: `#[proptest(regex = "string")]` or `#[proptest(regex("string"))]`, where
`string` is a regular expression. May also be invoked as
`#[proptest(regex(function_name))]`, where `function_name` is a no-argument
function that returns an `&'static str`.

Usable on: fields

This modifier specifies to generate character or byte strings for a field which
match a particular regular expression.

The `regex` modifier is equivalent to using the [`strategy`](#strategy) modifier and
enclosing the string in [`string_regex`] or [`bytes_regex`]. It can only be
applied to fields of type `String` or `Vec<u8>`.

Example:

```rust
# extern crate proptest_derive;
# extern crate proptest;
# use proptest_derive::Arbitrary;
# use proptest::proptest;
#[derive(Debug, Arbitrary)]
struct FileContent {
    #[proptest(regex = "[a-z0-9.]+")]
    name: String,
    #[proptest(regex = "([0-9]+\n)*")]
    content: Vec<u8>,
}
```

[`string_regex`]: https://docs.rs/proptest/latest/proptest/string/fn.string_regex.html
[`bytes_regex`]: https://docs.rs/proptest/latest/proptest/string/fn.bytes_regex.html

## `skip`

Form: `#[proptest(skip)]`

Usable on: enum variants

Annotating an enum variant with `#[proptest(skip)]` prevents proptest from
generating that particular variant. This is useful when there is no sensible
way to generate the variant or when you want to temporarily stop generating
some variant during development.

Example:

```rust
# extern crate proptest_derive;
# extern crate proptest;
# use proptest_derive::Arbitrary;
# use proptest::prelude::*;

#[derive(Debug, Arbitrary)]
enum DataSource {
    Memory(Vec<u8>),

    // There's no way to produce an "arbitrary" file handle, so we skip
    // generating this case.
    #[proptest(skip)]
    File(std::fs::File),
}
```

It is an error to annotate all inhabited variants of an enum with
`#[proptest(skip)]` as this leaves proptest with no options to generate the
enum.

## `strategy`

Form: `#[proptest(strategy = S)]` or `#[proptest(strategy = S)]`, where `S` is
either a string containing a Rust expression which evaluates to an appropriate
`Strategy`, or a bare identifier naming a function which, when called with no
arguments, returns such a `Strategy`.

Usable on: enum variants, fields

By default, enum variants are generated by recursing into their definition as
is done for struct declarations, and fields are generated by invoking
`Arbitrary` on the field type to produce a `Strategy`. The `strategy` modifier
allows to manually provide a custom strategy directly.

In the case of fields, the strategy must produce values of the same type as
that field. For enum variants, it must produce values of the enum type itself
and these values ought to be of the variant in question.

Example:

```rust
# extern crate proptest_derive;
# extern crate proptest;
# use proptest_derive::Arbitrary;
# use proptest::prelude::*;
# use proptest::strategy::Strategy;

#[derive(Debug, Arbitrary)]
enum Token {
    Delimitation {
        // This field is still generated via Arbitrary
        delimiter: Delimiter,

        // But for this field we use a custom strategy
        #[proptest(strategy = "1..(10 as u32)")]
        count: u32,

        // Here we also use a custom strategy, generated by the function
        // `offset_strategy`.
        #[proptest(strategy = "offset_strategy()")]
        offset: u32,
    },

    // Specify how to generate the whole enum variant
    #[proptest(strategy = "\"[a-zA-Z]+\".prop_map(Token::Word)")]
    Word(String),
}

#[derive(Debug, Arbitrary)]
enum Delimiter {
    # Nope
    /* ... */
 }

fn offset_strategy() -> impl Strategy<Value = u32> {
  0..(100 as u32)
}
```

## `value`

Form: `#[proptest(value = V)]` or `#[proptest(value(V))]`, where V can be: (a)
a Rust expression enclosed in a string; (b) another literal, or (c) a bare
identifier naming a no-argument function.

Usable on: enum variants, fields

The `value` modifier indicates that proptest should use the given expression or
function to produce a value for the field, instead of going through the usual
value generation machinery.

The argument to `value` is directly used as an expression for the field value
or enum variant to be generated, except that in the third form where it is a
bare identifier, it is called as a no-argument function to produce the value.

Using `value` is equivalent to using [`strategy`](#strategy) and enclosing the
value in `LazyJust`.

Example:

```rust
# extern crate proptest_derive;
# extern crate proptest;
# use proptest_derive::Arbitrary;
# use proptest::prelude::*;
# use std::time::Instant;

#[derive(Debug, Arbitrary)]
struct EventCounter {
    // We always start with the first two fields set to 0/None
    #[proptest(value = 0)]
    number_seen: u64,

    #[proptest(value = "None")]
    last_seen_time: Option<Instant>,

    // This field is generated normally
    max_events: u64,
}
```

## `weight`

Form: `#[proptest(weight = W)]` or `#[proptest(weight(W))]`, where `W` is an
expression evaluating to a `u32`. `weight` may also be abbreviated to `w`, as
in `#[proptest(w = W)]`.

Usable on: enum variants

The `weight` modifier determines how likely proptest is to generate a
particular enum variant. Weights are relative to each other; for example, a
`weight = 3` variant is 50% more likely to be generated than a `weight = 2`
variant and three times as likely to be generated as a `weight = 1` variant.

Variants with no `weight` modifier are equivalent to being annotated
`#[proptest(weight = 1)]`.

Example:

```rust
# extern crate proptest_derive;
# extern crate proptest;
# use proptest_derive::Arbitrary;
# use proptest::proptest;
#[derive(Debug, Arbitrary)]
enum FilterOption {
    KeepAll,
    DiscardAll,

    // This option is presumably harder for the code to handle correctly,
    // so we generate it more frequently than the other options.
    #[proptest(weight = 3)]
    OnlyMatching(String),
}
```

---

# Error Index

[issue tracker]: https://github.com/proptest-rs/proptest

## E0001

[lifetime parameters]: https://doc.rust-lang.org/stable/book/second-edition/ch10-03-lifetime-syntax.html#lifetime-annotations-in-struct-definitions

This error occurs when `#[derive(Arbitrary)]` is used on a type which has any
[lifetime parameters]. For example:

```rust,compile_fail
#[derive(Debug, Arbitrary)]
struct Foo<'a> {
    bar: &'a str,
}
```

[issue#9]: https://github.com/proptest-rs/proptest/issues/9

It is not yet possible to define a `Strategy` which generates a type that is
lifetime-generic (e.g. `&'a T`). Thus, proptest cannot implement `Arbitrary` for
such types either and therefore you cannot `#[derive(Arbitrary)]` for such types.
GATs are available in stable rust as of 1.65 and we will be revisiting how to support
this. To follow the progress, consult the [tracking issue][issue#9] on the matter.

## E0002

This error occurs when `#[derive(Arbitrary)]` is used on a `union` type.
An example:

```rust,compile_fail
#[derive(Debug, Arbitrary)]
union IU32 {
    signed: i32,
    unsigned: u32,
}
```

There are two main reasons for the error.

1. It is not possible to `#[derive(Debug)]` on `union` types and manual
   implementations cannot know which variant is valid so there are not
   many valid implementations which are possible.

2. Second, we cannot mechanically tell which variant out of `signed` and
   `unsigned` to generate. While we could allow you to tell the macro,
   with an attribute such as `#[proptest(select)]` on the variant,
   we have opted for a more conservative approach for the time being.
   If you have a use case for `#[derive(Arbitrary)]` on `union` types,
   please reach out on the [issue tracker].

## E0003

This error occurs when `#[derive(Arbitrary)]` is used on a struct which
contains known [uninhabited types](https://doc.rust-lang.org/nomicon/exotic-sizes.html#empty-types). This
in turn means the struct itself is uninhabited and so it there is no sensible
`Arbitrary` implementation since values of the struct cannot be produced.

A trivial example:

```rust,compile_fail
#[derive(Debug, Arbitrary)]
struct Uninhabited {
    inhabited: u32,
    never: !,
}
```

Because there exist no values assignable to field `never`, it is also
impossible to construct an instance of struct `Uninhabited`.

Proptest's ability to identify uninhabited types is limited. If it does not
recognise a particular type as uninhabited, the type will instead be assumed to
be inhabited and you will instead get an error about the type not implementing
`Arbitrary` trait.

## E0004

This error occurs when `#[derive(Arbitrary)]` is used on an enum with no
variants at all. For example:

```rust,compile_fail
#[derive(Debug, Arbitrary)]
enum Uninhabited {}
```

Such an enum has no values at all, so it does not make sense to provide an
`Arbitrary` implementation for it since no values can be generated.

## E0005

This error occurs if `#[derive(Arbitrary)]` is used on an enum whose variants
are all uninhabited, using the same logic as described for [`E0003`](#e0003).
As a result, the enum itself is totally uninhabited.

Example:

```rust,compile_fail
#[derive(Debug, Arbitrary)]
enum Uninhabited {
    Never(!),
    NeverEver(!, !),
}
```

## E0006

This error occurs if `#[derive(Arbitrary)]` is used on an enum where all
inhabited variants are marked with [`#[proptest(skip)]`]. In other words,
proptest is forbidden from generating any of the enum's variants, and thus the
enum itself cannot be generated.

Example:

```rust,compile_fail
#[derive(Debug, Arbitrary)]
enum MyEnum {
    // Ordinarily, proptest would be able to generate either of these variants,
    // but both are forbidden, so in the end proptest isn't allowed to generate
    // anything at all.
    #[proptest(skip)]
    UnitVariant,
    #[proptest(skip)]
    SimpleVariant(u32),
    // This variant is implicitly skipped because proptest knows it is
    // uninhabited.
    Uninhabited(!),
}
```

## E0007

This error happens if an attribute [`#[proptest(strategy = "expr")]`] or
[`#[proptest(value = "expr")]`] is applied to the same item that has
`#[derive(Arbitrary)]`.

Example:

```rust,compile_fail
#[derive(Debug, Arbitrary)]
#[proptest(value = "MyStruct(42)")]
struct MyStruct(u32);
```

This is rejected since nothing is being "derived" *per se*. A written out
implementation of `Arbitrary` should be used instead.

## E0008

This error happens if [`#[proptest(skip)]`] is applied to an unskippable item.
For example, struct fields cannot be skipped because Rust requires every field
of a struct to have a value.

Example:

```rust,compile_fail
#[derive(Debug, Arbitrary)]
struct WidgetContainer {
    desired_widget_count: usize,
    #[proptest(skip)]
    widgets: Vec<Widget>,
}
```

In general, the appropriate way to request proptest to not generate a field
value is to use [`#[proptest(value = "expr")]`] to provide a fixed value
yourself. For example, the above code could be properly written as follows:

```rust,compile_fail
#[derive(Debug, Arbitrary)]
struct WidgetContainer {
    desired_widget_count: usize,
    #[proptest(value = "vec![]")] // Always generate an empty widget vec
    widgets: Vec<Widget>,
}
```

## E0009

This error happens if [`#[proptest(weight = <integer>)]`] is applied to an item
where this does not make sense, such as a struct field. For example:

```rust,compile_fail
#[derive(Debug, Arbitrary)]
struct Point {
    x: u32,
    #[proptest(weight = 42)]
    y: u32,
}
```

The `weight` attribute only is sensible where proptest has a choice between
multiple items, i.e., enum variants. In contrast, with struct fields proptest
must provide a value for *every* field so there is no "this-or-that" choice.

## E0010

This error occurs if [`#[proptest(params = "type")]`] and/or
[`#[proptest(no_params)]`] are set on both an item and its parent.

Example:

```rust,compile_fail
#[derive(Debug, Arbitrary)]
#[proptest(params = "String")]
struct Foo {
    #[proptest(no_params)]
    bar: String,
}
```

If the parent item has any explicit parameter configuration, it totally defines
the parameters for the whole `Arbitrary` implementation and the child items
must work with that and cannot specify their own parameters.

## E0011

This error occurs if [`#[proptest(params = "type")]`] is set on a field but no
explicit strategy is configured with [`#[proptest(strategy = "expr")]`] or
another such modifier. For example:

```rust,compile_fail
#[derive(Debug, Arbitrary)]
struct Foo {
    #[proptest(param = "u8")]
    some_string: String,
}
```

This example illustrates why both must be specified: `String`'s arbitrary
implementation takes a `proptest::string::StringParam`, but here we try to pass
it a `u8`.

While the generated code could work if the type given by `param` is the same as
that for the default strategy, there would be no purpose in specifying the
parameter type by hand; therefore specifying only `param` is in all cases
forbidden.

## E0012

This error occurs if [`#[proptest(filter = "expr")]`] is set on an item, but the
item containing it specifies a direct way to generate the whole value, which
would thus occur without consulting the filter.

Example:

```rust,compile_fail
#[derive(Debug, Arbitrary)]
enum Foo {
    #[proptest(value = "Foo::Bar(42)")]
    Bar {
        #[proptest(filter = "is_even")]
        even_number: u32,
    },
    // ...
}
```

In this example, the entire `Bar` variant specifies how to generate itself
wholesale. As a result, the `filter` clause on `even_number` has no opportunity
to run.

## E0013

This error would occur if an outer attribute of the form `#![proptest(..)]`
were applied to something underneath a `#[derive(Arbitrary)]`.

As of Rust 1.30.0, there are no known ways to produce this error since the Rust
compiler will reject the attribute first.

## E0014

This error occurs if a bare `#[proptest]` attribute is applied to anything,
since it has no meaningful content.

Example:

```rust,compile_fail
#[derive(Debug, Arbitrary)]
struct Foo {
    #[proptest]
    field: u8,
}
```

The only legal use of the attribute is the form `#[proptest(..)]`.

## E0015

This error occurs if an attribute of the form `#[proptest = value]` is
encountered in any context.

Example:

```rust,compile_fail
#[derive(Debug, Arbitrary)]
struct Foo {
    #[proptest = 1234]
    field: u8,
}
```

## E0016

This error occurs if a literal (as opposed to `key = value`) is passed inside
`#[proptest(..)]` in any context.

Example:

```rust,compile_fail
#[derive(Debug, Arbitrary)]
struct Foo {
    #[proptest(1234)]
    field: u8,
}
```

## E0017

This error occurs if any modifier of `#[proptest(..)]` is set more than once on
the same item.

Example:

```rust,compile_fail
#[derive(Debug, Arbitrary)]
#[proptest(no_params, no_params)]
struct Foo(u32);
```

## E0018

This error occurs if an unknown modifier is passed in `#[proptest(..)]`.

Example:

```rust,compile_fail
#[derive(Debug, Arbitrary)]
#[proptest(frobnicate = "true")]
struct Foo(u32);
```

Please see the [modifiers reference](modifiers.md) to see what modifiers are
available.

## E0019

This error happens if anything extra is passed to [`#[proptest(no_params)]`].

Example:

```rust,compile_fail
#[derive(Debug, Arbitrary)]
#[proptest(no_params = "true")]
struct Foo(u32);
```

`no_params` takes no configuration. The correct form is simply
`#[proptest(no_params)]`.

## E0020

This error happens if anything extra is passed to [`#[proptest(skip)]`].

Example:

```rust,compile_fail
#[derive(Debug, Arbitrary)]
enum Foo {
    Small,
    #[proptest(skip = "yes")]
    Huge(ExpensiveType),
}
```

`skip` takes no configuration. The correct form is simply `#[proptest(skip)]`.

## E0021

This error happens if [`#[proptest(weight = <integer>)]`] is passed an invalid
integer or passed nothing at all.

Example:

```rust,compile_fail
#[derive(Debug, Arbitrary)]
enum Foo {
    #[proptest(weight)]
    V1,
    #[proptest(weight = heavy)]
    V2,
}
```

The only acceptable form is `#[proptest(weight = <integer>)]`, where
`<integer>` is either an integer literal which fits in a `u32` or the same but
enclosed in quotation marks.

## E0022

This error occurs if more than one of [`#[proptest(no_params)]`] and
[`#[proptest(params = "type")]`] are applied to the same item.

Example:

```rust,compile_fail
#[derive(Debug, Arbitrary)]
#[proptest(no_params, params = "u8")]
struct Foo(u32);
```

One attribute or the other must be picked depending on desired effect.

## E0023

This error happens if an invalid [`#[proptest(params = "type")]`] attribute is
applied to an item.

Example:

```rust,compile_fail
#[derive(Debug, Arbitrary)]
#[proptest(params = "Vec<u8")] // Note missing '>'
struct Foo(u32);
```

There are a few different ways to get this error:

- Pass nothing at all. E.g., `#[proptest(params)]`.

- Pass something other than a string as the value. E.g.,
  `#[proptest(params = 42)]`.

- Pass a malformed type in the string, as in the example above. (See also
  [caveat on syntax](#valid-rust-syntax).)

## E0024

This error happens if an invalid `#[proptest ..]` attribute is applied using a
syntax the `proptest-derive` crate is not prepared to handle.

Exactly what conditions can produce this error vary by Rust version.

## E0025

This error happens if more than one of [`#[proptest(strategy = "expr")]`],
[`#[proptest(value = "expr")]`], or [`#[proptest(regex = "string")]`] are applied
to the same item.

Example:

```rust,compile_fail
#[derive(Debug, Arbitrary)]
struct Foo {
    #[proptest(value = "42", strategy = "Just(56)")]
    bar: u32,
}
```

Each of these modifiers completely describe how to generate the value, so they
cannot both be applied to the same thing. One or the other must be chosen
depending on the desired effect.

## E0026

This error happens if an invalid form of [`#[proptest(strategy = "expr")]`] or
[`#[proptest(value = "expr")]`] is used.

Example:

```rust,compile_fail
#[derive(Debug, Arbitrary)]
struct Foo {
    #[proptest(value = "3↑↑↑↑3")] // String content is not valid Rust syntax
    g1: u128,
}
```

There are a few different ways to get this error:

- Pass nothing at all. E.g., `#[proptest(value)]`.

- Use another illegal form. E.g., `#[proptest(value("a", "b"))]`.

- Pass a string expression which is not valid Rust syntax, as in the above
  example. (See also [caveat on syntax](#valid-rust-syntax).)

## E0027

This error happens if an invalid form of [`#[proptest(filter = "expr")]`] is
used.

Example:

```rust,compile_fail
#[derive(Debug, Arbitrary)]
struct Foo {
    #[proptest(filter = "> 3")] // String content is not an expression
    big_number: u128,
}
```

There are a few different ways to get this error:

- Pass nothing at all. E.g., `#[proptest(filter)]`.

- Use another illegal form. E.g., `#[proptest(filter("a", "b"))]`.

- Pass a string expression which is not valid Rust syntax, as in the above
  example. (See also [caveat on syntax](#valid-rust-syntax).)

## E0028

This error occurs if a modifier which implies a value is to be generated is
applied to an enum variant which is also marked [`#[proptest(skip)]`].

Example:

```rust,compile_fail
#[derive(Debug, Arbitrary)]
enum Enum {
    V1(u32),
    #[proptest(skip, value = "Enum::V2(42)")]
    V2(u32),
}
```

Here, the [`#[proptest(value = "expr")]`] modifier suggests the user intends
some value to be generated for the enum variant, but at the same time
[`#[proptest(skip)]`] indicates not to generate that variant.

## E0029

This error happens if a modifier which would constrain or control how the value
of an enum variant is to be generated is applied to a unit variant.

Example:

```rust,compile_fail
#[derive(Debug, Arbitrary)]
enum Foo {
    #[proptest(value = "Foo::V1")]
    UnitVariant,
    // ...
}
```

Unit variants only have one possible value, so there is only one possible
strategy. As a result, it is pointless to try to specify an alternate strategy
or to filter such variants.

## E0030

This error happens if a modifier which would constrain or control how the value
of a struct is to be generated is applied to a unit struct.

Example:

```rust,compile_fail
#[derive(Debug, Arbitrary)]
#[proptest(params = "u8")]
struct UnitStruct;
```

Unit structs only have one possible value, so there is only one possible
strategy. As a result, it is pointless to try to specify an alternate strategy
or to filter such structs.

## E0031

This error occurs if [`#[proptest(no_bound)]`] is applied to something that is
not a type variable.

Example:

```rust,compile_fail
#[derive(Debug, Arbitrary)]
struct Foo {
    #[proptest(no_bound)]
    bar: u32,
}
```

The `no_bound` modifier only makes sense on generic type variables, as in

```rust,compile_fail
#[derive(Debug, Arbitrary)]
struct Foo<#[proptest(no_bound)] T> {
    #[proptest(value = "None")]
    bar: Option<T>,
}
```

## E0032

This error happens if [`#[proptest(no_bound)]`] is passed anything.

Example:

```rust,compile_fail
#[derive(Debug, Arbitrary)]
struct Foo<#[proptest(no_bound = "yes")] T> {
    _bar: PhantomData<T>,
}
```

The only valid form for the modifier is `#[proptest(no_bound)]`.

## E0033

This error occurs if the sum of the weights on the variants of an enum overflow
a `u32`.

Example:

```rust,compile_fail
#[derive(Debug, Arbitrary)]
enum Foo {
    #[proptest(weight = 3_000_000_000)]
    ThreeFifths,
    #[proptest(weight = 2_000_000_000)]
    TwoFifths,
}
```

The only solution is to reduce the magnitude of the weights so that their sum
fits in a `u32`. Keep in mind that variants without a `weight` modifier still
effectively have `#[proptest(weight = 1)]`.

## E0034

This error occurs if [`#[proptest(regex = "string")]`] is used with invalid
syntax.

The most common forms are `#[proptest(regex = "string-regex")]` and
`#[proptest(regex("string-regex"))]`.

## E0035

This error occurs if both [`#[proptest(regex = "string")]`] and
[`#[proptest(params = "type")]`] are applied to the same item.

Values generated via regular expression take no parameters so the `params`
modifier would be meaningless.

## "Valid Rust syntax"

The definition of "valid Rust syntax" in various string modifiers is determined
by the `syn` crate. If valid syntax is rejected, you can work around it in a
couple ways depending on what the syntax is describing:

For types, simply define a type alias for the type in question. For example,

```rust,compile_fail
type RetroBox = ~str; // N.B. "~str" is not valid Rust 1.30 syntax

//...
#[derive(Debug, Arbitrary)]
#[proptest(params = "RetroBox")]
struct MyStruct { /* ... */ }
```

For values, you can generally factor the code into a constant or function. For
example,

```rust,compile_fail
// N.B. Rust 1.30 does not have an exponentiation operator.
const PI_SQUARED: f64 = PI * *2.0;

//...
#[derive(Debug, Arbitrary)]
struct MyStruct {
    #[proptest(value = "PI_SQUARED")]
    factor: f64,
}
```

If you need to implement such a work around, consider also [filing an issue](https://github.com/proptest-rs/proptest/issues).

[`#[proptest(filter = "expr")]`]: modifiers.md#filter
[`#[proptest(no_bound)]`]: modifiers.md#no_bound
[`#[proptest(no_params)]`]: modifiers.md#no_params
[`#[proptest(params = "type")]`]: modifiers.md#params
[`#[proptest(regex = "string")]`]: modifiers.md#regex
[`#[proptest(skip)]`]: modifiers.md#skip
[`#[proptest(strategy = "expr")]`]: modifiers.md#strategy
[`#[proptest(value = "expr")]`]: modifiers.md#value
[`#[proptest(weight = <integer>)]`]: modifiers.md#weight

---
