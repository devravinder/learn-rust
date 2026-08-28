# Setup

## Installation

- [Follow Docs](https://rust-lang.org/learn/get-started/)

## VSC Editor Setup

- install extensions
  - `rust-analyzer`
  - `CodeLLDB`
  - `Even Better TOML`

## Cargo Usage

- new project: `cargo new basics`
  - always open a project root folder in a VSC instance for better IDE support
  - `cd basics && code .`

- build your project with `cargo build`
- run your project with `cargo run`
  - by default it'll run `main` executable binary
- test your project with `cargo test`
- build documentation for your project with `cargo doc`
- publish a library to crates.io with `cargo publish`

### Running Multiple Files With Cargo

- by default `cargo run` runs `src/main.rs`
  - it trats only `main.rs` as executable
- but we keep files under `src/bin`, then it'll treat every file as executable
  - without bin only `src/main.rs` is executable
  - with bin every file is executable

  - ```text
     src/
      ├── main.rs
      └── bin/
          ├── variables.rs
          ├── ownership.rs
          └── structs.rs
     ```

## To Run without IDE

- compile - this produces executable
  - `rustc main.rs`
- to run - `.main`

## Recomondations

- always use cargo
