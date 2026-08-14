# Extracting libraries

A proposal, not a record of work done. Written from a full read of the tree at `1d3004c7`.

## Why

Two separate goals, and they pull in the same direction:

1. **Less game code.** ~16.3k lines of game (`shared` 4.1k, `server` 4.5k, `client` 7.8k)
   against ~5.2k of libraries. A good chunk of that 16.3k is machinery that has nothing to
   do with Terraria: bounds-checked grids, id registries, a threaded socket, a lua host, five
   hand-rolled clocks.
2. **A boundary an agent cannot wander across.** A library with a written contract and no
   access to game types is a thing you can work inside without reading the game — and, more
   importantly, a thing you *cannot accidentally couple to the game*. That is the actual
   defence against a monolith: not smaller files, but fewer edges between them.

The second goal is the one that decides the design. A library that is merely "code moved to a
different folder" buys nothing. A library that **states what it is for and refuses everything
else** buys a boundary.

## The rule

Something belongs in `libraries/` when all four hold:

- Its public API names no game concept. No `Block`, `Player`, `World`, `Chunk`-as-in-terrain.
  `Grid<T>`, `Registry<T>`, `Transport`, `FixedStep` — yes.
- You could plausibly use it in a different game, or in something that is not a game.
- It has a one-paragraph contract at the top of its `mod.rs` saying what it does **and what it
  deliberately does not do**.
- It compiles without `crate::shared`, `crate::client` or `crate::server`.

That last one is true of every library today (checked: zero such imports outside test files).
It is the invariant worth defending mechanically — see *Enforcement*.

## What is there now

| Library | Lines | Assessment |
|---|---|---|
| `graphics` | 5,186 (+3,471 tests) | Genuinely general, well documented. Two libraries wearing one coat — see G below. |
| `events` | ~50 | Fine as is. |
| `serialization` | ~30 | Fine as is. Perfect example of the pattern: one decision, one place. |

---

## Tier 1 — the big four

These are where most of the value is. Roughly 1,800–2,200 lines of game code become ~1,400
lines of library, and the duplication between the copies disappears.

### A. `libraries/grid` — bounded 2D grids

> A dense, bounds-checked 2D grid of `T`: coordinate translation, chunk coordinates,
> serialization, and iteration. It knows nothing about what a cell means.
>
> **Not** in scope: what to do when a cell changes, chunk *loading*, meshes, events.

**What it replaces.** `shared/world_map/` plus four hand-rolled grids that each reimplement
the same five methods against it:

| | grid | `create` | `create_from_2d_vec` | `serialize` | bounds handling |
|---|---|---|---|---|---|
| `Blocks` | `Vec<BlockId>` | fills with air | yes | `snap(postcard)` | `#[allow(indexing_slicing)]` + a comment justifying it |
| `Walls` | `Vec<WallId>` | fills with **undefined** | yes | `snap(postcard)` | `.get().ok_or_else(anyhow!)` |
| `Liquids` | `Vec<Liquid>` | fills with empty | no | `snap(postcard)` | `.get().ok_or_else(anyhow!)` |
| `Lights` | `Vec<Light>` + `Vec<LightChunk>` | fills with default | no | not saved | `.get().ok_or_else(anyhow!)` |

Four answers to the same question, one of which (`Walls::create` filling with `undefined`) is
documented in CLAUDE.md as a trap you must not step in. A `Grid<T>` constructed as
`Grid::filled(size, value)` cannot have that trap: the fill value is an argument, so there is
no "forgot to overwrite it" state.

25 call sites of `translate_coords` / `translate_chunk_coords` collapse into indexing on the
grid. The two disagreeing coordinate schemes (`x * height + y` for cells, `x + y * width` for
chunks — CLAUDE.md warns not to "fix" either) become two named, documented methods of one
type instead of a landmine sitting in a game module.

**Deletes:** ~300–400 lines. **Effort:** medium. **Risk:** low — mechanical, and the four
users all have tests.

