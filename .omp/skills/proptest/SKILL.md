---
name: proptest
description: Property-based testing in Rust with the proptest crate. Use when writing fuzz-like tests that generate arbitrary inputs, compose strategies, derive Arbitrary, configure shrinking, or debug test failures via persistence/replay.
---

# Proptest Skill

Property-based testing framework for Rust. Generates random inputs, shrinks failures to minimal cases, and persists regressions.

## Cargo Setup

```toml
[dev-dependencies]
proptest = "1"
proptest-derive = "0.5" # #[derive(Arbitrary)]

# Speed up strategy generation in test builds:
[profile.test.package.proptest]
opt-level = 3
```

## `proptest!` Macro

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn my_property(x in 0..100i32, y in "[a-z]+") {
        // x: i32 in [0,100), y: String matching regex
        prop_assert!(x >= 0);
        prop_assert_eq!(y.len() > 0, true);
    }
}
```

Override config per-block:

```rust
proptest! {
    #![proptest_config(ProptestConfig::with_cases(10_000))]
    #[test]
    fn thorough(x in any::<u64>()) { /* ... */ }
}
```

## Strategy Quick Reference

| Expression                               | Generates                                |
| ---------------------------------------- | ---------------------------------------- |
| `0..100i32`                              | `i32` in `[0, 100)`                      |
| `"[a-z]{1,8}"`                           | `String` matching regex                  |
| `any::<T>()`                             | Full range of `T` (requires `Arbitrary`) |
| `Just(value)`                            | Always `value` (no shrinking)            |
| `prop_oneof![s1, s2]`                    | Uniformly picks one strategy             |
| `prop::bool::weighted(0.7)`              | `true` 70% of the time                   |
| `prop::collection::vec(elem, size)`      | `Vec<T>` with `size` range               |
| `prop::collection::hash_map(k, v, size)` | `HashMap<K,V>` with `size` range         |
| `prop::option::of(strat)`                | `Option<T>` (Some or None)               |

## Combinators

```rust
// Transform generated values
(0..100i32).prop_map(|x| x * 2)

// Chain: first strategy's output parameterises second
(1..10usize).prop_flat_map(|len| prop::collection::vec(any::<u8>(), len..=len))

// Reject values (prefer narrowing the strategy instead)
(0..100i32).prop_filter("positive", |x| *x > 0)

// Recursive data (required for recursive types)
let tree = prop::bool::ANY.prop_recursive(
    4,    // max depth
    64,   // max nodes
    1,    // items per recursive step
    |inner| prop::collection::vec(inner, 0..4).prop_map(Node::Branch),
);

// Use .boxed() when strategy type is unnameable
fn my_strat() -> BoxedStrategy<MyType> {
    any::<u32>().prop_map(MyType::new).boxed()
}
```

## `prop_compose!`

Define reusable parameterised strategies:

```rust
prop_compose! {
    fn bounded_vec(max_len: usize)
                  (vec in prop::collection::vec(any::<i32>(), 0..max_len))
                  -> Vec<i32> {
        vec
    }
}

// Two-level: first draw feeds second draw's constraints
prop_compose! {
    fn vec_and_index()
        (vec in prop::collection::vec(any::<i32>(), 1..100))
        (index in 0..vec.len(), vec in Just(vec))
        -> (Vec<i32>, usize) {
        (vec, index)
    }
}
```

## `#[derive(Arbitrary)]`

```rust
use proptest_derive::Arbitrary;

#[derive(Debug, Arbitrary)]
struct Config {
    #[proptest(strategy = "1..=100u32")]
    retries: u32,
    #[proptest(regex = "[a-z]{1,10}")]
    name: String,
    #[proptest(skip)]
    _phantom: (),
}
```

| Modifier                | Target           | Effect                         |
| ----------------------- | ---------------- | ------------------------------ |
| `strategy = "expr"`     | field/variant    | Use `expr` as the strategy     |
| `filter = "\|x\| pred"` | field/variant    | Reject if predicate false      |
| `regex = "pattern"`     | `String` field   | Generate from regex            |
| `skip`                  | field            | Use `Default::default()`       |
| `weight = N`            | enum variant     | Relative selection weight      |
| `value = "expr"`        | variant          | Always produce `expr`          |
| `no_bound`              | field type param | Omit `Arbitrary` bound         |
| `params = "Type"`       | struct/enum      | Custom `Arbitrary::Parameters` |

## Assertions

| Macro                   | Use                                                       |
| ----------------------- | --------------------------------------------------------- |
| `prop_assert!(cond)`    | Assert condition; triggers shrinking on failure           |
| `prop_assert_eq!(a, b)` | Equality with shrinking                                   |
| `prop_assert_ne!(a, b)` | Inequality with shrinking                                 |
| `prop_assume!(cond)`    | Skip this case (use sparingly; prefer strategy narrowing) |

## Configuration

Key `ProptestConfig` fields:

| Field              | Default       | Purpose                                                  |
| ------------------ | ------------- | -------------------------------------------------------- |
| `cases`            | 256           | Test cases per `proptest!` block                         |
| `max_shrink_iters` | 0 (unlimited) | Cap shrinking iterations                                 |
| `fork`             | false         | Run each case in a subprocess (catches aborts/segfaults) |
| `timeout`          | 0 (none)      | Per-case timeout in ms (requires `fork`)                 |
| `verbose`          | 0             | Verbosity level (0=quiet, 1=failures, 2=all cases)       |

Environment variable overrides (useful in CI):

| Env Var                     | Overrides          |
| --------------------------- | ------------------ |
| `PROPTEST_CASES`            | `cases`            |
| `PROPTEST_MAX_SHRINK_ITERS` | `max_shrink_iters` |
| `PROPTEST_FORK`             | `fork`             |
| `PROPTEST_TIMEOUT`          | `timeout`          |
| `PROPTEST_VERBOSE`          | `verbose`          |

## Failure Persistence

On failure, proptest writes a regression seed to `proptest-regressions/<test_module>/<test_name>.txt`. These files replay the exact failing input on future runs.

- **Commit `proptest-regressions/` to VCS** so CI replays known failures.
- Delete a seed file to stop replaying that case.
- Set `PROPTEST_MAX_SHRINK_ITERS=0` to skip shrinking during debugging.

## State Machine Testing

For stateful protocol/API testing, use `proptest-state-machine`:

```toml
[dev-dependencies]
proptest-state-machine = "0.3"
```

Implement `ReferenceStateMachine` (abstract model) + `StateMachineTest` (real system under test), then invoke via `prop_state_machine!`. See `references/proptest_book.md` § State Machine Testing.

## Pitfalls

- **`prop_filter` vs narrower strategy**: Filters discard; too many discards = test failure. Prefer constructing valid values directly.
- **Recursive types need `prop_recursive`**: A plain recursive strategy causes infinite generation. Always bound depth.
- **Unnameable strategy types**: Use `.boxed()` to erase to `BoxedStrategy<T>` when returning from functions.
- **Shared mutable state**: Use `RefCell` inside `prop_map` closures; proptest may call strategies multiple times during shrinking.
- **WASM/`no_std`**: Proptest supports `no_std` via `default-features = false`, but some features (forking, persistence) are unavailable.

## Reference

Full proptest book: [`references/proptest_book.md`](references/proptest_book.md)
