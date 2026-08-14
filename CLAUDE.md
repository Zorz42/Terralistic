# Terralistic — working notes

A Terraria-like 2D sandbox game in Rust. Single binary that runs as client, server, or
server-with-GUI. Rendering is wgpu through a hand-written UI toolkit, with winit for the
window and input.
Game content (blocks, items, walls, liquids, biomes, recipes, commands) lives in **Lua
mods**, not in Rust — `base_game` is itself a mod.

~12k lines of game Rust, on top of ~11k lines of libraries that know nothing about it, plus
~9k lines of tests. Small enough to read in full; do that before large refactors.

**`libraries/` is the boundary that keeps it that way.** Nothing under it may name a game
concept or import `crate::shared`, `crate::client` or `crate::server` - which is true today
and worth checking before you add to one. Each library's `mod.rs` opens with what it does
**and what it deliberately does not do**; the second half is what stops game logic drifting
in. See `docs/LIBRARIES.md` for what is extracted and what is still a candidate.

## Commands

```bash
cargo run                 # client
cargo run --release       # client, release
cargo build --profile dist # what you ship: release + LTO, 4.63 MB vs 5.49 MB
cargo run -- server       # server with GUI
cargo run -- server nogui # headless server
cargo run -- version      # print version
cargo test                # 532 tests, all should pass
cargo clippy --all-targets
./coverage.sh             # coverage via config-coverage.toml

cargo run --features render-tests -- rendertest             # 46 golden-image tests
cargo run --features render-tests -- rendertest dump        # + viewable PPMs of every case
cargo run --features render-tests -- rendertest regenerate  # rewrite the goldens
```

Tests are pure unit tests — no graphics context needed, so they run anywhere.

**The profiles are tuned for the loop you are actually in.** `dev` and `test` rebuild in
about 2s after a one line edit; `release` takes 10s because it optimises but does not link
the whole program; `dist` takes a minute or more because it does. `[profile.test]` used to
carry `lto = true` and `opt-level = 3`, which made every `cargo test` a 62 second wait to run
a two second suite — if you find yourself tempted to add LTO to a profile you rebuild, that
is what it costs.
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
| `client/menus/` | Title screen, world selector, settings, multiplayer |
| `libraries/graphics/` | The wgpu renderer: draw list, backend, window, textures, glyphs |
| `libraries/ui/` | The widget toolkit: layout, input, widgets, menus, list pages, docks |
| `libraries/events/` | Type-erased event queue (`Box<dyn Any>` + downcast) |
| `libraries/grid/` | Bounded 2D grids, chunk addressing, least-recently-used eviction |
| `libraries/registry/` | Register a value, get a typed id back; look up by id or name |
| `libraries/timing/` | Fixed-step accumulators, frame limiter, work budgets, frame stats |
| `libraries/net/` | Typed-message TCP transport, both halves. No protocol |
| `libraries/scripting/` | Sandboxed lua modules with resources. No game hooks |
| `libraries/config/` | Settings registered at runtime, persisted as a flat file |
| `libraries/container_file/` | Versioned magic-header files holding named sections |
| `libraries/log/` | Timestamped levelled lines and one process-global sink |
| `libraries/procgen/` | Fractal noise, 1D smoothing, weighted picks |
| `libraries/testing/` | `#[cfg(test)]`: temp dirs, free ports, spin-until-or-fail |
| `libraries/serialization.rs` | The one place the binary format is chosen |
| `base_game/` | Lua mod: all actual game content |
| `resources/` | Client-side assets (fonts, icons, UI textures) |
| `integration_tests/` | Tests that drive several subsystems against each other |

### The shared/server/client triple

Most subsystems exist three times and that's the core pattern to understand:

- `shared/blocks/` — the data structure and rules (`Blocks`, `BlockId`, break logic, serialization)
- `server/server_core/blocks.rs` — `ServerBlocks`: owns the authoritative copy, sends packets
- `client/game/blocks.rs` — `ClientBlocks`: applies packets, renders

Same for `walls`, `liquids`, `items`, `entities`, `players`, `mod_manager`. **When changing game
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

**The transport is `libraries/net`; the protocol is the game's.** `PacketServer` and
`PacketClient` own the sockets, the threads and the channels. `shared/packet/` and the two
`networking.rs` files own what is said: the version, the name, the welcome, and who counts as
"everyone".

A `Packet` (`libraries/net/packet.rs`) is `{ id: u64, data: Vec<u8> }`, where `id` is an FNV
hash of `TypeId::of::<T>()` and `data` is postcard.

**All serialization goes through `libraries/serialization.rs`** — packets, world saves and
`.mod` files alike. That module picks the format (postcard: little endian, LEB128 varint) in
one place. Don't call `postcard::` directly; when the backend changes, that file is meant to
be the only edit — and when bincode went unmaintained, it very nearly was. Receivers call
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
- **The bind happens on the networking thread, so its failure arrives late.** `init` returns
  before the port is even attempted; the thread's `Result` only reaches anyone when
  `ServerNetworking::update` notices the thread has finished and joins it. A server whose
  port is taken therefore looks like it started and then accepts nobody forever, so the
  error names the address it could not bind rather than being a bare `AddrInUse`.

Three seams keep the policy out of the library, and they are where to look when changing it:

- The server hands over **every** packet from every connected peer, including one it is about
  to refuse. `ServerNetworking::handle_packet` is the version and name check, and
  `PacketServer::disconnect` is how it acts. That check used to live on the networking thread.
- The client's handshake is two things the game supplies: a greeting to send on connecting
  (version, then name) and a predicate saying which packet ends it
  (`WelcomeCompletePacket`). `Received::during_handshake` is how the game still knows which
  traffic was the welcome.
- `net::ClientError` is a *kind*, not a string. The transport knows the socket shut; only
  `ClientNetworking::explain` knows that a close mid-handshake means the server refused this
  client. Player-facing wording lives there.

Connection handshake: client sends `NamePacket` → server replies `WelcomeCompletePacket` →
client stops "welcoming" mode and starts the normal receive loop. Welcome-phase packets are
buffered into `pre_events` in `client/game/core_client.rs` and drained before the game starts.

Both networking modules run a dedicated thread with an mpsc channel pair in and out. The
main loop only ever touches the channels, never the socket.

**The client's welcome phase runs a 1ms signal timer of its own**, and that is what makes it
cancellable. Everything else in that phase happens in reaction to something the server sent, so
with no timer there was no point at which the thread looked at `is_running` — `stop()` waited
forever on a thread that was not listening, and the game could not offer to quit during a join
at all. The timer is armed once before `listener.for_each` and re-armed by whichever branch
handles it, so there is exactly one chain: don't send a second signal when the welcome
completes.

### Mod system (Lua via rlua)

`libraries/scripting`. Each `ScriptModule` owns its own `Lua` state, its minified source, and
a `HashMap<String, Vec<u8>>` of resources. Resource keys use `:` as separator, e.g.
`blocks:dirt.opa`. A `ScriptHost` drives a set of them.