**Take with it:** `ChunkTracker` (client/game, 55 lines, already fully general — a
least-recently-touched index evictor over `BTreeSet<(time, index)>`).

### B. `libraries/registry` — id-indexed type registries

> Register a value, get a typed id back. Look up by id, look up by name, iterate all ids.
> Ids are dense, stable for the process, and never reused.
>
> **Not** in scope: persistence of the registry (the game saves *grids*, not registries, and
> reconstructs ids from mods on load — that stays a game decision).

**What it replaces.** Twenty near-identical functions across `shared/`:
`register_new_block_type` / `get_block_type` / `get_block_id_by_name` / `get_all_block_ids`,
and the same four again for walls, items, liquids and tools. They have already drifted:
`Blocks::get_block_type` returns `&Block`, `Walls::get_wall_type` returns a **clone** of
`Wall` — same intent, different cost, for no reason anyone chose.

`Settings::register_setting` is the same shape a third time (a counter that never reuses a
number, handing out handles), and CLAUDE.md records the bug that came from treating one of
those handles as a row index. One `Registry` with a documented "an id is a handle, not a
position" makes that a property of the type.

**Deletes:** ~200 lines. **Effort:** low-medium. **Risk:** low.

### C. `libraries/net` — typed message transport

> A TCP transport that carries Rust values. Each message is identified by a hash of its type,
> so any `Serialize` type is a message with no registry and no hand-written ids. Runs its
> socket on a background thread and communicates by channel; surfaces connect, disconnect and
> message as events; reports a bind failure through the owner's `update`.
>
> **Not** in scope: the handshake. Who is allowed to connect, what is sent first, and what
> counts as "welcomed" are policy, and policy is the game's.

**What it replaces.** `shared/packet/packet.rs` (50) + `server/server_core/networking.rs`
(387) + `client/game/networking.rs` (328) = ~765 lines, of which the two sides duplicate:

- the thread spawn with an mpsc pair in each direction,
- the `Arc<AtomicBool>` running flag and the 1 ms signal timer that watches it,
- the `SendStatus` retry loop (`ResourceNotAvailable` → sleep 1 ms → retry),
- deserialize-or-drop on a malformed frame,
- `is_finished()` → `join()` → surface the thread's `Result` from `update()`.

Every one of those has a hard-won comment attached to it in the current code. Written once,
they are one documented mechanism instead of two that must be kept in step by hand.

**The handshake stays out.** `VersionPacket` → `NamePacket` → `WelcomeCompletePacket`, the
welcome-phase buffering into `pre_events`, and "the client watches its own server's alive
flag" are all game policy. The library should expose the hooks they need — a per-connection
pre-accept callback, and a "connection is in phase X" state the owner drives — and nothing more.

**Deletes:** ~250 lines net. **Effort:** high; this is the one with real risk. **Risk:**
medium-high — it is the subsystem with the most documented near-misses (the late bind error,
the cancellable welcome, the orphaned thread holding the port). Mitigated by
`integration_tests/networking.rs` (415 lines) already exercising both halves over a loopback
socket. **Do this one third, not first**, once the pattern of extraction is established.

### D. `libraries/scripting` — sandboxed script modules

> Hosts a set of script modules, each with its own interpreter state, source, and named binary
> resources. Registers host functions under a configurable prefix, calls lifecycle hooks by
> name, enumerates a module's global symbols, and defines the on-disk package format
> (`snap(postcard(ModuleData))`).
>
> **Not** in scope: which functions are registered, what the hooks mean, or what a symbol
> named `command_foo` implies.

**What it replaces.** `shared/mod_manager.rs` (241) + `shared/mod_data.rs` (24) are already
*exactly* this — the only game-specific thing in them is the string `"terralistic_"`, which
becomes a constructor parameter. `build_project/compile_mod.rs` (175) writes the package
format and is likewise general once the lua-concatenation policy is a parameter.

