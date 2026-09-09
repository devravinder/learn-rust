# Migration Tasks — reorder `learn_*` → `concept_*`

Goal: renumber/rename lesson folders to match [`roadmap.md`](roadmap.md), merge
the `learn_01_core_relearn` files into topical folders, and switch the prefix
`learn_` → `concept_`.

## Rules (hard constraints)

- **No code loss.** Rust logic stays byte-for-byte. Only allowed edits: the
  `// header` / `Run:` comment lines and binary/file names.
- Use `git mv` for every move (preserves history + diffs).
- Per task: move → rename binaries + update header lines → update workspace
  `Cargo.toml` members → update root `README.md` mermaid → fill roadmap link →
  `cargo build` the affected package → **one commit** (short message).
- Out of scope (do not touch): `proj_*`, `tauri_*`, `capstone`.

## Rename map (current → target)

| Current | Target |
| --- | --- |
| `learn_01_core` | `concept_01_core` |
| `learn_02_ownership` | `concept_02_ownership` |
| `learn_07_modules_crates` | `concept_03_modules_crates` |
| `learn_03_structs_enums` | `concept_04_structs_enums` |
| `learn_06_error_handling` | `concept_05_error_handling` |
| `learn_05_traits` (+ relearn operator_overload) | `concept_06_traits` |
| `learn_04_collections_generics` (generics file) | `concept_07_generics` |
| `learn_04_collections_generics` (rest) | `concept_08_collections` |
| `learn_08_closures_iterators` (+ relearn iterators/adaptors/iter_into_iter/function_pointer) | `concept_09_closures_iterators` |
| `learn_10_lifetimes` (+ relearn lifetime) | `concept_10_lifetimes` |
| `learn_09_smart_pointers` | `concept_11_smart_pointers` |
| `learn_11_concurrency` | `concept_12_concurrency` |
| `learn_12_async_tokio` | `concept_13_async_tokio` |
| `learn_13_macros` + `learn_13_proc_macro` | `concept_14_macros` + `concept_14_proc_macro` |
| `learn_14_unsafe_ffi` | `concept_15_unsafe_ffi` |
| `learn_01_core_relearn` | dissolved (files distributed above) |
| `learn_00_setup/setup.md` | `./setup.md` (root); remove empty folder |

---

## Tasks (one commit each)

### T0 — Move setup.md to root
- `git mv learn_00_setup/setup.md setup.md`
- remove empty `learn_00_setup/` folder
- commit: `chore: setup docs`

### T1 — concept_01_core
- `git mv learn_01_core concept_01_core`
- rename binaries `learn_01_NN_*` → `concept_01_NN_*` (+ header/Run lines)
- update `Cargo.toml` member + `README.md` mermaid + roadmap link
- `cargo build -p concept_01_core`
- commit: `feat: core concepts`

### T2 — concept_02_ownership
- `git mv learn_02_ownership concept_02_ownership`
- rename binaries + headers; update Cargo/README/roadmap; build
- commit: `feat: ownership & borrowing`

### T3 — concept_03_modules_crates (was learn_07)
- `git mv learn_07_modules_crates concept_03_modules_crates`
- rename binaries `learn_07_*` → `concept_03_*` (incl. file-module submodule dir)
- update Cargo/README/roadmap; build
- commit: `feat: modules & crates`

### T4 — concept_04_structs_enums (was learn_03)
- `git mv learn_03_structs_enums concept_04_structs_enums`
- rename binaries `learn_03_*` → `concept_04_*`
- update Cargo/README/roadmap; build
- commit: `feat: structs & enums`

### T5 — concept_05_error_handling (was learn_06)
- `git mv learn_06_error_handling concept_05_error_handling`
- rename binaries `learn_06_*` → `concept_05_*`
- update Cargo/README/roadmap; build
- commit: `feat: error handling`

### T6 — concept_06_traits (was learn_05, + relearn operator_overload)
- `git mv learn_05_traits concept_06_traits`
- rename binaries `learn_05_*` / `lession_05_06_*` → `concept_06_*`
  (fix the `lession` typo while renaming)