The on-disk `.mod` format is `snap(postcard(ScriptModuleData))` — see
`libraries/scripting/module_data.rs`, which is deliberately a dependency-free leaf module so
the build script can write mods without pulling in Lua. `build_main.rs` names that leaf file
rather than the `scripting` module, which is what keeps rlua out of `[build-dependencies]`.

A handle type is made passable to lua by `script_handle!(Type)` (or `script_handle!(Type,
eq)` where lua should be able to compare two), which is a macro because the orphan rule stops
either side writing the blanket impl. Anything that is not a handle is a lua conversion error
rather than the `unreachable!()` the six hand written copies each had.

Rust exposes functions to Lua with the `terralistic_` prefix — `ScriptHost::add_global_function`
adds it automatically, so `add_global_function("get_block", ..)` is called as
`terralistic_get_block(x, y)` from Lua. The prefix is `shared::MOD_FUNCTION_PREFIX`, handed to
the host at construction: it is what keeps *this* game's names out of a mod's way, so it lives
with the game rather than in the library. The registration sites are the `mod_interface.rs`
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
5. Flood everything below sea level that the sky can reach, with each column's biome's
   `base_liquid`.
6. Grow the multiblocks (`ServerBlocks::expand_big_blocks`), which is what turns a tree's one
   canopy cell into its 5x5 footprint.

**Step 6 visits multiblocks only, and that is load-bearing for how long joining takes.**
`update_block` on every cell in the world pushed a `BlockUpdateEvent` per cell — 5.4M of them
for the default world — which sat in the queue until the first `update()` and were then offered
to every subsystem and handed to lua's `on_block_update` one at a time. That first update took
18 seconds in a debug build and held about a gigabyte, and it ran *after* the loading screen had
closed, with the client already connected and waiting to be welcomed: a newly generated world
looked exactly like a game that had hung. The scan for where to start still reads every cell, so
it takes the lock once and clones nothing — cloning a `Block` (two `Vec`s) 5.4M times was
seconds of its own.

`start` has to return a **finished** world, which is why the growth is a local walk rather than
being left to the `BlockChangeEvent`s it queues: the client is welcomed with a copy of the world
and a save can be written before the first update, so a canopy half grown at that moment is a
canopy half grown on disk.

**Sea level is a percentile of the terrain, not a constant.** `FLOODED_COLUMN_FRACTION` is
0.15, so sea level is the height that 15% of the columns are lower than: the lowest ground
of whatever world the generator is handed floods, and a perfectly flat one does not. A fixed
y would be underground or in the sky for any mod that picked different terrain heights,
which are lua.

**The fill walks down from the top of the world, not from sea level**, stopping at the first
non-ghost block. That is the difference between lakes and a hillside with water hidden
inside it: starting at sea level fills any cave that happens to cross that line, whether or
not anything could have poured into it, and it put water in a third of the columns of a test
world instead of the intended 15%. Coming down from the sky, a column whose ground is above
sea level stops at its own surface, while a pit or a cave mouth open to the sky below sea
level floods — which is what a hole in a shoreline should do, and the flow simulation drains
it from there.

Default world is 4400×1200, seed 423657 — `Server::world_size` and `Server::world_seed`,
defaulted in `Server::new` from the `DEFAULT_WORLD_*` constants. They are fields rather
than literals inside `start` so a test can ask for a world small enough to assert about.

Generation is reproducible from the seed on the rust side, but **not end to end**: each
biome's lua `generator_function` decorates with `math.random`, which lua seeds per state.
So terrain and walls repeat for a given seed and the trees do not.

### Liquids

`shared/liquids/` is the grid and the flow simulation, `server/server_core/liquids.rs` owns
the authoritative copy, `client/game/liquids.rs` draws what it is told. It was carried in the
tree commented out for a long time; what is there now is a rewrite in the current style, not
the original.

A cell is a `Liquid { id: LiquidId, level: u8 }`, where level runs 0 to `MAX_LIQUID_LEVEL`
(100). Empty is a registered type at id 0, so `create` fills the grid with something real
rather than `undefined` — the trap `Walls::create` still has. `set_liquid` normalizes level 0
back to the empty type, so nothing downstream has to handle "20% of nothing".

Four things about it are load-bearing:

- **The simulation is scheduled, not scanned.** A full scan is 5.28M cells at 20 TPS for the
  default world. Instead every change schedules itself and its four neighbours in a
  `BTreeSet`, and a settled cell drops out and costs nothing. `BTreeSet` rather than
  `HashSet` because the order cells flow in decides how a stream splits, and Rust randomises
  hash iteration per process.
- **The scheduled set is derived, not saved.** `Liquids::deserialize` clears it and
  `Server::start` calls `schedule_all_unsettled`, which scans once and keeps only cells with
  room beside them — for a settled ocean, none of them. A world saved mid-splash carries on
  flowing; a settled one costs one scan.
- **A liquid only moves when something wakes it.** `ServerLiquids::on_event` schedules on
  `BlockChangeEvent` for exactly that reason: digging the floor out from under a pool is not
  a liquid change, and without it the pool never moves again.
- **The client never simulates.** The server sends one `LiquidChangesPacket` per update
  holding every cell that changed, because water flowing through a cave is hundreds of cells
  a second and one packet each — the way blocks do it — is a different order of traffic.

Flow order per cell is down first, then an averaging step with whichever horizontal
neighbours hold *less*. Both halves matter: a neighbour holding more is left alone because
evening a pair out from both sides is how liquid sloshes forever, and the integer remainder
stays with the source cell so a row that will not divide evenly settles instead of passing
the last drop back and forth. Each type flows on its own clock (`flow_time`, in ms) and is
not owed steps it missed - `timing::Interval`, which is that rule written down once.

Physics is in `shared/entities/entities.rs`: `liquid_submersion` samples the cell the
entity's middle is in, and the level scales both the extra drag (through the type's
`speed_multiplier`) and the buoyancy that cancels most of that tick's gravity. Holding jump
under the surface swims (`update_players_ms`). Both run on client and server, off each
side's own copy, which is what keeps prediction agreeing with the authority.

Content is `base_game/liquids.lua` — water, and nothing else yet. The lua interface is
`register_liquid_type`, `get_liquid_id_by_name`, `get_liquid`, `get_liquid_level`, plus
`set_liquid` which is **server only**, like `set_block`, because the client's grid is a
replica. `/water <x> <y> [level]` pours some in by hand. The obvious next step is a bucket
item (`places_liquid` alongside `places_block`), which does not exist.

**Liquids draw in front of the player**, after the entities and before the floating damage
text (`core_client.rs`). Water is translucent, so a player wading through a lake is seen
through its surface rather than pasted on top of it.

### Rendering and UI

**Two libraries, and the split matters when you go looking.** `libraries/graphics` is the
renderer - draw list, wgpu backend, window, surfaces, textures, glyphs. `libraries/ui` is the
widget toolkit - layout, input routing, widgets, menus. Neither has game knowledge; treat
both as vendored.

