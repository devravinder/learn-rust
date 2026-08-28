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

- manual
  - `rustc main.rs`
  - to run - `.main`

- with Cargo
  - `cargo run`

- with Short Keys in VSC: `Ctrl + R, Ctrl + R`
  - add the below short key (to open short keys:  Ctrl + K, Ctrl + S )

  ```json
     {
       "key": "ctrl+r ctrl+r",
       "command": "workbench.action.terminal.sendSequence",
       "args": { "text": "cargo run --bin ${fileBasenameNoExtension}\u000D" }
     }
  ```

## Recomondations

- always use cargo

## Workspace Layout (this repo)

This repo is a **single Cargo workspace**: one root `Cargo.toml` lists all the
`learn_*` and `proj_*` packages as `members`, and they share ONE `target/`
directory and ONE `Cargo.lock`.

```text
learn-rust/
├── Cargo.toml            # workspace: members + shared [workspace.dependencies]
├── learn_01_core/        # each folder = one package (crate)
│   ├── Cargo.toml
│   └── src/bin/          # each file here = one runnable binary
├── learn_02_ownership/
├── ...
├── proj_01_cli_todo/     # projects have a src/main.rs (one app)
├── ...
└── capstone/             # standalone Tauri app (NOT a workspace member)
```

- **Folders** are numbered sequentially (`learn_01_`, `learn_02_`, ...).
- **Binary names** are globally unique (`learn_01_01_hello`) so
  `cargo run --bin <name>` (and the VS Code shortcut) works from anywhere.
- **Run any concept** from the workspace root: `cargo run --bin learn_01_01_hello`.
- **Add a new package**: create the folder + its `Cargo.toml`, then add it to the
  `members` array in the root `Cargo.toml`.

## Fixing the red "rust-analyzer" badge

When you open the whole workspace in VS Code, rust-analyzer auto-discovers every
`Cargo.toml` — including `capstone/`, which is a **separate** Tauri workspace
whose build script needs the Node + WebView toolchain (and generated icons /
frontend `dist/`) that don't exist until you scaffold/run it. That failing build
script shows up as the red `rust-analyzer` badge — it is NOT a problem with the
`learn_*`/`proj_*` code (which builds and runs fine).

Fix: this repo ships `.vscode/settings.json` that pins rust-analyzer to the root
workspace and excludes `capstone/`:

```jsonc
{
  "rust-analyzer.linkedProjects": ["./Cargo.toml"],
  "rust-analyzer.files.excludeDirs": ["capstone", "target"],
  "rust-analyzer.check.command": "check",   // use "clippy" if installed
  "rust-analyzer.check.workspace": true
}
```

After it's in place: Command Palette (`Ctrl+Shift+P`) →
`Developer: Reload Window`, or `rust-analyzer: Restart Server`. The badge clears.

To work *inside* the Tauri app, open the `capstone/` folder as its **own** VS Code
window (after `npm install` there), so rust-analyzer loads it correctly.

## Toolchain per stage

Most modules need only Rust + Cargo. A few projects pull extra tooling:

| Stage | Needs beyond Rust |
| --- | --- |
| `learn_*`, `proj_01`, `proj_02` | nothing extra |
| `proj_03_web_api_axum` | nothing extra (in-memory SQLite; test with `curl`) |
| `proj_04_grpc` | nothing extra — a vendored `protoc` is bundled via `protoc-bin-vendored` |
| `capstone` (Tauri) | **Node.js/npm** + OS WebView/build tools (see below) |

### Optional but recommended Rust components

```bash
rustup component add clippy   # richer lints (then set check.command to "clippy")
rustup component add rustfmt  # cargo fmt
```

### Tauri prerequisites (only for `capstone/`)

- **Windows**: WebView2 (preinstalled on Win10+/11) + MSVC build tools.
- **macOS**: Xcode command line tools.
- **Linux**: `webkit2gtk`, `libayatana-appindicator`, etc.
- Frontend: Node.js + npm. Then, inside `capstone/`:

  ```bash
  npm install
  npm run tauri dev     # dev with hot reload
  npm run tauri build   # produce an installer
  ```

- App icons are generated (not committed): `npm run tauri icon path/to/logo.png`.