- `git mv learn_01_core_relearn/src/bin/learn_16_operator_overload.rs`
  → `concept_06_traits/src/bin/concept_06_07_operator_overload.rs` (+ header)
- update Cargo/README/roadmap; build
- commit: `feat: traits`

### T7 — concept_07_generics (split from learn_04)
- create `concept_07_generics` package (Cargo.toml + README)
- `git mv learn_04_collections_generics/src/bin/learn_04_04_generics.rs`
  → `concept_07_generics/src/bin/concept_07_01_generics.rs` (+ header)
- add member to Cargo; update README/roadmap; build
- commit: `feat: generics`

### T8 — concept_08_collections (rest of learn_04)
- `git mv learn_04_collections_generics concept_08_collections`
- rename remaining binaries (`vectors, hashmap, string_vs_str, hashset`)
  `learn_04_*` → `concept_08_NN_*`
- update Cargo/README/roadmap; build
- commit: `feat: collections`

### T9 — concept_09_closures_iterators (was learn_08, + relearn iterators)
- `git mv learn_08_closures_iterators concept_09_closures_iterators`
- rename binaries `learn_08_*` → `concept_09_*`
- move relearn files (append numbering, + headers):
  - `learn_15_iterators.rs`            → `concept_09_05_iterators_advanced.rs`
  - `learn_17_iter_into_iter_iter_mut.rs` → `concept_09_06_iter_into_iter_iter_mut.rs`
  - `learn_18_iterator_adaptoers.rs`   → `concept_09_07_iterator_adaptors.rs`
  - `learn_20_function_pointer.rs`     → `concept_09_08_function_pointer.rs`
- update Cargo/README/roadmap; build
- commit: `feat: closures & iterators`

### T10 — concept_10_lifetimes (was learn_10, + relearn lifetime)
- `git mv learn_10_lifetimes concept_10_lifetimes`
- rename binaries `learn_10_*` → `concept_10_*`
- `git mv learn_01_core_relearn/src/bin/learn_19_lifetime.rs`
  → `concept_10_lifetimes/src/bin/concept_10_03_lifetime_advanced.rs` (+ header)
- update Cargo/README/roadmap; build
- commit: `feat: lifetimes`

### T11 — remove emptied learn_01_core_relearn
- confirm folder now has no `.rs` left; `git mv` its Cargo.toml/README if any
  content worth keeping, else remove the emptied package
- remove member from workspace Cargo.toml
- build workspace
- commit: `chore: remove emptied relearn package`

### T12 — concept_11_smart_pointers (was learn_09)
- `git mv learn_09_smart_pointers concept_11_smart_pointers`
- rename binaries `learn_09_*` → `concept_11_*`
- update Cargo/README/roadmap; build
- commit: `feat: smart pointers`

### T13 — concept_12_concurrency (was learn_11)
- `git mv learn_11_concurrency concept_12_concurrency`
- rename binaries `learn_11_*` → `concept_12_*`
- update Cargo/README/roadmap; build
- commit: `feat: concurrency`

### T14 — concept_13_async_tokio (was learn_12)
- `git mv learn_12_async_tokio concept_13_async_tokio`
- rename binaries `learn_12_*` → `concept_13_*`
- update Cargo/README/roadmap; build
- commit: `feat: async with tokio`

### T15 — concept_14_macros (+ concept_14_proc_macro)
- `git mv learn_13_macros concept_14_macros`
- `git mv learn_13_proc_macro concept_14_proc_macro`
- rename binaries `learn_13_*` → `concept_14_*`; update the proc_macro path dep
- update Cargo/README/roadmap; build (+ `cargo test` the dry lesson)
- commit: `feat: macros`

### T16 — concept_15_unsafe_ffi (was learn_14)
- `git mv learn_14_unsafe_ffi concept_15_unsafe_ffi`
- rename binaries `learn_14_*` → `concept_15_*`
- update Cargo/README/roadmap; build
- commit: `feat: unsafe & ffi`

### T17 — final sweep
- full `cargo build` workspace
- verify root `README.md` mermaid + all roadmap links resolve
- commit: `docs: finalize roadmap links`