It is a module split inside one crate, not a crate split, so the two do refer to each other.
There is one edge that is not going away without a bigger change: `render_inner` and
`update_inner` take a `&mut GraphicsContext`, because a few widgets genuinely build textures.
`RenderRect` reaches `GraphicsContext::blur_rect` and `shadow_context`, which are
`pub(crate)` for it.

The UI contract is `UiElement` / `BaseUiElement` in `ui/ui_element.rs`. You implement
`UiElement` — `get_container` is the only required method, everything else defaults — and
`BaseUiElement` is blanket-implemented and handles recursing into children. **Implement
`UiElement`, call `BaseUiElement`.** The one default worth knowing is `get_sub_elements*`,
which returns nothing: a leaf writes neither, and **an element with children has to override
both**, or it is laid out and drawn while its children are not.

Layout is `Container` + `Orientation` (`TOP_LEFT`, `CENTER`, …): a child positions itself
relative to a parent container by orientation plus offset. Theme constants (colors, `SPACING`,
`BLUR`, `TRANSPARENCY`, and the widget defaults) are in `theme.rs` — use them rather than
literals. **Every fade and slide in the game is `ui::approach(value, target, smooth_factor, epsilon)`**
per ready `timing::FixedStep::for_animation` frame; don't hand-roll another one. Nine places
in `client/` used to, none of them with an epsilon, and two of them wrote the epsilon out by
hand as a pair of `if`s afterwards. The epsilon is not
decoration — it is what makes an animation *land* on its target instead of nearing it forever,
which a hand-rolled `value -= value / factor` does not.

Hit testing is `UiElement::is_hovered`, a default method over `get_container`. Override it
only where an element is hoverable on other terms; `Button` does, because a disabled one is
never hovered.

Clicking is `gfx::ClickTracker`, which `Button` and `Toggle` each own one of. See the gotcha
below for the rule it encodes; the point of it being one type is that the rule is written down
once, and a third clickable widget gets it by construction.

#### `UiContext`: the line between layout and drawing

The trait methods split by what they need:

| Method | Takes | Why |
|---|---|---|
| `get_container`, `on_event_inner` | `&dyn UiContext` | layout and input only |
| `render_inner`, `update_inner` | `&mut GraphicsContext` | needs the GPU |

`update_inner` is where a widget advances an animation; `render_inner` is where it records
what to draw. **Keep the two apart** — a widget that steps its animation while rendering
freezes for any caller that lays a list out without drawing it, which is what `Scrollable`
did to menus sliding offscreen.

**`Scrollable` is vertical, all of it.** `scroll_pos` is bounded against `rect.size.1` and
`get_scroll_y` measures from `rect.pos.1`, which the world and server lists add straight to a
row's y. It used to be `get_scroll_x` reading `rect.pos.0`, and only produced the right number
because both menus leave their x at zero and added the y offset back by hand — so setting the
scrollable's x would have slid the rows vertically. If you ever want a horizontal one, add the
axis rather than reinterpreting this one.

`update_inner` takes a `&mut GraphicsContext` because a few of the game's menus genuinely
build textures in one, which leaves any animation stepped there unreachable from `cargo test`.
Where the stepping needs nothing from the context, split it into a `pub(super)` method and
call that — `Scrollable::advance_frame` and `TextInput`'s three geometry methods are the
pattern, and both were untestable before.

`UiContext` (`ui_context.rs`) is the whole non-rendering surface of `GraphicsContext`:
window size, mouse position, key states, clipboard. `GraphicsContext` implements it, and so
does `HeadlessContext`, a `#[cfg(test)]` struct whose inputs are plain fields. That is what
makes hit testing, layout and event routing testable in CI with no window — see the headless
half of `libraries/graphics/tests.rs`.

**Keep new widgets off `GraphicsContext` in those two methods.** If an event handler needs
to draw or upload a texture, the work almost always belongs in `render_inner`. Three menus
genuinely do not fit — `world_creation` and `multiplayer_selector` build the next menu, and
`settings_menu` applies vsync/scale — and they use the documented escape hatch
`UiContext::as_graphics_context() -> Option<&mut GraphicsContext>`, which is `None` headlessly.
Those branches are the only UI logic tests cannot reach.

Companion `#[cfg(test)]` constructors exist for the same reason: `Font::new_headless` (skips
the glyph texture upload, so text measurement and `create_text_surface` are testable),
`Texture::new_sized` (reports a size, allocates nothing) and `TextInput::new_headless`.

Rendering goes to an offscreen texture which is blitted to the window in `update_window()`,
which is what makes the blur/shadow effects possible.

**That offscreen is the display's real pixel size, not the logical one.** Layout is in
logical pixels either way — the transform mapping those onto clip space is a ratio and does
not care how many pixels the target has — but drawing at the real resolution is what lets a
smooth animation move one device pixel at a time instead of jumping two, and it makes the
final blit a copy rather than an upscale. It costs about 1 ms of GPU per frame in game
(3.1 ms → 4.2 ms measured on an M-series Mac at 3340x2100), which leaves a 60 fps frame
plenty of headroom. If that ever becomes a problem on weaker hardware, the offscreen size is
chosen in one place, `GraphicsContext::handle_window_resize`, and would make a reasonable
setting.

#### The window: `window.rs` is the only module that knows winit exists

`GraphicsContext` deals in `gfx::Event` and `gfx::IntSize`; `events.rs` is pure data with no
window-system types in it.

The game's three main loops (`client/game/core_client.rs`,
`client/menus/title_screen_renderer.rs`, `server/server_ui/ui_manager.rs`) drive their own
simulation and rendering, so the event loop is **pumped, not run**:
`EventLoopExtPumpEvents::pump_app_events` with a zero timeout. Inverting three loops around
winit's `run_app` callback was not worth it. The cost is that a resize drag does not repaint
mid-drag on macOS, and that `pump_app_events` is desktop only — which is all this builds for.

Three things about it are load-bearing:

- **Keys are physical positions, not labels.** `translate_key` maps winit's `KeyCode`, so
  `Key::W` is wherever W sits on QWERTY and WASD stays a square on AZERTY. Typing is
  unaffected: text arrives separately as `Event::TextInput`.
- **`Geometry` caches the window size, and must.** Layout asks ~540 times a frame — every
  `Container`, the camera bounds, every chunk testing visibility. winit's `inner_size` and
  `scale_factor` are objc message sends on macOS at ~12µs each, which measured at 40% of the
  game's wall clock. The resize events are the authority; never read the size back per call.
- **The pump happens at the end of `update_window`, not at the top of the frame.** On macOS
  pumping is what services the layer's pending drawable, so with slack in the frame the wait
  for the display lands there and can be most of a frame. See *Timing* for why that has to
  stay outside the frame's budget window. `get_event` only pumps if nothing has yet this
  frame (`polled_this_frame`).

The window also asks for focus on creation — a pumped loop leaves macOS not treating the app
as active, so it would otherwise open behind the terminal — and `key_states` is cleared when
the window loses focus, so a held key does not stick across an alt-tab.

