# libraries

Code that knows nothing about Terralistic.

Everything here could be lifted into a different game, or into something that is not a game
at all. That is not an aspiration — it is the rule the directory is for, and it is what makes
these safe to work inside without reading the game first.

## The rule

Something belongs here when all four hold:

- **Its public API names no game concept.** No `Block`, `Player`, `World`. `Grid<T>`,
  `Registry<T>`, `PacketServer`, `FixedStep` — yes.
- You could plausibly use it somewhere else.
- Its `mod.rs` opens with what it does **and a `# Not in scope` section**. The second half is
  the load-bearing one: it is what stops game logic drifting in a function at a time.
- It compiles without `crate::shared`, `crate::client` or `crate::server`.

That last one is true of every library here today. It is not enforced by the compiler — this
is a module split inside one binary crate, not a workspace — so it is worth a grep before
adding anything:

```bash
grep -rn "crate::shared\|crate::client\|crate::server" libraries/   # should print nothing
```

## What each one is

| Library | Code | Tests | What it does |
|---|---:|---:|---|
| [`graphics`](graphics/) | 3,681 | 1,899 | The renderer. Records draw commands and turns them into pixels through wgpu; owns the window and input |
| [`ui`](ui/) | 1,741 | 1,385 | The widget toolkit. Layout, hit testing, input routing, widgets, and the composites built from them |
| [`net`](net/) | 804 | 325 | A TCP transport that carries Rust values. Both halves, on background threads |
| [`timing`](timing/) | 407 | 316 | Clocks. Fixed-step accumulators, a frame limiter, work budgets, frame statistics |
| [`scripting`](scripting/) | 350 | 180 | Sandboxed lua modules with named binary resources, and their on-disk package format |
| [`grid`](grid/) | 267 | 322 | Bounded 2D grids, chunk addressing, and least-recently-touched eviction |
| [`config`](config/) | 163 | 274 | Settings registered at runtime, persisted as a flat key-to-number file |
| [`registry`](registry/) | 155 | 187 | Register a value, get a typed handle back; look it up by handle or by name |
| [`container_file`](container_file/) | 112 | 102 | Versioned binary files: a magic string, a format version, then named sections |
| [`log`](log/) | 109 | 113 | Timestamped, levelled lines, and one process-global place for them to go |
| [`procgen`](procgen/) | 104 | 131 | Fractal noise, 1D smoothing, and weighted random picks |
| [`testing`](testing/) | 104 | — | `#[cfg(test)]` only: temp dirs, free ports, spin-until-or-fail |
| [`serialization`](serialization/) | 51 | 85 | The one place the binary format is chosen |
| [`events`](events/) | 50 | 71 | A type-erased event queue: push anything, downcast to read it |

Each `mod.rs` has the real contract. What follows is why you would open each one.

### `graphics` — what to draw, and how it becomes pixels

**No drawing call touches the graphics API.** `rect.render(..)`, `texture.render(..)` and
friends *record* a `DrawCommand` into the frame's `DrawList`, and one call hands the whole
list to the wgpu backend. That seam is why `ui` is testable with no GPU and why the
golden-image suite is possible at all.

Also here: the window and input (the only module that talks to winit), surfaces, textures,
the texture atlas, glyph rasterisation, and the blur.

### `ui` — layout, input, and widgets

You implement `UiElement`, whose only required method is `get_container`, and call
`BaseUiElement`, which is blanket-implemented and recurses into children.

`UiContext` is the whole non-rendering surface a widget needs — window size, mouse position,
key states, clipboard. Layout and input go through it, so `HeadlessContext` can drive real
widgets in `cargo test` with no window.

Beyond the primitives, three composites are worth knowing before writing a screen:
`ListPage` (a scrolling list between a title bar and a button bar), `MenuStack` (a stack of
screens where the top one is live), and `DockNode` (a binary split-pane layout).

### `net` — the transport, not the protocol

A `Packet` is any serializable type, identified on the wire by a hash of that type — no
registry, no ids to allocate. `PacketServer` accepts connections, `PacketClient` makes one,
both on threads of their own.

**Who is allowed to connect and what has to be said first are explicitly not here.** The
server hands over every packet from every connected peer, including one the owner is about to
refuse; the client's handshake is two things the owner supplies, a greeting and a predicate
saying which packet ends it.

### `timing` — is it time yet, how long should I sleep, have I spent my budget

One accumulator behind all of it. `FixedStep::new` owes every step that elapsed, because a
simulation that skips one has silently run slow; `FixedStep::for_animation` caps the backlog,
because frames nobody saw are worth nothing. Picking the wrong constructor is the bug this
library exists to make visible.

### `scripting` — modules, not a sandbox

Each module gets its own interpreter state, so two cannot clobber each other's globals.
Everything a module reaches outside itself is a host function the owner registered, under a
prefix the owner chose. What the lifecycle hooks *mean*, and what a symbol named by
convention implies, are the owner's.

`module_data.rs` is a deliberately dependency-free leaf so a build script can write packages
without linking the interpreter.

### `grid` — where is cell (x, y) in this Vec

A dense grid that is always checked against its own size, so an out-of-bounds coordinate is an
error rather than a read of the wrong cell. The fill value is an argument to `Grid::filled`,
so "cells meaning nothing is here yet" has to be said rather than implied.

`Chunks` is the partitioning, and its indices are **row major** while `Grid`'s cells are
**column major**. They disagree on purpose; there is a test pinning it.

### `config`, `container_file`, `log`, `procgen`, `serialization`, `events`

Small and single-purpose:

- **`config`** — a setting's id is a handle from a counter that never reuses one, *not* a row.
  Lay settings out by position in a sorted list.
- **`container_file`** — the header is checked before the body is handed to any decoder, which
  is the whole point: a version kept *inside* the encoded body cannot be read once the
  encoding is the thing that changed.
- **`log`** — the sink is global because the code that logs usually has nothing to reach
  through. First one installed wins.
- **`procgen`** — takes an `Rng` rather than seeding itself, so reproducibility is decided
  outside.
- **`serialization`** — postcard, chosen once. Don't call `postcard::` directly; when the
  backend changes this should be the only edit.
- **`events`** — a `VecDeque<Box<dyn Any + Send>>`. No registration and no dispatch table:
  adding an event type is defining a struct, and nothing enforces that anyone handles it.

### `testing`

The bits of a harness every other library ends up rewriting. It exists because the crate has
no dev-dependencies and a library's own `tests.rs` cannot reach the game's integration
harness — which had already led to two `free_port` implementations with different port ranges.

## Conventions

- One folder per library, always. `mod.rs` re-exports, `tests.rs` beside it holds
  `#![cfg(test)] mod tests { .. }`.
- Tests never need a graphics context or a window, so the whole suite runs on headless CI.
- Errors are `anyhow::Result`. `unwrap`, `expect`, `panic` and indexing are warned on
  crate-wide; a local `#[allow(..)]` with a reason is the convention when one is genuinely
  right.

## Adding one

Ask what it *refuses* to do before what it does — if the `# Not in scope` section is hard to
write, the boundary is probably in the wrong place. `docs/LIBRARIES.md` has the survey this
directory was built from, including the two candidates that were deliberately not extracted
and why.
