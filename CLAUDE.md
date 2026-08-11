# Terralistic — working notes

A Terraria-like 2D sandbox game in Rust. Single binary that runs as client, server, or
server-with-GUI. Rendering is SDL2 + raw OpenGL 3.3 through a hand-written UI toolkit.
Game content (blocks, items, walls, biomes, recipes, commands) lives in **Lua mods**, not
in Rust — `base_game` is itself a mod.

~19k lines of Rust across 145 files. Small enough to read in full; do that before large refactors.

## Commands

```bash
cargo run                 # client
cargo run --release       # client, release
cargo run -- server       # server with GUI
cargo run -- server nogui # headless server
cargo run -- version      # print version
cargo test                # 59 tests, all should pass
cargo clippy --all-targets
./coverage.sh             # coverage via config-coverage.toml
```

Tests are pure unit tests — no graphics context needed, so they run anywhere.
CI enforces `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings`; the tree is
currently clippy clean, so keep it that way rather than dropping the flag.

## Entry point

`main.rs` dispatches on `argv[1]`: absent or `client` → `client_main()`, `server` →
`server_main()`, `version` → print. It also carries the entire crate's lint configuration:
~136 `#![warn(clippy::...)]` lines at the top. **That list is the project's style contract** —
`unwrap_used`, `expect_used`, `panic`, `indexing_slicing`, `todo` are all warned on. Prefer
`Result` + `anyhow` and `.get()` over indexing. When you must violate one, add a local
`#[allow(...)]` with a reason comment, which is the existing convention.

`rustfmt.toml` sets `max_width = 200`. Lines are long on purpose; don't reflow them.

## Layout

| Path | What |
|---|---|
| `main.rs` | Entry point + crate-wide lints |
| `build_main.rs` + `build_project/` | Build script: compiles resources and the base_game mod |
| `shared/` | Game logic used by **both** client and server |
| `server/server_core/` | Authoritative simulation, networking, world gen, commands |
| `server/server_ui/` | Optional GUI for the server (console, player list, stats) |
| `client/game/` | In-game client: rendering, input, prediction |
| `client/menus/` | Title screen, world selector, settings, multiplayer, login |
| `libraries/graphics/` | The UI toolkit + OpenGL renderer (no game knowledge) |
| `libraries/events/` | Type-erased event queue (`Box<dyn Any>` + downcast) |
| `base_game/` | Lua mod: all actual game content |
| `resources/` | Client-side assets (fonts, icons, UI textures) |

### The shared/server/client triple

Most subsystems exist three times and that's the core pattern to understand:

- `shared/blocks/` — the data structure and rules (`Blocks`, `BlockId`, break logic, serialization)
- `server/server_core/blocks.rs` — `ServerBlocks`: owns the authoritative copy, sends packets
- `client/game/blocks.rs` — `ClientBlocks`: applies packets, renders

Same for `walls`, `items`, `entities`, `players`, `mod_manager`. **When changing game
behaviour, ask which of the three layers it belongs in.** Logic that must agree between
client and server (physics, collision, inventory rules) goes in `shared/` — putting it in
one side only causes desync.

## Architecture

### Event bus

`libraries/events` is a `VecDeque<Box<dyn Any + Send>>`. Everything is pushed as an
`Event::new(SomeStruct { .. })` and consumed by downcasting:

```rust
if let Some(event) = event.downcast::<BlockChangeEvent>() { ... }
```

There is no registration or dispatch table. Both `Server::handle_events` and the client's
main loop drain the queue and offer each event to *every* subsystem in a fixed order.
Consequences to keep in mind:

- Adding an event type = defining a struct. Nothing enforces that anyone handles it.
- Handler order is the literal call order in `handle_events()` / the client `while let` loop.
- Handlers can push new events while draining; those are processed in the same pass.

### Networking

`shared/packet/packet.rs` is the whole protocol. A `Packet` is `{ id: u64, data: Vec<u8> }`,
where `id` is an FNV hash of `TypeId::of::<T>()` and `data` is bincode.

