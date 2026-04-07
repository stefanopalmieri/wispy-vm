# wispy-vm

Fork of [Stak Scheme](https://github.com/raviqqe/stak) with the [Cayley table](https://github.com/stefanopalmieri/wispy-table) integrated into the VM. Bytecode execution, semi-space GC, `no_std`/`no_alloc`, and native algebra primitives (`dot`, `tau`, `type-valid?`).

## What's different from upstream Stak

- **Cayley table in `vm/src/type.rs`** — the 32×32 algebraic dispatch table (1KB), element constants, and `dot()` function
- **`wispy/` crate** — `WispyPrimitiveSet` (wraps `SmallPrimitiveSet` with algebra ops at IDs 600-602), `compile_wispy()` compiler entry point, `(wispy algebra)` prelude library, CLI with `(load)` resolution
- **`cmd/wispy-repl/`** — interactive REPL with algebra pre-loaded into the interaction environment (164ms startup)
- **`examples/wispy/`** — 15 Scheme programs: reflective tower, partial evaluator, Futamura projections, metacircular CPS evaluator, algebra smoke tests
- **`benchmarks/wispy/`** — compiler benchmark inputs and C reference implementations

Everything else is stock Stak v0.12.11. Upstream updates via `git fetch upstream && git merge upstream/main`.

## Quick start

```bash
# Build
cargo build --release --package wispy
cargo build --release --package wispy-repl

# Run a file (compile + execute)
./target/release/wispy examples/wispy/algebra-smoke.scm     # 83 tests pass
./target/release/wispy examples/wispy/reflective-tower.scm   # 12ms execution

# Interactive REPL
./target/release/wispy-repl
wispy> (dot CAR T_PAIR)
12
wispy> (tau (cons 1 2))
12
wispy> (define (fib n) (if (< n 2) n (+ (fib (- n 1)) (fib (- n 2)))))
wispy> (fib 10)
55

# Compile to bytecode separately
./target/release/wispy compile examples/wispy/fib.scm -o fib.bc
./target/release/wispy run fib.bc
```

## Part of the Wispy ecosystem

| Repo | What |
|------|------|
| [**wispy-table**](https://github.com/stefanopalmieri/wispy-table) | 1KB Cayley table + Lean proofs (14 theorems) + Z3 search |
| **wispy-vm** (this repo) | Stak VM fork + REPL + examples + benchmarks |
| [**wispy-compile**](https://github.com/stefanopalmieri/wispy-compile) | Scheme → Rust AOT compiler (1.7× faster than Chez) |

## Test coverage

| Example | Tests | Status |
|---------|-------|--------|
| algebra-smoke.scm | 83 | PASS |
| pe.scm | 29 | PASS |
| metacircular.scm | 25 | PASS |
| reflective-tower.scm | 20 | PASS |
| futamura-real.scm | 10 | PASS |
| futamura-cps.scm | 23 | PASS |
| futamura.scm | 15 | PASS |

Plus 9 unit tests in `wispy/src/lib.rs` covering `dot`, `tau`, `type-valid?`.

## Performance

| Runtime | Mode | Time |
|---------|------|------|
| wispy-compile → Rust | AOT native | 5.3ms (fib 30) |
| wispy-vm | bytecode VM | 160ms (fib 30) |
| wispy-vm | pre-compiled bytecode | 12ms (reflective tower) |
| wispy-repl | startup + 1 eval | 164ms |

## Architecture

```
wispy/src/
├── lib.rs            compile_wispy() — prepends (wispy algebra) + compile_r7rs
├── primitive_set.rs  WispyPrimitiveSet — dot(600), tau(601), type-valid?(602)
├── prelude.scm       (wispy algebra) library: 23 constants + 3 native primitives
└── main.rs           CLI: wispy [compile|run] file.scm

cmd/wispy-repl/
├── build.rs          compile-time: prelude + REPL source → bytecode
├── src/main.rs       runtime: VM with WispyPrimitiveSet
└── src/main.scm      REPL loop with (wispy algebra) in interaction-environment
```

## Upstream

Forked from [raviqqe/stak](https://github.com/raviqqe/stak) v0.12.11 — the miniature, embeddable R7RS Scheme in Rust. Based on [Ribbit Scheme](https://github.com/udem-dlteam/ribbit).

## License

[MIT](LICENSE)