winit reports the true `HiDPI` drawable size, and the game draws at it rather than being
smoothed up to it by the compositor. That also means the surface can be bigger than
`Limits::downlevel_defaults` allows a texture to be, which is why the device asks for
`downlevel_defaults().using_resolution(adapter.limits())` — with the plain defaults a
1670x1050 window on a 2x display fails `Surface::configure` outright.

#### The draw list: what to draw vs. how

**No drawing call touches the graphics API.** `rect.render(..)`, `texture.render(..)`,
`rect_array.render(..)`, `font.render_text(..)` and `shadow_context.render(..)` all *record*
a `DrawCommand` into the frame's `DrawList`. `GraphicsContext::update_window` hands the whole
list to `WgpuBackend::execute`, which is the only place the frame path talks to wgpu.

| File | Role |
|---|---|
| `draw_list.rs` | `DrawCommand`, `DrawList`, `BlendMode`, the `DrawTarget` trait, and `DrawRecorder` for tests |
| `wgpu_backend.rs` | `WgpuBackend`: pipelines, uniforms, offscreen textures, blur, present |
| `shaders.wgsl` | Both shaders. One vertex entry point, one fragment each for normal draws and blur |
| `gpu_device.rs` | The device, the resource registry, and deferred release |
| `window.rs` | The window and the input. The only module that talks to winit |

Consequences worth knowing:

- **Commands are in window pixels**, y-down from the top left, holding the caller's raw
  arguments (pos, scale, source rect) rather than a precomputed destination. Clip space, the
  y flip and the divide by window size all belong to the backend. Keeping the arithmetic
  there in the same order is what keeps the goldens bit-exact.
- **`DrawTarget::push_draw_command` takes `&self`**, backed by a `RefCell` on
  `GraphicsContext`. It has to: `graphics.font.render_text(graphics, ..)` and
  `graphics.shadow_context.render(graphics, ..)` borrow the context twice. So `render`
  methods take `&dyn DrawTarget`, and `&mut GraphicsContext` coerces to it.
- **Order is the list order.** Blur and blend mode are commands (`DrawCommand::Blur`,
  `SetBlendMode`) precisely because they only mean anything relative to the draws around
  them. Call `graphics.set_blend_mode(..)`, which records.
- **Blend mode is baked into a pipeline**, so the backend keeps one per mode and switching
  mid-frame means switching pipeline. There is no global state for a frame to inherit.
- **The flush is the *first* thing `update_window` does**, before the blur and scale
  animations advance and before the normalization transform is recomputed. That is what
  makes deferral invisible: a command executes with exactly the transform and blur intensity
  it would have been drawn with immediately. Don't reorder it.
- **`handle_window_resize` also owns the normalization transform**, and has to. The frame
  about to be recorded draws through whatever transform is in force *now*, so a transform
  only ever set at the end of `update_window` is the identity for the first frame — the
  whole window's drawing collapsing into the top-left two-by-two of clip space — and one
  size stale for the frame after every resize. Both last exactly one frame, which is why
  neither was ever noticed; removing the call is how you get them back.
- **A `Blur` command splits the frame into separate render passes**, because wgpu cannot
  sample the texture it is currently drawing into. Everything before the blur is in one
  pass, the gaussian ping-pongs between the two scratch textures, and the rest resumes
  with `LoadOp::Load`.
- **The clear gets a render pass of its own**, on the frame texture, before any of that. Only
  the golden harness ever asks for a clear — the game draws an opaque background over the whole
  window — and `blur_over_a_cleared_frame` is the case that pins it. It used to have to be its
  own pass for a second reason too: the blur's first pass wrote the *back* offscreen, so a clear
  folded into it landed on the wrong texture and the blur read the previous frame straight back
  in.

##### The blur is the frame's one area-priced effect, so it runs small

Every pass is thirteen taps for every pixel of the region and there are four to six of them, so
a menu blurring most of a fullscreen `HiDPI` window is seven million pixels through half a
billion samples. Measured at 3340x2100 on an M-series Mac that was **7.3 ms of GPU in one
command** — most of a 60 fps frame before the game has drawn anything — which is what made a big
blur region drop a fullscreen window to 45 fps.

So the region is shrunk into a scratch texture, blurred there, and stretched back
(`plan_blur`). The tap spacings are unchanged — they are still the same distances in window
coordinates — so it is the same blur, sampled on a coarser grid, and a blur has nothing in it
finer than its own kernel to lose. Over vertical one pixel stripes, the worst case there is, no
channel moved more than 6 of 255 against the full resolution result, and the blur no longer
measures above the harness's noise.

Four things about it are load-bearing:

- **The downscale follows the radius** (`BLUR_TEXELS_PER_RADIUS`), because the round trip is
  itself a blur about a texel wide. Fixing it would over-blur a small radius, which is what a
  fade spends its first frames on.
- **The scratch pair is allocated at `MIN_BLUR_DOWNSCALE`**, so a whole-window region fits at
  the finest downscale and `plan_blur` never has to fit a region that does not. A region smaller
  than that uses the top left corner of the pair and clamps its sampling to it.
- **The blur samples linearly, and it is the only thing in the toolkit that does.** Stretching
  the result back with `NEAREST` is visible blocks. It gets its own bind group layout and
  sampler in `gpu_device` rather than relaxing the shared one, which declares itself
  non-filterable precisely so that a pixel art game cannot smooth anything by accident.
- **The upsample is an ordinary `Segment::Draw`**, not a pass of its own, so it joins whatever
  draw pass follows the blur. On a tiled GPU a render pass costs its whole attachment to load
  and store however small the quad in it is, and the attachment here is the frame.

`execute` plans before it encodes: `Plan` collects a `Uniforms` per draw plus a flat list of
`Segment`s, because wgpu wants all the uniform data written before any of it is encoded.

##### The device is global, and that is deliberate

`Texture::load_from_surface(&surface)` is called from ~80 places and takes no context. The
choice was to thread a `&Device` through all 80 call sites or keep the coupling and write it
down. `gpu_device.rs` is the latter — a `OnceLock<GpuDevice>` the renderer publishes once.

One consequence is an improvement: **creating a texture with no device is not an error**, it
produces a `Texture` that knows its size and owns nothing. Layout code works headlessly and
`cargo test` can build real textures.

##### Deferred release is load-bearing

Resources live in a registry inside `gpu_device` and commands name them by id, so a resource
can be dropped while a command still refers to it. That is not a corner case: several menus
build a text texture inside `render_inner` and drop it there, every world chunk replaces
its whole `RectArray` via `self.rect_array = RectArray::new()` when it changes, and the
golden cases draw from temporaries that die at the end of the statement. `Drop` therefore
parks the id and `execute` sweeps after the frame has been submitted.

**Don't make that an immediate removal.** The command would resolve to nothing and the draw
would silently vanish.

Mutating a live mesh mid-frame would still be wrong, but nothing does it: every `RectArray`
mutation replaces the whole object, so the old buffer keeps its old contents until it is
collected.

