# Improvement catalogue

Findings from a full read of the repository at commit `7e3d0505` (branch `Claude-testing`).
Baseline at time of writing: `cargo build` clean, `cargo test` 22/22 passing,
`cargo clippy --all-targets` 107 warnings on the binary.

Each item says whether it was **confirmed** (reproduced or verified by reading the exact
code path) or **suspected** (looks wrong, not proven). Nothing here has been fixed yet.

---

## 1. Correctness — worth fixing first

### 1.1 Any player can crash the server by typing `/` in chat — CONFIRMED

`server/server_core/commands.rs:88`

```rust
let mut arguments = command.split_whitespace().map(...).collect::<Vec<_>>();
let command_name = arguments.remove(0);   // panics when arguments is empty
```

Path: client sends `ChatPacket { message: "/" }` → `on_event` sees the leading `/` →
strips it to `""` → `execute_command("")` → `split_whitespace()` yields zero tokens →
`Vec::remove(0)` panics with *removal index (is 0) should be < len (is 0)*.

The stock client's only guard is `!text.is_empty()` (`client/game/chat.rs:132`), and `"/"`
is not empty, so **no modified client is needed**. Also reachable from the server GUI
console, which passes its input to `execute_command` with no slash-stripping at all
(`core_server.rs:handle_events`).

Fix: return early on an empty token list.

```rust
let Some(command_name) = arguments.first().cloned() else {
    return Ok(String::new());
};
arguments.remove(0);
```

Note this is exactly the class of bug the crate-wide `#![warn(clippy::panic)]` and
`#![warn(clippy::indexing_slicing)]` lints are meant to catch — clippy can't see
`Vec::remove`, so the lint list gives false confidence here. Worth a test.

### 1.2 The server only listens on loopback, so multiplayer cannot work — CONFIRMED

`server/server_core/networking.rs:99`

```rust
let listen_addr = format!("127.0.0.1:{server_port}");
```

Binding to `127.0.0.1` accepts connections only from the same machine. Meanwhile the client
ships a complete "add server by IP" flow (`client/menus/add_server_menu.rs`,
`multiplayer_selector.rs`) that parses arbitrary `host:port` — so the UI advertises
something the server cannot serve.

Fix is `0.0.0.0:{server_port}`, but that is a deliberate exposure decision, not a typo
cleanup: it opens the port to the network. Given there is no authentication on the game
protocol and item 1.1 exists, bind address and a basic auth story should be decided together.
Singleplayer is unaffected (it connects to `127.0.0.1` on port 49152).

### 1.3 Unbreakable blocks can be broken client-side — SUSPECTED

`shared/blocks/breaking.rs`

`start_breaking_block` correctly refuses blocks with `break_time == None`. But
`set_break_progress` creates a `BreakingBlock` with no such check, and
`update_breaking_blocks` then compares against `break_time.unwrap_or(1)` — treating an
unbreakable block as breaking in 1 ms.

`set_break_progress` is driven by server packets, so this is not directly player-triggerable,
but the two functions disagree about what `None` means. `get_break_stage` has the same
problem in the other direction: it divides by `break_time.unwrap_or(0)`, so a non-zero
progress on an unbreakable block yields `inf` and casts to `i32::MAX`.

Fix: make `None` mean unbreakable consistently in all three functions.

### 1.4 Player spawn point is the topmost solid block in the column — SUSPECTED

`server/server_core/players.rs:get_spawn_coords`

```rust
// find a spawn point
// iterate from the top of the map to the bottom
for y in (0..blocks.get_size().1).rev() {
    for x in 0..(PLAYER_WIDTH.ceil() as i32) {
        ...
        if !is_ghost { spawn_y = y as f32 - PLAYER_HEIGHT; break; }   // breaks inner loop only
    }
}
```

Three things disagree here. The comment says top-to-bottom; `.rev()` iterates
**bottom-to-top**. The `break` exits only the inner `x` loop, so the outer loop keeps going
and the *last* write wins — meaning the final `spawn_y` comes from the smallest `y` with a
solid block, i.e. the highest solid tile in the column. If a tree, canopy or floating island
sits above the surface, players spawn on top of it. If the column is entirely air,
`spawn_y` stays `0.0`.

It also scans the full world height for every spawn (O(width × height)) when it should stop
at the first hit.

### 1.5 `ChunkTracker` uses `0` as both a timestamp and a sentinel — SUSPECTED

`client/game/chunk_tracker.rs`

`modified_time` uses `0` to mean "not tracked", but `timer.elapsed().as_secs()` legitimately
returns `0` for the whole first second of runtime. A chunk first touched during that second
records time `0`; when it is touched again later, `*get_modified_time(chunk)? != 0` is false,
so the stale `(0, chunk)` entry is never removed from `queue` and a duplicate is inserted.
`get_oldest_chunk` then keeps returning that chunk, so the eviction logic can pick a
recently-used chunk.

Fix: use `Option<u32>` instead of a `0` sentinel.