The payoff is bigger than the line count: `mod_data.rs` exists as a dependency-free leaf module
solely so the build script can write `.mod` files without dragging rlua into
`[build-dependencies]`. As a separate library with the lua host behind a feature flag, that
constraint is expressed by the dependency graph instead of by a comment asking you not to
break it.

`CommandManager::init` scans symbols for a `command_` prefix — the *scan* is a library
operation (`module.symbols_with_prefix("command_")`), the meaning is the game's.

**Deletes:** ~40 lines (this one is a move, not a dedup). **Effort:** low. **Risk:** low.

---

## Tier 2 — worth doing, smaller

### E. `libraries/timing` — clocks, budgets, limiters

> Fixed-timestep accumulators, frame limiters, per-frame work budgets, rate limiters, and
> frame-time statistics. Everything in here answers one of three questions: *is it time yet*,
> *how long should I sleep*, *have I spent my budget*.

**Five hand-rolled implementations today**, no two alike:

| Where | What | Known failure it already survived |
|---|---|---|
| `FramerateMeasurer::has_5ms_passed` | fixed step | — |
| `Server::advance_timers` + `ms_counter` | fixed step | first-update skip, two guards for one condition |
| `FrameLimiter` (graphics/renderer.rs) | averaged limiter | `f32` ledger stalled after ~4 h; debt now capped at one frame |
| `AnimationTimer` (graphics) | catch-up ticker | `i32` ms overflowed at 24.8 days; 3.6M catch-up frames on the pause menu |
| `Liquids::elapsed_ms` / `next_flow` | per-type rate limit | — |
| `frame_timer.elapsed() < 10ms` in `walls.rs`, `lights.rs` | work budget | inlined literal, duplicated; `blocks.rs` line 235 has it **commented out** |

Those overflow and catch-up bugs are the same bug twice, in two files, found months apart.
A `FixedStep` / `FrameLimiter` / `Budget` / `RateLimiter` set gets one 64-bit, catch-up-capped
answer that everything shares. The budget in particular deserves a type: right now it is a
bare `&std::time::Instant` threaded through four render functions with a magic `10` at the
end of it, which is exactly the kind of thing that gets silently dropped (and has been).

**Deletes:** ~150 lines. **Effort:** low. **Risk:** low. **Do this one first** — it is the
cheapest, it touches five subsystems, and it demonstrates the pattern.

### F. `libraries/config` — registered settings with persistence

> Settings registered at runtime, each with a stable handle and a config key; typed as toggle,
> choice or slider; loaded from and saved to a flat key→value file.

`client/settings.rs` (144 lines) contains nothing game-specific already — it is a general
settings store that happens to live under `client/`. Move it verbatim. `GlobalSettings`
(*which* settings exist, and what they do to the graphics context) stays in the client, which
is exactly the right split: the library holds the mechanism, the game holds the list.

`server_ui`'s `ui_config.json` load/save can then stop hand-rolling its own.

**Deletes:** ~30 lines. **Effort:** trivial. **Risk:** none.

### G. Split `libraries/graphics` into `graphics` + `ui`

Not an extraction from the game — a split of the biggest library into the two things it
actually is:

- **`graphics`**: device, draw list, wgpu backend, surface, texture, atlas, font rasterisation,
  window, events. "Record draw commands; execute them on a GPU."
- **`ui`**: `UiElement`/`BaseUiElement`, `Container`, `UiContext`, theme, `Button`, `Toggle`,
  `TextInput`, `Scrollable`, `RenderRect`, `Sprite`, `ClickTracker`, `AnimationTimer`.
  "An immediate-mode-ish widget toolkit that draws through a `DrawTarget`."

The seam is already there — `UiContext` is precisely the non-rendering surface of
`GraphicsContext`, and `HeadlessContext` already proves the widgets do not need the renderer.
Today they share one 2,378-line `tests.rs` next to a 1,187-line `wgpu_backend.rs`, and an
agent asked to touch a button has both in scope.

**Two things move *into* `ui` from the game:**