That build-once-then-only-draw pattern is also why **`VertexBuffer::upload` takes the staged
vertices rather than borrowing them**: once the GPU has the data, keeping a copy is a second
`RectArray` in RAM for every chunk — ~50 KB each across three caches of `MAX_LOADED_CHUNKS`.
An `upload` with nothing staged therefore leaves an already uploaded mesh alone instead of
replacing it with an empty one.

##### Outlines

`render_outline` draws **four one-pixel quads on the rectangle's own edge pixels**, not a
line primitive. Line rasterisation rules differ between Metal, Vulkan and DX12, which would
make the output depend on the machine — the opposite of what the goldens are for. The border
is exactly the rect's footprint, which is also what a UI border should be.

#### Golden-image tests

`libraries/graphics/render_tests.rs` renders 46 cases into the offscreen texture,
reads them back with `GraphicsContext::capture_frame`, and compares against committed
`Surface`s in `libraries/graphics/goldens/*.opa`. They cover rects and outlines, `RectArray`,
textures (scale, flip, source rect, tint), blend modes, both fonts, containers and all nine
orientations, `RenderRect` fill/border/shadow/blur, sprites, the atlas, and the button,
toggle and text input widgets.

**They are not `#[test]`s and cannot be.** They need a real window to hang a GPU surface
off, macOS requires that on the main thread, and libtest always runs a test body on a spawned
worker — even under `--test-threads=1`. So they get a `main.rs` dispatch arg instead, behind
the `render-tests` feature. `cargo test` is untouched and still needs no graphics context.

The window is hidden, so running them does not flash windows across the desktop. The harness
is also the one caller that draws at the window's *logical* resolution rather than the
display's (`render_at_logical_resolution`), and capture is taken from the offscreen texture
rather than the surface — between them, a golden is the same image whatever the DPI of the
machine that runs it. On a `HiDPI` display the game's own offscreen would be four times the
pixels and match nothing.

These are one of **two** tiers. The draw-list tests at the bottom of `tests.rs` assert on the
commands a primitive records and run under `cargo test`; these assert that the backend turns
those commands into the right pixels, which is the half no headless test can see. **Every
primitive reaches the first tier**, because building one without a device is a supported state
rather than a failure — `ShadowContext` and `Font` get textures that know their size and own
nothing, and a `RectArray` stages vertices for an upload that never happens — so "does the
shadow cover the whole border" and "does the pen land where measuring said" are ordinary
`#[test]`s, and only "is it the right colour" needs a GPU. These are also the real test of
deferred release: `fixture_texture().render(..)` drops the texture at the end of the statement,
well before the frame executes.

`CASES` is built by the `cases!` macro from the drawing functions' own names, so a golden
cannot end up compared against a different case's image. Adding one is a line: `case_foo`, or
`case_foo: BLURRY` where `EXACT` will not do.

Determinism is the whole game, and the toolkit fights it in three places. Each has a
`#[cfg(feature = "render-tests")]` hook: wall-clock animations (`FixedStep::freeze`,
`Button::settle_hover`, `Toggle::settle_animation`, `TextInput::settle_animation`), the blur
and scale fades (`GraphicsContext::settle_animations`), and hover states that read the real
mouse — which the settle hooks also neutralise. **If you add a case, run it five times before
committing the golden.**

Tolerances are per case, and **45 of the 46 are `EXACT`** — bit-identical. Only
`render_rect_blur` is `BLURRY`, because the gaussian blur shader's float error differs
between drivers. The two knobs compose rather than alternate: a pixel is over tolerance when a
channel moved further than `max_channel_delta`, and the case fails when more than
`max_differing_fraction` of the frame is over tolerance. As an `||` the channel limit was
unreachable — any 2% of a `BLURRY` frame could change by any amount at all. The shadow looks like it belongs in that group and does not: `ShadowContext`
bakes its gaussian into a CPU `Surface` once and draws it as an ordinary `NEAREST` texture.
Keep new cases `EXACT` unless a shader is genuinely involved — a loose tolerance once
absorbed a real one-pixel shift in the text input cases.

One known-bad behaviour is recorded as-is rather than fixed: **alpha eaten by blending.**
The blend factors apply to the alpha channel as well as to colour, so drawing anything
translucent lowers the framebuffer's alpha below 1. Invisible on screen, because the final
blit ignores alpha, but it shows up in a capture.

#### Texture sampling

Two rules here are easy to break by accident; both are pinned by goldens.

**No fudge factor on the source rectangle.** `WgpuBackend::plan_command` maps the quad's
`[0,1]` texture coordinate onto the source rectangle as
`u = (src.pos + t * src.size) / texture_width`. Stretching by `src.size + 0.1` instead pushes
the last output column past the end of the source rectangle — for an 8-texel region drawn at
scale 8 the final pixel samples `src.pos + 8.037`, which floors to the *neighbouring* texel.
Symptoms are a one-pixel-early quadrant boundary on any scaled texture, a missing last column
on sub-rectangle renders, and item sprites showing a sliver of another item. Sampling happens
at pixel centres, so the exact mapping already lands strictly inside the region.

##### Draws are snapped to whole pixels

`WgpuBackend::plan_command` rounds a texture draw's destination position to a whole pixel,
and that is not cosmetic. Sampling is `NEAREST`: a destination pixel takes whichever texel
its centre falls in. On a whole pixel with an integer scale those centres land halfway
through a texel and every texel gets the same number of pixels. Half a pixel off and they
land exactly *on* the texel boundaries, where which side they fall on comes down to the last
bit of a float interpolated across the quad — so a 3x glyph pixel came out 2 or 4 wide, and
differently along the string, which reads as uneven and faintly slanted text. At scale 1 it
swallowed the one pixel gaps between strokes outright.

Layout puts things on half pixels constantly — anything centred inside an odd-width parent,
anything inside a `Scrollable`, anything mid-animation — so this was visible in the world
list on every row. `text_on_fractional_offsets` is the case that would catch a regression: it
draws the same string at four sub-pixel offsets and they must come out identical.

The grid is the **offscreen's**, which is the display's real resolution, so on a `HiDPI`
screen this still leaves half a logical pixel of movement for an animation to use. Nothing is
lost by snapping: there is no sub-pixel detail in this renderer, since nothing filters, so a
draw moves by at most half a device pixel and gains an exact texel mapping. Meshes are
**not** snapped: `RectArray` maps texture-coordinate corners per vertex rather than sampling
across a scaled quad, and the world would jitter against the camera if it were.

`TextureAtlas::new` packs left to right **in key order**, which is why `KeyType: Ord`.
Packing in `HashMap` iteration order gives a different layout on every launch, since Rust
randomises it per process — the same class of bug that made `GameModData.resources` a
`BTreeMap`. The atlas is one row: as wide as the surfaces laid end to end, as tall as the
tallest.

#### Text

