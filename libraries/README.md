# libraries

Code that knows nothing about Terralistic. Everything here could be lifted into a different
game, or into something that is not a game — that is the rule the directory exists for, and it
is what makes these safe to work inside without reading the game first.

## The contract

Something belongs here when all four hold:

- **Its public API names no game concept.** No `Block`, `Player`, `World`. `Grid<T>`,
  `Registry<T>`, `PacketServer`, `FixedStep` — yes.
- You could plausibly use it somewhere else.
- Its `mod.rs` opens with what it does **and what it is not for**. The second half is the
  load-bearing one: it is what stops game logic drifting in a function at a time.
- It compiles without `crate::shared`, `crate::client` or `crate::server`.

The last one holds today and is not enforced by the compiler — this is a module split inside
one binary crate, not a workspace — so it is worth a grep before adding anything:

```bash
grep -rn "crate::shared\|crate::client\|crate::server" libraries/   # should print nothing
```

## The libraries

Each `mod.rs` holds the real contract, including what it refuses to do.

| Library | Code | Tests | What it does |
|---|---:|---:|---|
| [`graphics`](graphics/) | 3,426 | 1,843 | The renderer: records draw commands and turns them into pixels through wgpu. Owns the window and input |
| [`ui`](ui/) | 1,645 | 1,371 | The widget toolkit: layout, hit testing, input routing, widgets, and the composites built from them |
| [`net`](net/) | 727 | 325 | A TCP transport carrying Rust values, both halves, on background threads. The transport, not the protocol |
| [`timing`](timing/) | 367 | 315 | Clocks: fixed-step accumulators, a frame limiter, work budgets, frame statistics |
| [`scripting`](scripting/) | 324 | 180 | Sandboxed lua modules with named binary resources, and their on-disk package format |
| [`grid`](grid/) | 242 | 318 | Bounded 2D grids, chunk addressing, and least-recently-touched eviction |
| [`config`](config/) | 145 | 273 | Settings registered at runtime, persisted as a flat key-to-number file |
| [`fixed`](fixed/) | 260 | 151 | Deterministic 16.16 fixed-point numbers: exact, comparable, hashable |
| [`registry`](registry/) | 138 | 187 | Register a value, get a typed handle back; look it up by handle or by name |
| [`container_file`](container_file/) | 101 | 102 | Versioned binary files: a magic string, a format version, then named sections |
| [`log`](log/) | 98 | 110 | Timestamped, levelled lines, and one process-global place for them to go |
| [`procgen`](procgen/) | 96 | 131 | Fractal noise, 1D smoothing, and weighted random picks |
| [`testing`](testing/) | 92 | — | `#[cfg(test)]` only: temp dirs, free ports, spin-until-or-fail |
| [`serialization`](serialization/) | 34 | 85 | The one place the binary format is chosen |
| [`events`](events/) | 49 | 71 | A type-erased event queue: push anything, downcast to read it |

## Conventions

- One folder per library. `mod.rs` re-exports; `tests.rs` beside it holds
  `#![cfg(test)] mod tests { .. }`.
- Tests never need a graphics context or a window, so the whole suite runs on headless CI.
- Errors are `anyhow::Result`. `unwrap`, `expect`, `panic` and indexing are warned on
  crate-wide; a local `#[allow(..)]` with a reason is the convention when one is right.

## Adding one

Ask what it *refuses* to do before what it does — if that half is hard to write, the boundary
is in the wrong place. `docs/LIBRARIES.md` records what was extracted and what was deliberately
left in the game.
