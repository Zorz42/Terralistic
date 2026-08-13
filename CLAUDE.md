# Terralistic — working notes

A Terraria-like 2D sandbox game in Rust. Single binary that runs as client, server, or
server-with-GUI. Rendering is wgpu through a hand-written UI toolkit, with winit for the
window and input.
Game content (blocks, items, walls, biomes, recipes, commands) lives in **Lua mods**, not
in Rust — `base_game` is itself a mod.

~21k lines of Rust across 140 files, plus ~7.6k lines of tests in 26 more. Small enough to
read in full; do that before large refactors.

## Commands

```bash
cargo run                 # client
cargo run --release       # client, release
cargo build --profile dist # what you ship: release + LTO, 4.63 MB vs 5.49 MB
cargo run -- server       # server with GUI
cargo run -- server nogui # headless server
cargo run -- version      # print version
cargo test                # 423 tests, all should pass
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
| `client/menus/` | Title screen, world selector, settings, multiplayer, login |
| `libraries/graphics/` | The UI toolkit + wgpu renderer (no game knowledge) |
| `libraries/events/` | Type-erased event queue (`Box<dyn Any>` + downcast) |
| `base_game/` | Lua mod: all actual game content |
| `resources/` | Client-side assets (fonts, icons, UI textures) |
| `integration_tests/` | Tests that drive several subsystems against each other |

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
where `id` is an FNV hash of `TypeId::of::<T>()` and `data` is postcard.

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

Connection handshake: client sends `NamePacket` → server replies `WelcomeCompletePacket` →
client stops "welcoming" mode and starts the normal receive loop. Welcome-phase packets are
buffered into `pre_events` in `client/game/core_client.rs` and drained before the game starts.

Both networking modules run a dedicated thread with an mpsc channel pair in and out. The
main loop only ever touches the channels, never the socket.

### Mod system (Lua via rlua)

`shared/mod_manager.rs`. Each `GameMod` owns its own `Lua` state, its minified source, and
a `HashMap<String, Vec<u8>>` of resources. Resource keys use `:` as separator, e.g.
`blocks:dirt.opa`.

The on-disk `.mod` format is `snap(postcard(GameModData))` — see `shared/mod_data.rs`, which
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

Default world is 4400×1200, seed 423657 — `Server::world_size` and `Server::world_seed`,
defaulted in `Server::new` from the `DEFAULT_WORLD_*` constants. They are fields rather
than literals inside `start` so a test can ask for a world small enough to assert about.

Generation is reproducible from the seed on the rust side, but **not end to end**: each
biome's lua `generator_function` decorates with `math.random`, which lua seeds per state.
So terrain and walls repeat for a given seed and the trees do not.

### Rendering and UI

`libraries/graphics` is a self-contained immediate-mode-ish toolkit. It has no game
knowledge — treat it as a vendored library.

The UI contract is `UiElement` / `BaseUiElement` in `ui_element.rs`. You implement
`UiElement` — `get_container` is the only required method, everything else defaults — and
`BaseUiElement` is blanket-implemented and handles recursing into children. **Implement
`UiElement`, call `BaseUiElement`.** The one default worth knowing is `get_sub_elements*`,
which returns nothing: a leaf writes neither, and **an element with children has to override
both**, or it is laid out and drawn while its children are not.

Layout is `Container` + `Orientation` (`TOP_LEFT`, `CENTER`, …): a child positions itself
relative to a parent container by orientation plus offset. Theme constants (colors, `SPACING`,
`BLUR`, `TRANSPARENCY`, and the widget defaults) are in `theme.rs` — use them rather than
literals. Every fade and slide in the toolkit is `gfx::approach(value, target, smooth_factor,
epsilon)` per ready `AnimationTimer` frame; don't hand-roll another one. The epsilon is not
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
  pass, the gaussian ping-pongs between the two offscreen textures, and the rest resumes
  with `LoadOp::Load`.
- **The clear gets a render pass of its own**, on the front texture, before any of that.
  Folding it into the first pass instead is wrong precisely when the frame opens with a
  blur: that pass writes the *back* texture, so the clear lands there, the front keeps the
  previous frame, and the blur samples it straight back in. Only the golden harness ever
  asks for a clear — the game draws an opaque background over the whole window — and
  `blur_over_a_cleared_frame` is the case that pins it.

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
can be dropped while a command still refers to it. That is not a corner case: `login.rs`
builds a text texture inside `render_inner` and drops it there, every world chunk replaces
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
`#[cfg(feature = "render-tests")]` hook: wall-clock animations (`AnimationTimer::freeze`,
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

### Timing

- Server: fixed 20 TPS (`tps_limit`), with a 5 ms accumulator inside `update()` driving
  player and entity physics.
- Client: renders as fast as allowed (vsync / fps limit configurable), with the same 5 ms
  accumulator via `FramerateMeasurer::has_5ms_passed()` for simulation.
- Physics constants live in `shared/entities/entities.rs` and `shared/players.rs`. The
  `/ 200.0` divisors there are the 5 ms tick expressed as a fraction of a second.
- The fps limit is an **average**, not a per-frame cap: `renderer.rs`'s `FrameLimiter` keeps a
  ledger of what the frames so far should have taken against what they did, so an overrun is
  made up by the next frames. The debt is capped at one frame, because otherwise a stall — a
  world loading, a laptop waking, a breakpoint — bought that many frames of uncapped rendering
  afterwards.