`Font` cuts a 16x16 atlas into one `Surface` and one `Texture` per ascii value.
`get_text_size` and `create_text_surface` share one walk, `Font::layout`, which is what keeps
measuring and rasterising from drifting apart — a test asserts they agree. `render_text` is
the odd one out: it draws **one line** straight from the glyph textures and honours neither
`\n` nor a width limit, so anything that might wrap goes through `create_text_surface` and a
`Texture`, which is what `Sprite` does.

**`get_text_size` is an advance width**, and it has to be: `TextInput` places its cursor by
measuring the text before it, so measuring a prefix must land exactly where the next glyph is
drawn. `Font::advance` is the one rule for how far the pen moves, shared by `layout` and
`render_text` so the two cannot drift; sampling the width before adding a space's extra gap
put the cursor two pixels left of whatever followed a space.

**A width limit breaks between words, not inside them.** `layout` measures the whole word at
its first character and moves all of it down together; only a word too wide for a line of its
own falls back to breaking at whichever character overruns, which is what keeps a narrow limit
from wrapping forever. The space a wrap steps over is neither drawn nor counted — it would
indent the line it lands on and push the block past the limit it just wrapped to — and both of
those apply *only* when there is a limit, so measuring a prefix (which `TextInput` does, always
without one) stays additive. The only text in the game that wraps is `ChoiceMenu`'s title, which
is where an error message is shown.

**The space is the only character whose advance is not its glyph's width**, and only in a
proportional font: trimming leaves its glyph empty, so it gets `SPACE_WIDTH` of its own. A
mono font's space was already padded out to the common width by `load_surfaces`, and adding
`SPACE_WIDTH` on top made the space two pixels wider than every other character — which is the
one thing a mono font promises not to do, and the server console is a column of timestamps
drawn in it.

`TextInput`'s view into a value too long for the box centres on the cursor while the field is
selected, and shows the tail when it is not. Nothing in this toolkit clips, so anything let off
the left edge does not disappear — it goes on painting a white bar over the widget beside it.
That is why `cursor_rect` clips itself to the widget: a *selection* is as wide as the text it
covers, which the box has no obligation to be. Centring rather than pulling the view back just
far enough is what makes both directions behave: with the cursor's moving end held against the
left edge, everything shift-and-right-arrow selected was scrolled off behind it. The geometry
is `view_offset` / `visible_text_rect` / `cursor_rect`, kept out of `render_inner` so that a
`Font::new_headless` is all a test needs to drive it.

Several older UI pieces predate the `UiElement` trait and are hand-rolled — the `//TODO make
this a UI element` comments in `client/game/chat.rs`, `pause_menu.rs`, `debug_menu.rs`,
`inventory.rs`, `respawn_screen.rs` mark them. Converting one is a good self-contained task.

#### Composites worth knowing before writing a menu

- **`ui::ListPage`** is a scrolling list between a title bar and a button bar: it owns the
  two bars, the scrollable, the row layout and the top bar's fade-in. A row implements
  `ui::ListRow` - a height, a position it accepts, and whether it is hoverable. The world
  selector and the server selector are both this, and were each half of it written separately
  until they were merged.
- **`ui::Menu` / `ui::MenuStack`** is a stack of screens where the top one is live: it gets
  the events and the updates, `open_menu` pushes a successor, `should_close` pops, and
  whatever a pop reveals is told it has focus.
- **`ui::DockNode`** is a binary split-pane layout of named panes, with `area_at_path`
  turning a path through it into a fraction of the window. The server ui's module layout is
  this plus its own editing.

### Timing

**Every clock in the game is a `libraries/timing` type**, and there is one accumulator behind
all of them. There used to be five hand-rolled ones, and two had already been fixed for the
same class of bug months apart — an `i32` of milliseconds that overflowed after 24.8 days and
an `f32` ledger that stalled after four hours.

- Server: fixed 20 TPS (`tps_limit`), with a `FixedStep::new(5)` inside `update()` driving
  player and entity physics.
- Client: renders as fast as allowed (vsync / fps limit configurable), with the same 5 ms
  `FixedStep` for simulation and a `FrameStats` for the debug menu's numbers.
- **The two `FixedStep` constructors are the catch-up policy, and picking the wrong one is a
  bug.** `new` owes every step, because a simulation that skips one has silently run slower
  and nothing downstream can tell. `for_animation` caps the backlog at `MAX_CATCHUP_FRAMES`,
  because frames nobody saw are worth nothing — the pause menu's buttons are built when the
  world loads and first drawn when the player opens it, which after an hour of play was 3.6
  million animation steps on one frame. Every widget that animates owns a `for_animation` one.
- `Interval` is the third rule: a set of things sharing one simulated clock, each at its own
  rate, and **a missed one is not owed**. That is the opposite of `FixedStep::new`, and it is
  what each liquid type's `flow_time` runs on.
- `DeltaTimer::tick` answers `None` on its first call, because the time since construction is
  however long starting up took rather than a frame anybody rendered. That is exactly the
  server's "skip this update".
- Physics constants live in `shared/entities/entities.rs` and `shared/players.rs`. The
  `/ 200.0` divisors there are the 5 ms tick expressed as a fraction of a second.
- The fps limit is an **average**, not a per-frame cap: `timing::FrameLimiter` keeps a ledger
  of what the frames so far should have taken against what they did, so an overrun is made up
  by the next frames. The debt is capped at one frame, because otherwise a stall — a world
  loading, a laptop waking, a breakpoint — bought that many frames of uncapped rendering
  afterwards.

**The frame's first 10 ms are a budget, and it is easy to spend by accident.**
`core_client.rs` starts a `timing::Budget::of_ms(10)` at the top of its loop and passes it to
`walls.rs` and `lights.rs`, which rebuild chunk meshes only while `budget.has_time_left()`.
The limit travels with the clock rather than being a literal rewritten at each check — it used
to be a bare `&Instant` and a `< 10`, and `blocks.rs` still has its copy commented out. That is the whole mechanism keeping the frame rate up while a
world loads. Anything slow *before* `walls.render` starves it, and the failure mode is not a
crash but a world that draws its blocks immediately and takes minutes to finish its walls and
lighting. Two things have done this already, both from the winit port: querying the window
size per call, and pumping the event loop at the top of the frame instead of at the end of
`update_window`. Measure `frame_budget.elapsed()` at `walls.render` if chunk loading ever
looks slow — healthy is under 0.1 ms.

Client-side prediction: the client simulates its own player and periodically sends
`PlayerPositionPacketToServer`; the server accepts it if within a tolerance of 2.0 blocks,
otherwise force-corrects (`server/server_core/players.rs`).

**A singleplayer client watches its own server's flag** (`run_game`'s `server_alive`, which is
`PrivateWorld`'s `server_running`). A welcome that is not coming is otherwise indistinguishable
from one that is slow: the client would wait forever on a port whose server has died, or that
something else on the machine happens to be holding. Multiplayer passes `None` — a remote
server's health is not this process's to know. Whatever kills the join, singleplayer now ends on
a `ChoiceMenu` naming the error, the same way `start_multiplayer.rs` always has; it used to
`println!` and drop the player back on the menu with nothing said.