- `client/menus/menu.rs` + `menu_stack.rs` (~110 lines) — a stack of screens where the top one
  gets events, can push a successor, and can ask to be popped. Zero game knowledge.
- `server/server_ui/ui_module_manager.rs`'s `ModuleTree` (~545 lines) — a binary split-pane
  layout with a resize/rename edit mode, serialized to JSON. This is a docking layout. Nothing
  in it knows what a server is. It is the single largest piece of general-purpose code
  currently sitting in a game directory.

**Effort:** medium (mostly import churn and splitting the test file). **Risk:** low.

### H. `libraries/container_file` — versioned section containers

> A binary file that is a magic string, a format version, and then a set of named sections,
> each optionally compressed. The header is read and checked before the body is handed to any
> decoder.

`Server::save_world` / `load_world` / `world_save_header` / `read_world_header` plus the four
`snap(postcard(...))` pairs that each subsystem hand-rolls in its own `serialize`. The
header-outside-the-serializer rule is documented in CLAUDE.md as the fix for two real
migrations that produced "Hit the end of buffer" instead of "your world is old" — that rule
is general, and it is currently expressed as prose plus one call site.

Section names (`blocks`, `walls`, `liquids`, `players`) and the version number stay in
`shared/versions.rs`, which is right.

**Deletes:** ~80 lines. **Effort:** low. **Risk:** low, with `integration_tests/world_persistence.rs` covering it.

---

## Tier 3 — smaller, or needs judgement

### I. `libraries/procgen` — noise and sampling helpers

`turbulence` (fractal Perlin) + `convolve` (1D box filter) + the weighted random walk over the
biome adjacency graph. ~100 lines of pure maths currently inside
`server/server_core/world_generator/`. Extracting it makes it testable in isolation, which
matters because generation determinism is a property the project already cares about. The
biome *content* and the generation *order* stay where they are.

**Risk:** low. **Value:** modest but cheap.

### J. `libraries/scheduler` — the wake-your-neighbours update set

> A set of scheduled cells with deterministic iteration order: a change schedules itself and
> its neighbours, a settled cell drops out and costs nothing.

Two users today, both with load-bearing subtleties:

- `Liquids::scheduled` — a `BTreeSet<(i32,i32)>`, *deliberately* not a `HashSet`, because
  iteration order decides how a stream splits and Rust randomises hash order per process.
- `Lights` — a per-cell `scheduled_light_update` flag plus a per-chunk
  `scheduled_light_update_count`, which is the same idea with a different (and cheaper for
  dense updates) representation.

Worth extracting **only as a scheduler** — the flow rules and the light falloff stay in the
game. Two implementations of a hard-to-get-right idea, one of which has a documented
determinism requirement, is a good case for a library; but it is also the proposal most at
risk of becoming an abstraction that fits neither user well. Do it last, and only if the
`Grid` extraction (A) makes the shapes line up.

### K. `libraries/aabb` — swept AABB against a solid grid

`collides_with_blocks` / `is_touching_ground` / the step-loop integrator in
`shared/entities/entities.rs`. The sweep is general if "is this cell solid" is a closure. The
coefficients (`FRICTION_COEFFICIENT`, `BUOYANCY_COEFFICIENT`, `DEFAULT_GRAVITY`,
`LIQUID_RESISTANCE_COEFFICIENT`, the `/ 200.0` tick divisors) are game tuning and must stay in
`shared/`.

**Flagging a caveat:** physics is the one thing that *must* agree bit-for-bit between client
and server, and it currently does because both call `shared/`. Extracting the sweep keeps that
(both would call the library), but it adds a boundary to a correctness-critical path for a
fairly small line-count win. **My recommendation: skip this one**, or do it only after
everything else has proven the pattern.

### L. `libraries/log` — leveled console output

`print_to_console` (timestamp, `[INFO]`/`[WARNING]`/`[ERROR]`, newline splitting) plus
`UI_EVENT_SENDER`, the process-global `Mutex<Option<Sender>>` that exists because
`print_to_console` is a free function with no server to reach through. A small library with a
`LogSink` registered once turns a documented hack into a named mechanism — and gives the
"first non-`None` sender wins" rule somewhere to be written down other than a comment on a
static.