### 1.6 `Lights::create` chunk-count arithmetic — SUSPECTED (over-allocates, does not crash)

`shared/lights.rs:create`

```rust
(size.0 as i32 / CHUNK_SIZE * size.1 as i32 / CHUNK_SIZE) as usize
```

`*` and `/` are left-associative, so this evaluates as `((w / 16) * h) / 16`, not
`(w / 16) * (h / 16)`. For the default 4400×1200 world both give 20625, which is why nothing
has broken. For a height not divisible by 16 the first form is larger, so the vector is
over-allocated rather than short — safe today, but it is not the expression that was meant.
Parenthesise it.

Related: `init_sky_heights` defaults missing entries to `-1`, while `on_event` reads them
with `.unwrap_or(&0)`. Pick one.

---

## 2. Undefined behaviour and thread-safety

### 2.1 `static mut` in the server core — CONFIRMED (compiler warns)

`server/server_core/core_server.rs:182-185, 347`

Five `static mut` items: four tick counters inside `Server::update`, one `Sender` in
`send_to_ui`. This is the only warning `cargo build` currently emits:

```
warning: creating a shared reference to mutable static
   --> server/server_core/core_server.rs:350:12
   = note: ... it's undefined behavior if the static is mutated ...
   = note: `#[warn(static_mut_refs)]` (part of `#[warn(rust_2024_compatibility)]`)
