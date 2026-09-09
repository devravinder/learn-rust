# Rust Roadmap

A learning order tuned for developers coming from **JavaScript / TypeScript /
Java**. Concepts build on each other top-to-bottom. Each concept folder is a
Cargo package with runnable examples under `src/bin/`.

- Reference: [Rust By Example](https://doc.rust-lang.org/stable/rust-by-example/index.html)
- Setup: [`setup.md`](setup.md)

> Folder links are filled in as each concept folder is reordered/renamed to the
> `concept_NN_*` convention. A checked box means the folder has been migrated.

## Learning order

### 01 — Core  ·  [`concept_01_core`](concept_01_core/)
- hello world
- variables & constants
  - shadowing
- data types
- functions
- control statements
  - if
  - loops
  - match
- printing
  - debug format
- arrays
- strings & slices
  - String vs str

### 02 — Ownership  ·  [`concept_02_ownership`](concept_02_ownership/)
- move
- borrow
- slices
- stack vs heap

### 03 — Modules & Crates  ·  [`concept_03_modules_crates`](concept_03_modules_crates/)
> Placed early: JS/Java devs expect a module system right after functions.
- modules
- file modules
- crates & the workspace layout

### 04 — Structs & Enums  ·  [`concept_04_structs_enums`](concept_04_structs_enums/)
- struct & methods
- enum (with data)
  - built-in enums
- Option
- pattern matching

### 05 — Error Handling  ·  [`concept_05_error_handling`](concept_05_error_handling/)
> Placed early: the biggest mindset shift for exception-based (try/catch) devs.
- Result
- panic
- the `?` operator
- thiserror
- anyhow

### 06 — Traits  ·  [`concept_06_traits`](concept_06_traits/)
- traits (~ interfaces)
- into, from
- dispatch: static, dynamic
- trait bounds
- Sized vs ?Sized
- operator overloading
- derive / Display

### 07 — Generics  ·  [`concept_07_generics`](concept_07_generics/)
- generics
- trait bounds in generics

### 08 — Collections  ·  [`concept_08_collections`](concept_08_collections/)
- Vec
- HashMap
- HashSet
- String vs str

### 09 — Closures & Iterators  ·  [`concept_09_closures_iterators`](concept_09_closures_iterators/)
- closures
- Fn, FnMut, FnOnce
- iterators
  - iter, into_iter, iter_mut
  - iterator adaptors
  - generic type vs associated type
- function pointers

### 10 — Lifetimes  ·  [`concept_10_lifetimes`](concept_10_lifetimes/)
- lifetimes
- struct lifetimes

### 11 — Smart Pointers  ·  [`concept_11_smart_pointers`](concept_11_smart_pointers/)
- Box
- Rc (Reference Count)
- RefCell
- Weak reference
- Arc (Atomic Reference Count) — multi-thread env
- Mutex — multi-thread env

### 12 — Concurrency  ·  [`concept_12_concurrency`](concept_12_concurrency/)
- threads
  - scoped threads
- channels
- Mutex
- Arc

### 13 — Async (Tokio)  ·  [`concept_13_async_tokio`](concept_13_async_tokio/)
- async / await
- tasks
- async channels
- native vs async

### 14 — Macros  ·  [`concept_14_macros`](concept_14_macros/) · [`concept_14_proc_macro`](concept_14_proc_macro/)
- declarative (`macro_rules!`)
- procedural (derive, attribute, function-like)

### 15 — Unsafe & FFI  ·  [`concept_15_unsafe_ffi`](concept_15_unsafe_ffi/)
- unsafe
- FFI

## Projects & Desktop (not part of the reorder)

Kept as-is: `proj_01_cli_todo`, `proj_02_file_parser`, `proj_03_web_api_axum`,
`proj_04_grpc`, `tauri_01_notes`, `tauri_02_projects`, `capstone`.
