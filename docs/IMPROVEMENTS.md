# Improvement catalogue

Findings from a full read of the repository at commit `7e3d0505` (branch `Claude-testing`).
Baseline at time of writing: `cargo build` clean, `cargo test` 22/22 passing,
`cargo clippy --all-targets` 107 warnings on the binary.

Each item says whether it was **confirmed** (reproduced or verified by reading the exact
code path) or **suspected** (looks wrong, not proven).

## Status

Everything catalogued below is done, except item 5.1 which needs a decision from you.
All of it is on `Claude-testing`; items 1.1-1.7, 2.1, 2.2, 4, 5.2 and 6 also went to
`master` via PRs #168-#178.

| Item | State |
|---|---|
| 1.1 Empty command panic | done (#168) |
| 1.2 Server binds loopback | done — `BindAddress` per server, singleplayer stays on loopback |
| 1.3 Unbreakable blocks | done (#175) |
| 1.4 Spawn point | done (#176) — clarity and early exit; not the behaviour bug I first suspected |
| 1.5 `ChunkTracker` sentinel | done (#174) |
| 1.6 `Lights::create` arithmetic | done (#174) |
| 1.7 Headless server never saves on shutdown | done (#173) |
| 2.1 `static mut` | done (#169) |
| 2.2 Network thread `expect` | done (#170) |
| 3.1 Build-dependency bloat | done — 21 build-dependencies down to 8 |
| 3.2 `message-io` skew | done — removed from build-dependencies entirely |
| 3.3 Build script error messages | done — failures now name the path |
| 4 CI gaps | done (#171), plus `-D warnings` once the tree was clean |
| 5.1 Dead `liquids` module | **open — needs your call, see below** |
| 5.2 Empty test stubs | done — walls (#175), inventory (#172), items; liquids left with 5.1 |
| 6 Duplication and cleanup | done (#178), plus all 107 clippy warnings cleared |
| 7 Protocol/format robustness | done — version handshake and world save version |

Found while doing the above, not in the original catalogue:

| Finding | State |
|---|---|
| `base_game.mod` was never reproducible — `HashMap` iteration order meant a committed artifact changed on every build | done — `GameModData.resources` is a `BTreeMap` |
| The blit in `update_window` was skipped entirely on any OS that is not windows/macos/linux | done (#178) |
| `TextInput` panicked when typing over a right-to-left selection | done — see below |
| The client hung forever on the loading screen when the server was down or refused it | done — see 5.5 |
| World generation ignored its seed for the biome layout, so the same seed gave a different world every run | done — see 5.5 |
| `despawn_entity` left every despawned id resolvable, and the id maps only ever grew | done — see 5.5 |

### The `TextInput` selection panic

Found by the headless UI tests, and reachable in the running game: select text with
shift+left, then type. The `Event::TextInput` handler deleted the selected range and then
collapsed the cursor with `self.cursor.1 = self.cursor.0`, which keeps whichever half is
*larger*. After a right-to-left selection that is the end of the range, which no longer
exists once the range is removed, so the following `insert_str` panicked on a non
char-boundary index.

The fix collapses onto the start of the selection, matching what the backspace, delete and
paste handlers already did. `test_typing_replaces_the_selection` is the regression test,
and `test_typing_replaces_a_forwards_selection` pins the other direction.

### The one open decision: 5.1

`shared/liquids/` is **entirely commented out**, which is worse than the original scan
suggested. Both files are a single `/* .. */` block from their first line to their last:
`liquids.rs` is 271 lines and `liquid_type.rs` is 92, and between them they contain zero
lines of compiled code. `shared/liquids/mod.rs` declares three modules that build to
nothing.

So the earlier description, "271 lines that nothing constructs", understated it — it is
not unused code, it is commented-out code that has been carried in the tree.

Options, roughly in order of effort: delete it (recoverable from git history), move it to
a branch, or uncomment and finish it. I have not touched it, because throwing away
unfinished work is your call. `shared/liquids/tests.rs` stays a stub, now with a comment
explaining that there is literally nothing to test.


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

### 1.7 A headless server never saves its world on shutdown — CONFIRMED

`main.rs:server_main`, `server/server_core/core_server.rs:run`, `server/server_ui/ui_manager.rs:should_ui_stop`

`Server::run` leaves its loop only when `is_running` goes false or a mod sets the state to
`Stopping`, and only then calls `stop()` → `save_world()`. In `nogui` mode **nothing ever
does either**: `server_running` is cleared in exactly one place, `UiManager::should_ui_stop`,
when the GUI window closes. There is no signal handling anywhere in the tree — grepping for
`ctrlc|signal_hook|SIGINT|SIGTERM|set_handler` returns nothing.

So Ctrl-C or SIGTERM kills the process before the save runs, and every shutdown of a
dedicated server loses all world progress since it started.

Confirmed by running the same binary two ways. On master, SIGTERM makes the process exit
instantly with no "saving world" line and no `server_data/` directory. With a `ctrlc`
handler that clears `server_running`, the same signal produces the full
`saving world → server stopped.` sequence and a 1.1 MB `server.world`.

The shutdown logic itself was already correct — it was simply unreachable.

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

### 3.1 The build script compiles 21 dependencies it does not need — DONE

`cargo tree -e build --depth 1` used to list sdl2, sdl2-sys, gl, rustls, webpki-roots,
message-io, arboard, hecs, rlua, kvptree and more as **build-dependencies**. The build
script's actual needs are tiny:

- `gfx::Surface`, `gfx::Color`, `gfx::IntPos`, `gfx::IntSize` — pure CPU pixel buffers, no GPU
- `shared::mod_manager::GameMod` — a struct with a `String` and a `HashMap`
- `png`, `darklua`, `bincode`, `snap`, `serde`, `winres`

The cause was `build_main.rs` re-declaring the whole module tree (`pub mod libraries { pub mod
graphics; } pub mod shared;`), which drags in every transitive dependency of the game.
Everything in that list was compiled **twice** — once for the host build script, once for the
target — and it was a large share of the ~1m40s cold build.

Fixed by having `build_main.rs` name the individual leaf files it needs
(`libraries/graphics/{color,position,surface}.rs`, `shared/mod_data.rs`) instead of
re-declaring `pub mod graphics;` / `pub mod shared;`. `[build-dependencies]` is now anyhow,
bincode, darklua, png, serde, serde_derive, snap and winres, and nothing else.

### 3.2 `message-io` version skew — DONE

`Cargo.toml` pinned `0.19` in `[dependencies]` and `0.18` in `[build-dependencies]`, so two
versions were compiled. The build script never used it. Subsumed by 3.1: it, `fnv`, `rand`
and `kvptree` are all gone from `[build-dependencies]`, and what remains carries
`default-features = false` where it matters.

### 3.3 Build script panics on every error — MINOR

`build_project/*.rs` is `.unwrap()` throughout and `build_main.rs` opens with
`#![allow(clippy::all)]`. Acceptable for a build script (a panic is a failed build), but the
messages are unhelpful — a missing `resources/` directory reports as a bare `unwrap` panic
with no path. Cheap improvement: `.expect("reading resources/...")` with the path in the message.

---

## 4. CI

`.github/workflows/rust.yml` runs `cargo test` on Ubuntu, and only for `master`.

Concrete gaps:

- **No pushes outside `master` are built.** Note the `pull_request` filter is on the *base*
  branch, so pull requests targeting `master` do get CI — an earlier draft of this document
  overstated this as "nothing runs on any other branch". The real gap is branch pushes with
  no PR open, and PRs targeting anything other than `master`.
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

There is now a second reason to do these: anything implementing `UiElement` gets its layout
and event handling tested headlessly for free (see 5.4), while a hand-rolled element does
not.

### 5.4 UI logic could not be tested at all — DONE

`get_container` and `on_event_inner` took a `&GraphicsContext`, which cannot be constructed
without a window and an OpenGL context, so none of the layout or input handling was
reachable from a test. They now take a `&dyn UiContext` — window size, mouse, keys,
clipboard and nothing else — which `GraphicsContext` and the test-only `HeadlessContext`
both implement.

That brought layout, hit testing, event routing, `Button`, `Toggle`, `Scrollable`,
`TextInput` and the font's text measurement under test, and immediately turned up the
`TextInput` selection panic above.

Not covered, and the honest limits of the approach:

- `render_inner` / `update_inner` still need a real GPU device, so what a widget *draws* is
  covered by the golden-image suite rather than by `cargo test`.
- The three menus that build GPU resources from an event handler take the
  `as_graphics_context()` escape hatch, and those branches are skipped headlessly.
- The `HeadlessContext` is a stand-in. It answers the same questions as the real context,
  but it is not proof that the window reports what we think it does.

### 5.5 Nothing tested more than one subsystem at a time — DONE

Every test was a unit test beside the module it covered, which left the seams between
modules — the ones with no compile-time check on them — completely unexercised. The
networking modules had no test at all, world generation had none, and the `.mod` artifact
was only ever exercised by running the game.

`integration_tests/` now drives the real pieces against each other: a real
`ClientNetworking` against a real `ServerNetworking` over a loopback socket, a real
`Server` on a temporary world, and the committed `base_game.mod` through lua into the rust
registries. 56 tests across five files, in about two seconds.

They found four bugs, all fixed and covered:

| Bug | Effect |
|---|---|
| The client ignored `Connected(_, false)` while welcoming | Joining a server that is down hung on the loading screen forever |
| The client ignored `Disconnected` while welcoming | Same, when the server *refused* the client — which is exactly what the version handshake was added to report |
| `generate_biome_ids` drew from `rand::random` | The world seed did not control the biome layout or the world's width, so the same seed built a different world every time |
| `despawn_entity` never cleared the id maps | Despawned ids stayed resolvable, and the two maps grew for the life of the server |

Known limits, again honestly:

- World generation is still not reproducible from a seed end to end. The rust half now is,
  but each biome's lua `generator_function` decorates with `math.random`, which is seeded
  per lua state. Making decoration reproducible means seeding lua from the world seed,
  which is a change to what mods can rely on — a decision, not a fix. The determinism test
  compares walls, which lua never touches, and says so.
- Rendering is still untested, for the reasons in 5.4.
- `ClientNetworking::stop` deadlocks if called while the client is still welcoming: the
  networking thread only checks the running flag from the normal loop. Nothing in the game
  does this — `run_game` only stops a client it has already waited for — so it is a trap
  rather than a live bug, and the harness documents it instead of working around it.

---

## 6. Duplication and cleanup

- ~~**`renderer.rs:update_window`** — three byte-identical per-platform
  `gl::BlitFramebuffer` blocks with a hardcoded 2× backing scale.~~ Gone with the wgpu port:
  presenting is one nearest-sampled quad at the surface's real drawable size.
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
