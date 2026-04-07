# wispy-vm

Fork of [Stak Scheme](https://github.com/raviqqe/stak) with the [Cayley table](https://github.com/stefanopalmieri/wispy-table) integrated into the VM. Bytecode execution, semi-space GC, `no_std`/`no_alloc`, and native algebra primitives (`dot`, `tau`, `type-valid?`).

Named after Wispy the guinea pig.

## What it is

An R7RS Scheme bytecode VM where type dispatch is branchless: instead of tag-bit branch chains, every dispatch decision is a single index into a 32×32 Cayley table. The table is 1KB, lives in `.rodata`, and is transparent to the optimizer.

```
TABLE[CAR][T_PAIR] → T_PAIR   (valid: proceed to car field)
TABLE[CAR][T_STR]  → BOT      (type error → returned as a value, not an exception)
TABLE[TAU][T_PAIR] → T_PAIR   (classify: it's a pair)
TABLE[TAU][T_SYM]  → T_SYM    (classify: it's a symbol)
```

The programmer can inspect and reason about the type system at runtime via three primitives: `dot` (table lookup), `tau` (classify a value), and `type-valid?` (check validity). The full language semantics (evaluation, scoping, closures, continuations, GC) come from Stak's Ribbit-derived VM. The table captures which operations are valid on which types and the algebraic relationships between operations.

## Quick start

```bash
# Build
cargo build --release --package wispy --package wispy-repl

# Run a file (compile + execute)
./target/release/wispy examples/wispy/algebra-smoke.scm     # 83 tests pass
./target/release/wispy examples/wispy/reflective-tower.scm   # 12ms execution

# Interactive REPL (164ms startup, instant per-expression)
./target/release/wispy-repl
wispy> (dot CAR T_PAIR)
12
wispy> (tau (cons 1 2))
12
wispy> (type-valid? CAR T_STR)
#f
wispy> (define (fib n) (if (< n 2) n (+ (fib (- n 1)) (fib (- n 2)))))
wispy> (fib 10)
55

# Compile to bytecode separately
./target/release/wispy compile examples/wispy/fib.scm -o fib.bc
./target/release/wispy run fib.bc
```

## The Algebra

All 12 core elements (`TOP`, `BOT`, `Q`, `E`, `CAR`, `CDR`, `CONS`, `RHO`, `APPLY`, `CC`, `TAU`, `Y`) and 8 type tags (`T_PAIR`, `T_SYM`, `T_CLS`, `T_STR`, `T_VEC`, `T_CHAR`, `T_CONT`, `T_PORT`) are bound as constants. `dot`, `tau`, and `type-valid?` are the three primitives.

```scheme
;; Table lookup: CAYLEY[a][b]
(dot CAR T_PAIR)          ; → T_PAIR (car of pair is valid)
(dot CAR T_STR)           ; → BOT (car of string is a type error)

;; Classify any value
(tau (cons 1 2))          ; → T_PAIR
(tau "hello")             ; → T_STR
(tau 42)                  ; → TOP (fixnum)

;; Retraction pair: Q and E are exact inverses
(dot E (dot Q CAR))       ; → CAR (round-trip)

;; Y fixed point: algebraic, not computed
(dot RHO (dot Y RHO))    ; → (dot Y RHO)
```

The algebra layer is always total: every input produces an output, no operation throws. Type errors return `BOT` as a value. This gives the specializer and reflective tower composability — they can fold through error cases because BOT is just another value.

The table's 12-element core was found by Z3 and is axiomatically equivalent to the [Kamea](https://github.com/stefanopalmieri/Kamea) project's Ψ₁₆ algebra. 14 Lean-proved theorems (zero `sorry`) verify absorbers, retraction, classifier dichotomy, branch, composition, Y fixed point, and extensionality. Proofs live in [wispy-table](https://github.com/stefanopalmieri/wispy-table).

## Self-hosted tools

Scheme programs running on wispy-vm, ported from the [Kamea](https://github.com/stefanopalmieri/Kamea) project's Psi Lisp originals:

| File | What it does | Tests |
|------|-------------|-------|
| `algebra-smoke.scm` | Absorbers, retraction, classifier, composition, Y fixed point | 83 |
| `pe.scm` | Online partial evaluator for Scheme — Futamura Projection 1 | 29 |
| `metacircular.scm` | Defunctionalized CPS evaluator with 14 inspectable continuation types | 25 |
| `reflective-tower.scm` | Three-level Smith (1984) tower with continuation modification | 20 |
| `futamura-real.scm` | Futamura P1 on a real Scheme evaluator (four-path verification) | 10 |
| `futamura-cps.scm` | Futamura P2 — eliminates interpreter dispatch, preserves continuations | 23 |
| `futamura.scm` | All three Futamura projections on the 32×32 algebra | 15 |
| `specialize.scm` | Partial evaluator for algebraic IR: constant-folds `dot`, cancels QE pairs | — |
| `transpile.scm` | IR → Rust code generator | — |

**The reflective tower** demonstrates three levels grounded in the Cayley table:

- **Level 0:** User programs (fib, fact) run through the meta-evaluator
- **Level 1:** The meta-evaluator probes the 32×32 table to verify algebraic invariants
- **Level 2:** `(reify)` captures the current continuation as walkable data; the program navigates the continuation chain, swaps the then/else branches of a pending `if`, and `(reflect)`s into the modified future

Every continuation is a tagged list, not a closure. The program can read, modify, and rewrite its own control flow. Smith's 3-Lisp on a bytecode VM with garbage collection.

## Part of the Wispy ecosystem

| Repo | What |
|------|------|
| [**wispy-table**](https://github.com/stefanopalmieri/wispy-table) | 1KB Cayley table + Lean proofs (14 theorems) + Z3 search |
| **wispy-vm** (this repo) | Stak VM fork + REPL + examples + benchmarks |
| [**wispy-compile**](https://github.com/stefanopalmieri/wispy-compile) | Scheme → Rust AOT compiler (1.7× faster than Chez) |

## What's different from upstream Stak

- **`vm/src/type.rs`** — re-exports [wispy-table](https://github.com/stefanopalmieri/wispy-table) (Cayley table, element constants, `dot()`)
- **`wispy/` crate** — `WispyPrimitiveSet` (wraps `SmallPrimitiveSet` with algebra ops at IDs 600-602), `compile_wispy()`, `(wispy algebra)` prelude, CLI with `(load)` resolution
- **`cmd/wispy-repl/`** — interactive REPL with algebra pre-loaded (164ms startup)
- **`examples/wispy/`** — 15 Scheme programs (205 test assertions)
- **`benchmarks/wispy/`** — compiler benchmarks and C reference implementations

Everything else is stock Stak v0.12.11. Upstream updates via `git fetch upstream && git merge upstream/main`.

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