**Effort:** trivial. **Value:** small, but it removes a global.

---

## What must never move

Stated explicitly so that a future pass does not have to re-derive it:

- Block, wall, liquid, item and tool **semantics** — breaking rules, drops, recipes, inventory
  rules, connected-texture rules, multiblock growth.
- The **handshake** and every packet type. `WelcomeCompletePacket` is not a transport concept.
- **World generation policy** — the biome graph's meaning, the order of the six generation
  steps, `FLOODED_COLUMN_FRACTION`, the fill-from-the-sky rule.
- **Physics constants** and the 5 ms tick's meaning.
- Anything whose name has to mention a block, a player, a world or a mod to make sense.

A useful test when unsure: *could this library's doc comment be written without naming
anything from the game?* If not, it is game logic wearing a library's clothes.

## Enforcement

The rule "no game imports in a library" is currently satisfied and currently unenforced. Two
options:

**Cheap (do this now):** a `#[test]` that walks `libraries/` and fails on any line matching
`crate::(shared|client|server)`. Ten lines, runs in CI with everything else, catches the
regression on the commit that introduces it.

**Strong (consider later):** make each library a real crate in a cargo workspace. Then the
boundary is enforced by the compiler and cannot be argued with. Costs, all real:

- The ~136 crate-wide clippy lints in `main.rs` would need to be per-crate — solvable cleanly
  with `[workspace.lints]` in the root `Cargo.toml`, which is the right home for them anyway.
- `#[cfg(test)]` helpers do not cross crate boundaries. `HeadlessContext`, `Font::new_headless`,
  `Texture::new_sized`, `TextInput::new_headless` and the `#[cfg(test)]` accessors on `Server`
  would each need a `testing` feature instead. That is a genuine cost and touches a lot of tests.
- The golden-image harness dispatches through `main.rs` behind the `render-tests` feature;
  it would need to stay reachable from the binary crate.

My read: do the cheap check now, and revisit the workspace once three or four libraries exist
and the boundaries have stopped moving. Splitting crates around a boundary you are still
adjusting is how you end up with a workspace full of `pub` escape hatches.

## Sequencing

Ordered by (value ÷ risk), not by size:

1. **E `timing`** — cheapest, touches five subsystems, proves the pattern. Fixes two classes of
   clock bug by construction.
2. **F `config`** — a pure move, ~an hour.
3. **B `registry`** — mechanical, well tested, removes 20 near-duplicate functions.
4. **A `grid`** (+ `ChunkTracker`) — the biggest dedup win in `shared/`. Do it after B, because
   the four grid owners are also the four registry owners and you want to touch them once.
5. **H `container_file`** — small, and it follows naturally from A.
6. **D `scripting`** — a move; do it before C so the `.mod` format is settled.
7. **G `graphics`/`ui` split** (+ `MenuStack`, + `ModuleTree`) — biggest reduction in what an
   agent has in scope at once.
8. **C `net`** — highest risk, do it when the pattern is boring. Lean on
   `integration_tests/networking.rs`.
9. **I `procgen`**, **L `log`** — opportunistic, any time.
10. **J `scheduler`** — only if A makes the two users line up.
11. **K `aabb`** — recommended *not* to do.

### Estimated end state

Rough, and the deletion figures are the least certain part:

| | now | after |
|---|---|---|
| `libraries/` code | 5,186 | ~8,500 |
| game code (`shared`+`server`+`client`) | 16,271 | ~12,000 |
| net lines | 21,457 | ~20,500 |

The line count barely moves. That is expected and is not the point: what changes is that
~3,500 lines stop being game code that an agent must hold in its head alongside the game, and
become library code behind a stated contract that it can use without reading. The ~1,000 lines
that genuinely disappear are duplication between the four grid owners, the four registries,
and the two networking halves — and duplication is where the two copies drift.