**All bincode goes through `libraries/serialization.rs`** — packets, world saves and `.mod`
files alike. That module picks the format (bincode 2's default: little endian, varint) in
one place. Don't call `bincode::` directly; if the backend ever changes, that file is meant
to be the only edit. Receivers call
`packet.try_deserialize::<SomeType>()`, which returns `None` if the type hash doesn't match.

This means: **there is no packet registry and no version negotiation.** Any serializable
struct is a packet. Packet structs live next to their subsystem (`BlockChangePacket` in
`shared/blocks/blocks.rs`, `InventorySwapPacket` in `shared/inventory/`, etc.).

Caveats worth knowing before touching it:
- `TypeId` is not stable across compiler versions, so client and server must be built by
  the same rustc. There is no handshake that checks this.
- Renaming a packet struct silently changes its wire id.
- Transport is `message-io` FramedTcp. The bind address is explicit per server, via
  `BindAddress`: the dedicated server uses `AllInterfaces`, singleplayer uses `Loopback`.
  **Keep singleplayer on loopback** — it runs a real `Server`, and binding it wide would put
  every singleplayer world on the local network.
- There is no authentication of any kind. Anyone who can reach the port can join.
- Ports: singleplayer 49152, multiplayer 49153 (`server/server_core/core_server.rs`).

Connection handshake: client sends `NamePacket` → server replies `WelcomeCompletePacket` →
client stops "welcoming" mode and starts the normal receive loop. Welcome-phase packets are
buffered into `pre_events` in `client/game/core_client.rs` and drained before the game starts.

Both networking modules run a dedicated thread with an mpsc channel pair in and out. The
main loop only ever touches the channels, never the socket.

### Mod system (Lua via rlua)

`shared/mod_manager.rs`. Each `GameMod` owns its own `Lua` state, its minified source, and
a `HashMap<String, Vec<u8>>` of resources. Resource keys use `:` as separator, e.g.
`blocks:dirt.opa`.

The on-disk `.mod` format is `snap(bincode(GameModData))` — see `shared/mod_data.rs`, which
is deliberately a dependency-free leaf module so the build script can write mods without
pulling in Lua.

Rust exposes functions to Lua with the `terralistic_` prefix — `ModManager::add_global_function`
adds that prefix automatically, so `add_global_function("get_block", ..)` is called as
`terralistic_get_block(x, y)` from Lua. The registration sites are the `mod_interface.rs`
files:

- `shared/blocks/mod_interface.rs` — `register_block_type`, `connect_blocks`, `get_block`, `register_tool`, block inventories
- `shared/items/mod_interface.rs` — item types, block/wall drops, recipes
- `shared/walls/mod_interface.rs` — wall types
- `server/server_core/world_generator/mod_interface.rs` — biomes and ores

Lua lifecycle hooks a mod may define: `init()`, `init_server()`, `update()`, `stop()`.
Commands are discovered by convention: any global named `command_<name>` becomes the
`/<name>` command, with optional `describe_command_<name>()` for help text
(`server/server_core/commands.rs`).

`base_game/` splits content across `main.lua`, `blocks.lua`, `walls.lua`, `items.lua`,
`tools.lua`, `recipes.lua`, `biomes.lua`, `commands.lua`. The build script concatenates
**all** `.lua` files in the directory into one chunk, so they share one global namespace and
load order is filesystem order — which is why `main.lua`'s `init()` calls the `register_*`
functions explicitly in dependency order (tools → blocks → walls → items → recipes) rather
than relying on file order.

`terralistic_register_block_type` takes 16 positional arguments. The Lua call sites use
`-- comment` lines to label each one; keep that style if you add parameters, and remember
that adding a parameter means editing every call site in `base_game/`.

### World generation

`server/server_core/world_generator/`. Order of operations:

1. Walk a biome graph (weighted random edges) to produce a per-column biome id array; this
   also determines final world width (`min_width` is a floor, not the exact value).
2. Convolve per-column min/max terrain heights and cave thresholds across biome boundaries
   so transitions are smooth (5 passes, kernel 50).
3. Perlin turbulence for terrain height, caves, and each ore.
4. Generate column by column; at each biome boundary hand the accumulated columns to the
   biome's Lua `generator_function` for decoration (trees, etc.).

Default world is 4400×1200, seed 423657, hardcoded in `Server::start`.

### Rendering and UI

`libraries/graphics` is a self-contained immediate-mode-ish toolkit over raw `gl` calls.
It has no game knowledge — treat it as a vendored library.

The UI contract is `UiElement` / `BaseUiElement` in `ui_element.rs`. You implement
`UiElement` (`get_container`, plus optional `render_inner` / `update_inner` /
`on_event_inner` and the `get_sub_elements*` accessors); `BaseUiElement` is blanket-implemented
and handles recursing into children. **Implement `UiElement`, call `BaseUiElement`.**

Layout is `Container` + `Orientation` (`TOP_LEFT`, `CENTER`, …): a child positions itself
relative to a parent container by orientation plus offset. Theme constants (colors, `SPACING`,
`BLUR`, `TRANSPARENCY`) are in `theme.rs` — use them rather than literals.

Rendering goes to an offscreen texture (`window_texture`) which is blitted to the default
framebuffer in `update_window()`, which is what makes the blur/shadow effects possible.

Several older UI pieces predate the `UiElement` trait and are hand-rolled — the `//TODO make
this a UI element` comments in `client/game/chat.rs`, `pause_menu.rs`, `debug_menu.rs`,
`inventory.rs`, `respawn_screen.rs` mark them. Converting one is a good self-contained task.

### Timing

- Server: fixed 20 TPS (`tps_limit`), with a 5 ms accumulator inside `update()` driving
  player and entity physics.
- Client: renders as fast as allowed (vsync / fps limit configurable), with the same 5 ms
  accumulator via `FramerateMeasurer::has_5ms_passed()` for simulation.
- Physics constants live in `shared/entities/entities.rs` and `shared/players.rs`. The
  `/ 200.0` divisors there are the 5 ms tick expressed as a fraction of a second.

Client-side prediction: the client simulates its own player and periodically sends
`PlayerPositionPacketToServer`; the server accepts it if within a tolerance of 2.0 blocks,
otherwise force-corrects (`server/server_core/players.rs`).

### Persistence

World save is `bincode(HashMap<String, Vec<u8>>)` with keys `blocks`, `walls`, `players`.
Blocks and walls are additionally snap-compressed. Written to `server_data/server.world`
relative to the process CWD. Client settings are JSON at
`<data_dir>/Terralistic/settings.txt`.

**There is no save format version field.** Any change to `BlocksData`, `SavedPlayerData`,
or the wall equivalent silently breaks existing worlds.

### Build pipeline

`build_main.rs` runs before every build and does two things:

1. `compile_resource_pack(resources/ → Build/Resources/)` — converts PNG to the custom
   `.opa` format (raw serialized `gfx::Surface`), copies everything else.
2. `compile_mod(base_game/)` — concatenates the Lua, minifies it with darklua, bundles
   resources, bincode + snap, writes `base_game/base_game.mod`.

`base_game/base_game.mod` and `Build/Resources/*` are **build artifacts that are committed**
and `include_bytes!`-ed into the binary. If you edit `base_game/*.lua` or `resources/*`, the
regenerated `.mod`/`.opa` files show up as diffs — that's expected, not a mistake.

The `.mod` build is reproducible: identical sources produce identical bytes. It used not to
be, because resources were serialized from a `HashMap` whose iteration order Rust randomises
per process, so the committed artifact changed on every single build. `GameModData.resources`
is a `BTreeMap` to keep that stable — don't change it back.

`Template_*.png` files get expanded at build time into 16-frame connected-texture atlases
(`process_template` in `build_project/compile_mod.rs`) and lose the prefix in the output.

## Gotchas

- **`build_main.rs` declares its own narrow module tree.** It names individual leaf files
  (`libraries/graphics/{color,position,surface}.rs`, `shared/mod_data.rs`) rather than
  `pub mod graphics;` / `pub mod shared;`, so the build script does not compile the game's
  dependency tree. Module *paths* must still match `main.rs`, because those files refer to
  themselves as `crate::libraries::graphics` and `crate::shared`. If the build script ever
  needs another type, prefer moving that type to a dependency-free leaf module over
  widening the declaration.
- `shared/world_map/world_map.rs` has two coordinate schemes that disagree:
  `translate_coords` is `x * height + y` (column-major), `translate_chunk_coords` is
  `x + y * width` (row-major). Both are internally consistent; don't "fix" one in isolation.
- `server/server_core/core_server.rs` uses `static mut` for tick counters and the UI sender.
  This warns under `rust_2024_compatibility` and is genuinely not thread-safe. Two `Server`
  instances in one process share those counters.
- Server `Blocks`/`Walls`/`Items`/`Entities` are behind `Arc<Mutex<_>>` and accessed via
  `get_blocks()` etc. that lock. Holding two of these at once in the wrong order is a
  deadlock waiting to happen; the codebase currently always takes them one at a time.
- `PoisonError::into_inner` is used everywhere instead of propagating lock poisoning. That's
  deliberate — follow it for consistency.
- Empty stub test files exist at `shared/{walls,items,inventory,liquids}/tests.rs`
  (`mod tests {}`). They are wired into their `mod.rs` already, so adding tests there needs
  no plumbing.
- `shared/liquids/` is **dead code** — fully written, never constructed by client or server,
  no serialization. Don't assume it works.

## CI

`.github/workflows/rust.yml` runs `cargo test` on Ubuntu for pushes/PRs to `master` only.
No clippy, no fmt check, no release build, and nothing runs on other branches.
