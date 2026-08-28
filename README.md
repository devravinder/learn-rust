# Learn Rust — Beginner to Advanced

A hands-on Rust learning workspace, structured for someone coming from
**Java, Node.js, TypeScript, React & Angular**. Each concept is a small,
runnable program with comments that map Rust ideas back to languages you
already know.

## How this repo is organized

This is a **Cargo workspace**: one repo, many small packages (crates), one
shared `target/` build directory.

- Folders are numbered sequentially (`learn_01_`, `learn_02_`, ...) in the
  intended learning order.
- Each folder is a Cargo package. Concepts live as separate executables under
  `src/bin/`.
- Every binary name is **globally unique** (prefixed with its folder number,
  e.g. `learn_01_01_hello`). This keeps the VS Code shortcut working from
  anywhere in the workspace.

### Running a concept

```bash
# From anywhere in the workspace:
cargo run --bin learn_01_02_variables

# Or classic (cd into the folder first):
cd learn_01_core && cargo run --bin learn_01_02_variables
```

VS Code shortcut (unchanged — works because binary names are unique):

```json
{
  "key": "ctrl+r ctrl+r",
  "command": "workbench.action.terminal.sendSequence",
  "args": { "text": "cargo run --bin ${fileBasenameNoExtension}\u000D" }
}
```

## Roadmap

```mermaid
graph TD
    subgraph Basics
        A[learn_01_core<br/>vars, types, functions, control flow] --> B[learn_02_ownership<br/>ownership, borrow, slices]
        B --> C[learn_03_structs_enums<br/>structs, enums, Option, match]
    end
    subgraph Intermediate
        C --> D[learn_04_collections_generics<br/>Vec, HashMap, generics]
        D --> E[learn_05_traits<br/>traits ~ interfaces]
        E --> F[learn_06_error_handling<br/>Result, ?, thiserror, anyhow]
        F --> P1[proj_01_cli_todo<br/>clap + serde]
        P1 --> G[learn_07_modules_crates]
        G --> H[learn_08_closures_iterators]
        H --> I[learn_09_smart_pointers<br/>Box, Rc, RefCell, Arc]
        I --> J[learn_10_lifetimes]
        J --> P2[proj_02_file_parser<br/>CSV/JSON]
    end
    subgraph Advanced
        P2 --> K[learn_11_concurrency<br/>threads, Mutex, channels]
        K --> L[learn_12_async_tokio<br/>async/await, Tokio]
        L --> M[learn_13_macros]
        M --> N[learn_14_unsafe_ffi]
        N --> P3[proj_03_web_api_axum<br/>Axum + Tokio + sqlx]
        P3 --> P4[proj_04_grpc<br/>tonic gRPC]
    end
    subgraph Desktop
        P4 --> T1[tauri_01_notes]
        T1 --> T2[tauri_02_projects]
        T2 --> CAP[capstone<br/>React-TS 19 + Tailwind 4 + Vite + Tauri]
    end
```

## Concept → language you already know

| Rust | Closest thing you know |
| --- | --- |
| `let` (immutable by default) | `const` in JS/TS |
| `let mut` | `let`/`var` in JS |
| Ownership & borrow checker | *no equivalent* — this is the big new idea |
| `struct` + `impl` | class fields + methods (but no inheritance) |
| `trait` | `interface` (Java/TS) |
| `enum` (with data) | tagged unions / sealed classes |
| `Option<T>` | nullable types (`T | null`) but enforced |
| `Result<T, E>` | `try/catch` made explicit in the type |
| `Vec<T>` | `ArrayList` / JS array |
| `HashMap<K,V>` | `HashMap` / JS `Map` / object |
| closures `|x| x + 1` | arrow functions |
| iterators `.map().filter()` | array methods in JS/TS |
| `async`/`await` + Tokio | `async`/`await` + Node event loop |
| Cargo | npm/Maven + build tool combined |

## Crates you'll meet along the way

| Crate | Purpose | JS/Java analogy |
| --- | --- | --- |
| `rand` | randomness | `Math.random` |
| `serde` / `serde_json` | (de)serialization | `JSON.parse/stringify` |
| `clap` | CLI arg parsing | `yargs`/`commander` |
| `anyhow` / `thiserror` | error handling | custom `Error` classes |
| `tokio` | async runtime | Node event loop |
| `axum` / `actix-web` | web frameworks | Express / Spring |
| `reqwest` | HTTP client | `fetch` / `axios` |
| `sqlx` | async SQL | Prisma / JDBC |
| `tonic` | gRPC | grpc-js |
| `tracing` | logging/observability | `winston` / SLF4J |
| `tauri` | desktop apps | Electron (but Rust core) |

## Prerequisites

See [`learn_00_setup/setup.md`](learn_00_setup/setup.md).