**The join draws its own frames, because nothing else is drawing.** `run_game` is called from
inside the menu loop's `open_menu`, so that loop is stopped for the whole of the join — waiting
for the handshake, unpacking the welcome packets, initialising mods, allocating the light grid,
loading resources. It used to draw nothing at all for the duration and sleep 1ms at a time,
which on a freshly generated world is seconds of a window the system reports as not responding,
and the close button did nothing until the world had finished loading. `JoinScreen` in
`core_client.rs` is the answer: a `MenuBack` plus the menus' own `LoadingScreen`, one frame per
phase, with the phase named. Anything long added to the join belongs behind a `frame` call with
a name on it, and the wait for the handshake is a frame per iteration rather than a sleep —
`update_window` sleeps out the rest of the frame's share of the clock anyway.

### Persistence

A world save is a fixed header followed by `postcard(HashMap<String, Vec<u8>>)` with keys
`blocks`, `walls`, `liquids`, `players`. Blocks, walls and liquids are additionally
snap-compressed. Written to
`server_data/server.world` relative to the process CWD. Client settings are JSON at
`<data_dir>/Terralistic/settings.txt`.

The header is `b"TERRAWLD"` then `WORLD_SAVE_VERSION` as a little endian `u32`, and it is
deliberately **outside** anything a serializer wrote. Versions 1 and 2 kept the version
*inside* the encoded map, which works right until the encoding is the thing that changed —
and then the version cannot be read either. Both format migrations hit exactly that: the
check was unreachable and the player got a decode error instead of being told their world
was old. `read_world_header` runs before the body is touched, so a world this build cannot
read is now named rather than guessed at.

The liquid grid is two bytes a cell and is written whether or not there is any liquid in it,
which on the default world is about 0.5 MB of a 1.6 MB save. That is the price of a dense
`Vec` grid, and it is the right shape for a world with an ocean in it — but it is worth
knowing that a dry world pays it too.

**The header versions the container, not the contents.** Any change to the shape of
`BlocksData`, `SavedPlayerData` or the wall and liquid equivalents still breaks existing
worlds silently unless you bump `WORLD_SAVE_VERSION` by hand — that is what it is for.
Adding the liquid grid took it to 4; `Grid` carrying its own size took it to 5.

### Build pipeline

`build_main.rs` runs before every build and does two things:

1. `compile_resource_pack(resources/ → Build/Resources/)` — converts PNG to the custom
   `.opa` format (raw serialized `gfx::Surface`), copies everything else.
2. `compile_mod(base_game/)` — concatenates the Lua, minifies it with darklua, bundles
   resources, postcard + snap, writes `base_game/base_game.mod`.

`base_game/base_game.mod` and `Build/Resources/*` are `include_bytes!`-ed into the binary but
are **not** committed — `.gitignore` carries `Build/` and `**/*.mod`, so every checkout builds
its own. The only `.opa` files in git are the 46 golden images. That means a change to the
serialization format costs nothing here, but it does mean the goldens have to be converted:
they are `snap(postcard(Surface))` too.

The `.mod` build is reproducible: identical sources produce identical bytes. It used not to
be, because resources were serialized from a `HashMap` whose iteration order Rust randomises
per process, so the committed artifact changed on every single build. `GameModData.resources`
is a `BTreeMap` to keep that stable — don't change it back.

`Template_*.png` files get expanded at build time into 16-frame connected-texture atlases
(`process_template` in `build_project/compile_mod.rs`) and lose the prefix in the output.

## Integration tests

`integration_tests/` holds the tests that span subsystems, as opposed to the `tests.rs`
files that cover one module each. They are ordinary `#[test]`s in the same binary —
the crate is binary-only, so a `tests/` directory would have nothing to link against.

| File | Covers |
|---|---|
| `harness.rs` | Shared machinery: temp worlds, free ports, drivers for a real server and client |
| `networking.rs` | The wire protocol over a loopback socket, both halves at once |
| `client_server.rs` | A real client joining a real `Server`: world sync, chat, commands, breaking blocks, persistence |
| `server_lifecycle.rs` | `start`/`update`/`stop`, and world generation with the real biomes |
| `world_persistence.rs` | Saving and loading, including the version and corruption paths |
| `mods.rs` | The committed `.mod` artifact through lua into the rust registries |

Things worth knowing before adding one:

- **Small worlds, not generated ones.** `TestServer::start_on_small_world` writes a save
  first, so the server takes the load path and skips generation entirely. Use
  `start_on_generated_world` only when generation is the thing under test; a generated
  world needs a height above ~250 or the terrain runs off the top and there is no sky.
- **Ports come from `free_port`**, which counts rather than asking the OS for port 0 —
  two tests probing one after the other can otherwise be handed the same port, and then
  one test's client joins the other test's server.
- **Wait for the listener, by asking it.** Both networking layers bind on a background
  thread, so `wait_until_listening` is what makes a connection immediately afterwards
  reliable. It takes a predicate backed by `ServerNetworking::is_listening`, an atomic the
  binding thread sets, and **must not go back to probing the port**. The obvious check —
  "has the port stopped being bindable" — has to *bind* the port to find out, which is the
  very collision it is watching for: polling every millisecond it regularly won the race,
  the server's own `listen` then failed with `AddrInUse`, and since that error only surfaces
  through a later `update()` the probe waited out the full 30 s timeout for a listener that
  no longer existed. That was a 1-in-10 flake over the whole integration suite, landing on
  whichever test lost the toss — which is why it never looked like it belonged to any of
  them. `SO_REUSEADDR` does not save you here: both sides set it, and it waives `TIME_WAIT`,
  not a live listener.
- **One lock at a time.** `Server::get_blocks()` and friends are `#[cfg(test)]` accessors
  that take the server's mutex. Two calls in one expression deadlock against yourself.
- **Let the clock run.** Anything that advances over time takes whole milliseconds of
  measured frame length, which truncates to zero in a tight loop — `update_slowly` exists
  for that.
- A few types are re-exported `#[cfg(test)]` from `server_core` and `client::game` purely
  so these tests can reach them. Keep that list short.

## Gotchas

- **A setting's id is a handle, not a row.** `Settings::register_setting` hands out numbers
  from a counter that never reuses one, and the in-game lights toggle is registered when a
  world loads and removed when it closes — so it is a higher id on every world opened.
  `settings_menu` used to place each row at `id * row height` and the lights toggle drifted a
  row further down the screen each time; it lays out by position in the sorted list instead.
- **`build_main.rs` declares its own narrow module tree.** It names individual leaf files
  (`libraries/graphics/{color,position,surface}.rs`, `libraries/scripting/module_data.rs`)
  rather than `pub mod graphics;` / `pub mod scripting;`, so the build script does not compile
  the game's dependency tree — naming `scripting` would pull in rlua. Module *paths* must
  still match `main.rs`, because those files refer to themselves as
  `crate::libraries::graphics` and `crate::libraries::scripting`. If the build script ever
  needs another type, prefer moving that type to a dependency-free leaf module over
  widening the declaration.
