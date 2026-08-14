# Improvement catalogue

Findings from a full read of the repository at commit `7e3d0505` (branch `Claude-testing`).
Baseline at time of writing: `cargo build` clean, `cargo test` 22/22 passing,
`cargo clippy --all-targets` 107 warnings on the binary.

Each item says whether it was **confirmed** (reproduced or verified by reading the exact
code path) or **suspected** (looks wrong, not proven).

## Status

Everything catalogued below is done. All of it is on `Claude-testing`; items 1.1-1.7, 2.1,
2.2, 4, 5.2 and 6 also went to `master` via PRs #168-#178.

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
| 5.1 Dead `liquids` module | done — finished and wired in, see below |
| 5.2 Empty test stubs | done — walls (#175), inventory (#172), items; liquids with 5.1 |
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
| Joining a world that had just been generated froze the window for tens of seconds with no way out | done — see below |
| A large blur region in a fullscreen window cost 7.3 ms of GPU a frame, dropping the game to 45 fps | done — see below |

### Joining a newly generated world looked like a hang

Reported as "sometimes when I generate a new world it just hangs forever". Three separate
things, all of them only visible on the *generation* path — loading an existing world takes
none of them.

**The event flood.** After generating, `Server::start` ran `update_block` on every cell in the
world, each pushing a `BlockUpdateEvent`. On the default 4514x1200 world that is 5.4 million
events queued before the first `update()`, which then offered each to every subsystem and handed
it to lua's `on_block_update`. Measured on the default world: the first update took **17.7s** in
a debug build (4.4s optimised) and peak RSS was **1.28 GB**. Nothing consumes `BlockUpdateEvent`
except that lua hook, and the sweep's actual job is growing multiblocks — a tree's canopy is 5x5
and the generator places one cell of it. `ServerBlocks::expand_big_blocks` now walks the
multiblocks only: **150ms** and 890 MB, with the canopies identical.

**Nothing rendered during the join.** `run_game` waited for the handshake in a `sleep(1ms)` loop
and then did every bit of setup without drawing a frame, all inside the menu loop's
`open_menu` — so the window stopped repainting the moment the server's loading screen closed and
stayed that way for the whole join. It also had a dead `Arc<Mutex<String>>` of loading text that
was never shown to anything. `JoinScreen` draws the menus' own `LoadingScreen` once per phase,
each phase named.

**The close button did nothing.** The join loop never looked at the window, and the networking
thread's welcome loop never looked at `is_running`, so there was no way to abandon a join and
`stop()` would have blocked forever if anyone had tried. The welcome loop now runs a 1ms signal
timer, `stop()` is safe at any point, and closing the window during a join returns.

**The orphaned networking thread.** This is the one that hangs *for good* rather than for
seconds, and the console line that identified it was
`Failed to send NewConnectionEvent: sending on a closed channel`. `Server::run` returned early
through `?` whenever `start` or `update` failed, which skips `stop` — and `start` binds the port
long before anything that can fail. The `Server` was then dropped with the networking thread
still running, so that thread kept the port and went on accepting clients whose
`NewConnectionEvent` had nowhere to go. A client that connected to one was accepted and never
welcomed: it waited forever, showing the last frame it drew, which is the loading screen. The
port also stayed taken for the rest of the process, so every world opened afterwards failed to
bind and hung the same way — which is how one failure turns into a session where nothing works.
`run` now wraps `run_until_stopped` and stops networking however it exits.

Two smaller guards on the same path, since a server can always fail for some new reason:
the client gives up on a singleplayer server whose flag has been cleared instead of waiting for
a welcome that is not coming, and a failed world now ends on a `ChoiceMenu` naming the error
rather than a `println!` and a silent return to the menu.

Regression tests: `test_generation_does_not_queue_an_event_per_block`,
`test_generation_grows_the_tree_canopies`,
`test_a_client_can_be_stopped_while_it_is_still_welcoming`,
`test_a_server_that_fails_to_start_releases_its_port`.

Still unknown: what made the server fail in the first place on that run. Every path from
"the server failed" to "the game hangs" is closed now, and all three of them report what
happened, so a recurrence should say so rather than sit there.

Not reproduced: the report also mentioned the server saying something about a wrong packet. The
only such message is `[peer] sent a packet that could not be deserialized, ignoring it`, which
drops one frame and keeps serving, and nothing here produced it — the handshake, including
multi-megabyte welcome packets over 64 KB socket reads, was checked packet by packet against a
real generated world. The version handshake's refusals (`[peer] refused: it is version …`) read
similarly and would explain a *failed* join rather than a slow one.

Found while testing this, unrelated to it: `test_a_player_is_remembered_across_a_restart`
sampled the player's position before the disconnect and compared it against the position the
server saved when it noticed, with a block of slack for the fall in between. On a loaded machine
that is not enough slack, and the test failed at random. It keeps the last position it saw
instead, so there is no race to be slack about.

### A big blur in a fullscreen window cost more than the rest of the frame

Reported as "if the blur region is big and the window is fullscreened, fps drops to 45".

The blur was the one effect in the toolkit priced by the *area* it covers, and it paid that
price four to six times over: `plan_blur` emitted a thirteen tap gaussian pass per axis, twice,
plus a two pixel pair once the radius was large enough, each of them a full render pass over
every pixel of the region at the offscreen's resolution — which is the display's real one. A
menu blurring most of a 3340x2100 fullscreen window is seven million pixels through half a
billion samples, and on a tiled GPU each of those passes also loads and stores the whole frame
texture because that is what it was attached to.

Measured with a hidden 3340x2100 context, timing frames with and against the same frame drawn
without the blur: **7.3 ms of GPU for the blur alone**, against a 16.7 ms budget at 60 fps.

The blur now runs on a shrunken copy. The first pass reads the region out of the frame into a
scratch texture at a quarter to a half of the frame's resolution, the rest of the gaussian
passes ping-pong on the scratch pair, and an ordinary draw stretches the result back over the
region. **The tap spacings are untouched** — still the same distances in window coordinates — so
the output is the same blur sampled on a coarser grid, and a blur holds nothing finer than its
own kernel. Against vertical one pixel stripes, the worst content available, no channel moved
more than 6 of 255 from the full resolution result, and `render_rect_blur` passes its committed
golden unchanged. The same measurement now puts the blur below the half millisecond the harness
can resolve.

Three details are worth keeping in mind, and are written up in `wgpu_backend.rs`: the downscale
follows the radius, because the round trip is itself a blur about a texel wide and a fade's
first frames ask for a radius of one or two pixels; the upsample is a `Segment::Draw` rather
than a pass of its own, so it joins the draw pass that follows the blur instead of costing
another full-frame attachment; and it samples linearly, through a bind group layout of its own,
so the toolkit's shared layout can go on declaring itself non-filterable.

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

### 5.1, resolved: liquids are back

The decision was to finish it. `shared/liquids/` is now compiled code with a `tests.rs` that
has something to test, and liquids exist in all three layers the way every other subsystem
does.

What is there:

- **`shared/liquids/`** — `Liquid { id, level }` cells on a `WorldMap`, with levels as whole
  numbers rather than the original `f32`. That is not a style preference: the old code
  compared `level as i32`, so cells that were never quite equal averaged each other forever
  and never settled. Flow is down first, then averaging with lower horizontal neighbours,
  driven off a scheduled set rather than a scan of the world.
- **`server/server_core/liquids.rs`** — the authority. Sends the grid on join, batches a
  flow step's changes into one `LiquidChangesPacket`, and reschedules liquid when a block
  changes next to it.
- **`client/game/liquids.rs`** — applies those packets and draws partly filled cells as a
  surface, in per-chunk meshes like walls.
- **Persistence** — a `liquids` key in the world save, and `WORLD_SAVE_VERSION` bumped to 4.
- **Physics** — drag and buoyancy scaled by how deep an entity sits, and swimming on the
  jump key, shared by client prediction and the server.
- **Content** — `base_game/liquids.lua` registers water, biomes name a `base_liquid`, and
  the generator floods everything below sea level that the sky can reach. `/water <x> <y>
  [level]` pours some in by hand.

The rewrite kept the original's shape (a type registry, `flow_time`, `speed_multiplier`) and
dropped what did not survive contact with the rest of the codebase: `Rc<LiquidType>` handles,
raw indexing with `assert!`, and the three `//TODO` holes where events and serialization
should have been.

Not done, and worth knowing before someone expects it: no bucket item, so a player cannot
carry water; and liquids neither drown anyone nor interact with lighting.


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

**Resolved: finished and wired in — see "5.1, resolved: liquids are back" above.**

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