**The frame's first 10 ms are a budget, and it is easy to spend by accident.**
`core_client.rs` starts a `frame_timer` at the top of its loop and passes it to
`walls.rs` and `lights.rs`, which rebuild chunk meshes only while
`frame_timer.elapsed() < 10ms`. That is the whole mechanism keeping the frame rate up while a
world loads. Anything slow *before* `walls.render` starves it, and the failure mode is not a
crash but a world that draws its blocks immediately and takes minutes to finish its walls and
lighting. Two things have done this already, both from the winit port: querying the window
size per call, and pumping the event loop at the top of the frame instead of at the end of
`update_window`. Measure `frame_timer.elapsed()` at `walls.render` if chunk loading ever
looks slow — healthy is under 0.1 ms.

Client-side prediction: the client simulates its own player and periodically sends
`PlayerPositionPacketToServer`; the server accepts it if within a tolerance of 2.0 blocks,
otherwise force-corrects (`server/server_core/players.rs`).

### Persistence

A world save is a fixed header followed by `postcard(HashMap<String, Vec<u8>>)` with keys
`blocks`, `walls`, `players`. Blocks and walls are additionally snap-compressed. Written to
`server_data/server.world` relative to the process CWD. Client settings are JSON at
`<data_dir>/Terralistic/settings.txt`.

The header is `b"TERRAWLD"` then `WORLD_SAVE_VERSION` as a little endian `u32`, and it is
deliberately **outside** anything a serializer wrote. Versions 1 and 2 kept the version
*inside* the encoded map, which works right until the encoding is the thing that changed —
and then the version cannot be read either. Both format migrations hit exactly that: the
check was unreachable and the player got a decode error instead of being told their world
was old. `read_world_header` runs before the body is touched, so a world this build cannot
read is now named rather than guessed at.

**The header versions the container, not the contents.** Any change to the shape of
`BlocksData`, `SavedPlayerData` or the wall equivalent still breaks existing worlds silently
unless you bump `WORLD_SAVE_VERSION` by hand — that is what it is for.

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
- **Wait for the listener.** Both networking layers bind on a background thread, so
  `wait_until_listening` is what makes a connection immediately afterwards reliable.
- **One lock at a time.** `Server::get_blocks()` and friends are `#[cfg(test)]` accessors
  that take the server's mutex. Two calls in one expression deadlock against yourself.
- **Let the clock run.** Anything that advances over time takes whole milliseconds of
  measured frame length, which truncates to zero in a tight loop — `update_slowly` exists
  for that.
- A few types are re-exported `#[cfg(test)]` from `server_core` and `client::game` purely
  so these tests can reach them. Keep that list short.

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
- `server/server_core/core_server.rs` keeps the tick counters as `Server` fields, so two
  servers in one process no longer share them — the integration tests rely on that. The
  channel back to the ui is still process-global (`UI_EVENT_SENDER`), because
  `print_to_console` is a free function with no `Server` to reach through: the first
  non-`None` sender wins and later ones are ignored.
- Server `Blocks`/`Walls`/`Items`/`Entities` are behind `Arc<Mutex<_>>` and accessed via
  `get_blocks()` etc. that lock. Holding two of these at once in the wrong order is a
  deadlock waiting to happen; the codebase currently always takes them one at a time.
- `PoisonError::into_inner` is used everywhere instead of propagating lock poisoning. That's
  deliberate — follow it for consistency.
- Test files follow one convention: a `tests.rs` beside the module it covers, declared in
  the neighbouring `mod.rs`, containing `#![cfg(test)] mod tests { .. }`. Every module
  directory now has one except `shared/liquids`. Tests that span subsystems live in
  `integration_tests/` instead — see below.
- Tests never need a graphics context. UI tests drive real widgets through
  `gfx::HeadlessContext` — see the `UiContext` section above. Nothing in the suite opens a
  window, so it all runs on CI's headless Ubuntu.
- `gfx::FloatPos` and `gfx::FloatSize` compare with a 0.0001 tolerance and **deliberately do
  not implement `Hash`.** Quantising them to hash would break the hash/eq contract: two
  values that compare equal land in different buckets. Don't add it to make one a map key —
  round to integers first.
- **`AnimationTimer` counts absolute milliseconds since construction**, and every widget
  that animates owns one. Two things follow. It is 64-bit because as `i32`/`u32` it
  overflowed after 24.8 days of uptime and every animation in the game stopped for good. And
  the frames it owes are owed for *elapsed* time, not for time anyone was looking, so it
  gives up after `MAX_CATCHUP_FRAMES` — the pause menu's buttons are built when the world
  loads and first drawn when the player opens it, which after an hour of play was 3.6 million
  animation steps on one frame.
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
- **A `gfx::Surface` is checked against its own size when it is deserialized**, because
  nothing downstream re-checks: `GpuDevice::create_texture` tells wgpu the texture is
  `get_size()` big and hands it `pixels`, and a mismatch is a wgpu validation error, which is
  a panic. Surfaces come out of `.mod` files, which are ordinary files on disk, so a
  hand-edited or truncated one used to take the client down on load.
- `shared/liquids/` is not just dead, it is **entirely commented out** — both `liquids.rs`
  and `liquid_type.rs` are one `/* .. */` block from first line to last, so the module
  compiles to nothing. Don't assume any of it works.
- Two `create` methods differ: `Blocks::create` fills the map with air, `Walls::create`
  fills it with `WallId::undefined()`, so reading a wall before setting one errors. Only
  `create_from_wall_ids` calls it, and that overwrites everything, so the game never hits
  it — but calling `Walls::create` directly is a trap.
- Serialization saves the block and wall **grids**, not the type registries. On load the
  server registers types from mods first, then deserializes, so ids line up. Tests have to
  do the same.

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