- **`libraries/grid` has two coordinate schemes that disagree, on purpose.** `Grid` indexes
  cells column-major (`x * height + y`); `Chunks` indexes chunks row-major (`x + y * width`).
  Both are internally consistent, and swapping either to match the other silently
  reinterprets every index computed with the old one. Don't "fix" one in isolation — there is
  a test pinning the disagreement. `CHUNK_SIZE` stays in `shared/mod.rs`, because the
  partition size is the game's tuning choice and `Chunks` takes it as an argument.
- `server/server_core/core_server.rs` keeps the tick counters as `Server` fields, so two
  servers in one process no longer share them — the integration tests rely on that. The
  channel back to the ui is still process-global (`UI_EVENT_SENDER`), because
  `print_to_console` is a free function with no `Server` to reach through: the first
  non-`None` sender wins and later ones are ignored.
- **The singleplayer loading screen closes on an empty status string and nothing else**, so a
  private server thread that ends without emptying it strands the player there for good. That is
  a `Drop` guard in `private_world.rs` rather than an `if result.is_err()`, because a panic in
  the server thread is exactly the case that leaves nobody to clear it. The guard clears the
  running flag *before* the text: the other order lets the menu see the loading screen finish
  while the server still looks alive, and try to join a world that is not there.
- **`Server::run` shuts the networking thread down on every exit path**, which is why it wraps
  `run_until_stopped` instead of being it. `start` binds the port early and everything after it
  can fail; a `?` that returned while that thread ran left it holding the port with the receiving
  end of its channel already dropped, so it accepted clients it could tell nobody about
  (`Failed to send NewConnectionEvent: sending on a closed channel`) and they waited for a
  welcome forever. The port stayed taken for the rest of the process too, so every world opened
  afterwards failed to bind and hung the same way. The world is deliberately *not* saved on that
  path — the error may be about the world.
- Server `Blocks`/`Walls`/`Items`/`Entities` are behind `Arc<Mutex<_>>` and accessed via
  `get_blocks()` etc. that lock. Holding two of these at once in the wrong order is a
  deadlock waiting to happen; the codebase currently always takes them one at a time.
- `PoisonError::into_inner` is used everywhere instead of propagating lock poisoning. That's
  deliberate — follow it for consistency.
- Test files follow one convention: a `tests.rs` beside the module it covers, declared in
  the neighbouring `mod.rs`, containing `#![cfg(test)] mod tests { .. }`. Every module
  directory has one. Tests that span subsystems live in `integration_tests/` instead — see
  below.
- Tests never need a graphics context. UI tests drive real widgets through
  `gfx::HeadlessContext` — see the `UiContext` section above. Nothing in the suite opens a
  window, so it all runs on CI's headless Ubuntu.
- `gfx::FloatPos` and `gfx::FloatSize` compare with a 0.0001 tolerance and **deliberately do
  not implement `Hash`.** Quantising them to hash would break the hash/eq contract: two
  values that compare equal land in different buckets. Don't add it to make one a map key —
  round to integers first.
- **A registry id is a handle, not a position.** `libraries/registry` hands them out in
  registration order and never reuses one, and `RegistryId::index` is an implementation
  detail of the lookup. Don't lay anything out by it — that is exactly the bug that made the
  lights toggle drift down the settings menu. `index` returns `Option` so the "undefined"
  value every one of these newtypes has resolves to nothing rather than to whatever entry a
  negative index casts to.
- **`timing::FixedStep` counts absolute milliseconds since construction**, and is 64-bit
  because as `i32`/`u32` it overflowed after 24.8 days of uptime and every animation in the
  game stopped for good. See *Timing* for the catch-up policies.
- **A click is a press and a release on the same widget** — `gfx::ClickTracker`, which
  `Button` and `Toggle` both hold. Half a click does nothing in either direction. The
  remembered press is deliberately *not* cleared by the release that consumes it, because
  several menus hold their buttons as sub-elements **and** dispatch to them again from
  `on_event_inner` — so one release reaches a button twice, fires its closure twice, and the
  menu reads the second answer. Worth knowing before adding a button with a side effect in its
  closure. A menu that dispatches to buttons itself has to forward *every* event, not only the
  release; `choice_menu` did the latter and its buttons went dead the moment the press started
  mattering. A widget that reacts to a bare release is the bug, not the pattern: `settings_menu`
  decided for itself from `Toggle::hovered` and so flipped a setting for any release that
  happened to land on a toggle. It now reads `toggle.toggled` back instead — the toggle is a
  sub-element, so it has already answered the same event by the time the menu sees it.
- **`gfx::Texture::load_from_bytes` turns a bad asset into an empty texture, not an error.**
  A missing or corrupt `.opa` should leave a hole in the screen rather than take the process
  down, and it is one fallback in one place - the ten call sites that spelled this out
  invented three different fallback sizes between them.
- **A `gfx::Surface` is checked against its own size when it is deserialized**, because
  nothing downstream re-checks: `GpuDevice::create_texture` tells wgpu the texture is
  `get_size()` big and hands it `pixels`, and a mismatch is a wgpu validation error, which is
  a panic. Surfaces come out of `.mod` files, which are ordinary files on disk, so a
  hand-edited or truncated one used to take the client down on load.
- **Liquid levels are whole numbers, and that is what makes water settle.** The first
  implementation held them as `f32` and compared `level as i32`, so two cells that were
  never quite equal averaged each other forever — on a server, a cell that sends a change
  packet twenty times a second and never stops. See the liquids section above before
  reaching for floats there.
- Server `Liquids` is created empty and sized in `Server::start`, *after* the world is
  loaded or generated, because the grid has to be exactly as big as the block grid. A save
  whose liquid grid disagrees is discarded rather than read.
- Two `create` methods differ: `Blocks::create` fills the grid with air, `Walls::create`
  fills it with `WallId::undefined()`, so reading a wall before setting one errors. Only
  `create_from_wall_ids` calls it, and that overwrites everything, so the game never hits
  it — but calling `Walls::create` directly is a trap. The fill is now an argument to
  `Grid::filled` at both call sites, so it is at least visible rather than implied.
- Serialization saves the block, wall and liquid **grids**, not the type registries. On load
  the server registers types from mods first, then deserializes, so ids line up. Tests have
  to do the same.

## CI

`.github/workflows/rust.yml` runs on Ubuntu, for pushes to `master`/`beta-5`, for pull
requests against any branch, and on manual dispatch. It checks, in order:
`cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, and a
release build. All four are blocking, so a new clippy warning fails the build — prefer a
targeted `#[allow]` with a reason over relaxing the flag.

**No system packages are installed.** winit's X11 and Wayland backends are loaded at runtime
through `x11-dl` and `wayland-dlopen`, so nothing is needed to build, and the test suite never
opens a window so nothing is needed to run either.

Not covered: macOS and Windows are never built. That gap matters for the renderer — the
backend is written to be portable and the goldens are written to be backend-independent, but
only Metal has actually run them. Adding a macOS job is the obvious next step, and it is
the only way the golden images would ever be checked in CI at all.
