# learn_01_core — Rust Core Basics

The foundation: how to declare data, control flow, and structure a program.
Everything here maps closely to Java/TS, with a few Rust twists called out in
each file's comments.

## Concepts in order

Run any concept with `cargo run --bin <name>`.

| # | Binary | Concept | Familiar analogy |
| --- | --- | --- | --- |
| 01 | `learn_01_01_hello` | `main`, `println!` macro | `System.out.println` / `console.log` |
| 02 | `learn_01_02_variables` | `let` vs `let mut`, formatting | `const` vs `let` |
| 03 | `learn_01_03_data_types` | integers, floats, bool, char, tuple, array | Java primitives + tuples |
| 04 | `learn_01_04_constants` | `const`, compile-time values | `static final` |
| 05 | `learn_01_05_shadowing` | re-`let` a name, change type | block-scoped `let` |
| 06 | `learn_01_06_debug_format` | `{}` Display vs `{:?}` Debug | `toString()` vs `JSON.stringify` |
| 07 | `learn_01_07_strings` | `String` vs `&str`, stdin | `StringBuilder` vs read-only view |
| 08 | `learn_01_08_functions` | fn signatures, expression return | typed functions |
| 09 | `learn_01_09_if_expression` | `if` returns a value | ternary for whole blocks |
| 10 | `learn_01_10_match` | exhaustive pattern matching | supercharged `switch` |
| 11 | `learn_01_11_loops` | `loop` / `while` / `for` | loops (no C-style for) |
| 12 | `learn_01_12_guess_number` | **mini-project** combining all above | — |

## Why this order?

The original set had the guessing game and error handling appear *before*
functions, `if`, `match`, and loops. Those examples actually USE those concepts,
so they were reordered: concepts first, then the `guess_number` capstone last.
(Standalone error handling now lives in its own `learn_06_error_handling`.)

```mermaid
graph LR
    hello --> variables --> data_types --> constants --> shadowing
    shadowing --> debug_format --> strings --> functions
    functions --> if_expression --> match --> loops --> guess_number
```

## Rust twists to remember

- **Immutable by default.** `let x = 5` cannot change; you need `let mut`.
- **Expression-based.** `if`, `match`, and blocks `{ }` evaluate to a value.
  The last line without a `;` is that value.
- **Exhaustive `match`.** The compiler forces you to handle every case.
- **No implicit numeric conversion.** Cast explicitly with `as`.
- **`String` vs `&str`.** Owned/growable vs borrowed/read-only view.