```

It will become a hard error on edition 2024. Beyond the UB, it means the counters are
process-global: a second `Server` in the same process (a test, or singleplayer alongside a
dedicated server) shares and corrupts them. The comment says the statics exist so outside
code can't mismanage the counters — private struct fields achieve that without UB.

Fix: move `MS_COUNTER` / `SECONDS_COUNTER` / `MS_TIMER` / `LAST_TIME` into `Server` fields,
and make `UI_EVENT_SENDER` an `OnceLock<Sender<UiMessageType>>`.

### 2.2 Two `expect` calls on the server network thread — CONFIRMED

`server/server_core/networking.rs:120, 151`

```rust
let packet: Packet = bincode::deserialize(packet).expect("Failed to deserialize");
...
Self::send_packet_internal(&handler, &packet_data, &conn).expect("Failed to send Packet");
```

A malformed TCP frame from any client kills the networking thread. The client side already
does this properly — it funnels errors into a shared `receive_loop_error` string that the
main loop checks. The server should do the same rather than `expect`.

---

## 3. Build and dependencies

### 3.1 The build script compiles 21 dependencies it does not need — CONFIRMED

`cargo tree -e build --depth 1` lists sdl2, sdl2-sys, gl, rustls, webpki-roots, message-io,
arboard, hecs, rlua, kvptree and more as **build-dependencies**. The build script's actual
needs are tiny:

- `gfx::Surface`, `gfx::Color`, `gfx::IntPos`, `gfx::IntSize` — pure CPU pixel buffers, no OpenGL
- `shared::mod_manager::GameMod` — a struct with a `String` and a `HashMap`
- `png`, `darklua`, `bincode`, `snap`, `serde`, `winres`

The cause is `build_main.rs` re-declaring the whole module tree (`pub mod libraries { pub mod
graphics; } pub mod shared;`), which drags in every transitive dependency of the game.
Everything in that list is compiled **twice** — once for the host build script, once for the
target — and it is a large share of the ~1m40s cold build.

Fix: split the handful of pure-data types (`Surface`, `Color`, `IntPos`, `IntSize`, and the
`GameMod` serialization shape) into a leaf module with no SDL/GL/network dependencies that
both the build script and the game can include. Then trim `[build-dependencies]`.

### 3.2 `message-io` version skew — CONFIRMED

`Cargo.toml` pins `0.19` in `[dependencies]` and `0.18` in `[build-dependencies]`. Two
versions get compiled. The build script does not use `message-io` at all — it can simply be
removed from `[build-dependencies]` (subsumed by 3.1).

Also: several `[build-dependencies]` (`fnv`, `rand`, `kvptree`, `png`) lack the
`default-features = false` that their `[dependencies]` counterparts carefully specify.

### 3.3 Build script panics on every error — MINOR

`build_project/*.rs` is `.unwrap()` throughout and `build_main.rs` opens with
`#![allow(clippy::all)]`. Acceptable for a build script (a panic is a failed build), but the
messages are unhelpful — a missing `resources/` directory reports as a bare `unwrap` panic
with no path. Cheap improvement: `.expect("reading resources/...")` with the path in the message.

---

## 4. CI

`.github/workflows/rust.yml` runs `cargo test` on Ubuntu, and only for `master`.

Concrete gaps:

- **Nothing runs on any other branch**, including this one — `push: branches: ["master"]`.
- No `cargo clippy`, despite ~150 curated lints in `main.rs`. That list is the project's
  main quality mechanism and CI never checks it.
- No `cargo fmt --check`, despite a committed `rustfmt.toml`.
- No release build, so a `--release`-only breakage ships.
- macOS and Windows are never built, though all three have platform-specific `Cargo.toml`
  sections and `#[cfg]` branches.
- `Swatinem/rust-cache@v2.5.0` and `actions/checkout@v3` are both behind.

Recommended minimum: add `cargo clippy --all-targets -- -D warnings` (after clearing the
current 107) and `cargo fmt --check`, and run on pull requests to any branch.

---

## 5. Dead and unfinished code

### 5.1 `shared/liquids/` is never constructed — CONFIRMED

271 lines implementing liquid types, spreading and a `Liquids` manager. Grepping the whole
repo, no file outside `shared/liquids/` references it apart from `pub mod liquids;`. Its own
comments confirm it is unfinished: `//TODO: to_serial, from_serial`, `//TODO: new event sender`,
`//TODO: implement new events`. It also uses raw indexing (`self.liquids[...]`) rather than
the `Result`-returning accessor style used everywhere else.

Decide: finish and wire it in, or move it to a branch. Leaving it gives a false impression
that liquids work.

### 5.2 Four empty test modules — CONFIRMED

`shared/walls/tests.rs`, `shared/items/tests.rs`, `shared/inventory/tests.rs`,
`shared/liquids/tests.rs` each contain exactly `#![cfg(test)] mod tests {}`. They are already
wired into their `mod.rs`, so tests can be added with no plumbing.

`shared/inventory` is the highest-value gap: `give_item` (stack merging, overflow into new
slots, dropping the remainder), `craft` (ingredient deduction) and `swap_with_selected_item`
are pure functions with real branching logic and no coverage at all.

### 5.3 UI elements that predate the `UiElement` trait — MINOR

Marked by `//TODO` comments in `client/game/chat.rs`, `pause_menu.rs`, `debug_menu.rs`,
`inventory.rs`, `respawn_screen.rs`, plus `settings_menu.rs:112`
(`//TODO this is shit, make it centered on the slider`). Each is a self-contained conversion.

---

## 6. Duplication and cleanup

- **`renderer.rs:update_window`** — the `#[cfg(target_os = "windows")]`,
  `#[cfg(target_os = "macos")]` and `#[cfg(target_os = "linux")]` blocks contain
  **byte-identical** `gl::BlitFramebuffer` calls. Collapse to one. The hardcoded `* 2.0`
  in them also assumes a 2× backing scale on every platform, which is wrong for non-Retina
  displays.
- **`main.rs` lint list** — 10 lints are listed twice (clippy reports "duplicated attribute"
  ×10), and two are dead: `clippy::string_to_string` and `clippy::match_on_vec_items` have
  been removed from clippy. Several are both warned and allowed
  (`allow_attributes_without_reason`, `shadow_unrelated`, `todo`, `unimplemented`) — the
  `allow` wins, so the `warn` is noise.
- **`get_block_id_by_name`** exists twice with identical logic: `Blocks::get_block_id_by_name`
  (`shared/blocks/blocks.rs`) and inline inside the Lua binding
  (`shared/blocks/mod_interface.rs`). The binding should call the method.
- **107 clippy warnings**, of which clippy says 82 are auto-fixable
  (`cargo clippy --fix --bin terralistic`). The bulk are `unwrap_used` in tests (fine, but
  should be `#[allow]`ed at the test-module level rather than warned every build),
  `missing_const_for_fn` ×17, `uninlined_format_args` ×12, and `implicit borrow as raw
  pointer` ×21 in the graphics layer.

---

## 7. Protocol and format robustness

These are design-level, listed for awareness rather than as defects.

- **Packet ids are `FNV(TypeId)`**, and `TypeId` is not stable across rustc versions. Client
  and server built with different compilers will silently fail to understand each other with
  no error message — `try_deserialize` just returns `None` and the packet is ignored. There
  is no version handshake. `shared/versions.rs` exists (`VERSION` from `CARGO_PKG_VERSION`)
  but is only used by `main.rs` to print. Sending it at connect time and refusing a mismatch
  would turn a silent failure into a clear one.
- **Renaming a packet struct changes its wire id** with no compile-time signal.
- **Save files have no version field.** `world.bin` is
  `bincode(HashMap<String, Vec<u8>>)` with keys `blocks`/`walls`/`players`; any field added
  to `BlocksData` or `SavedPlayerData` silently breaks existing worlds. A `version` key in
  that top-level map is a cheap fix while the format is still young.

---

## Suggested order

1. **1.1** — trivial fix, remote crash, add a regression test (~15 min)
2. **2.1** — removes the only build warning, unblocks edition 2024 (~1 h)
3. **2.2** — same treatment the client already uses (~30 min)
4. **4** — clippy + fmt in CI, running on all branches (~30 min)
5. **3.1 / 3.2** — meaningful cold-build speedup (~2 h)
6. **1.3–1.6** — gameplay correctness, needs a run to verify each (~2 h)
7. **5.2** — inventory tests, best value per line of the remaining work
8. **1.2** — decide bind address and auth together, not as a one-liner
