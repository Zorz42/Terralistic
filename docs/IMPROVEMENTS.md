# Improvement catalogue

A full read of the repository at `7e3d0505` produced a list of defects and gaps. **All of it is
done**, so what the list said about the broken code is no longer worth carrying — the fixes and
the reasoning behind them live in `CLAUDE.md` and in the code. What is kept here is the record
of what was found, and the limits that are still real.

## Fixed

| Found | Was |
|---|---|
| Empty command panic (#168) | Any player could crash the server by typing `/` in chat |
| Server bind address | Bound loopback only, so multiplayer could not work. Now `BindAddress` per server; singleplayer stays on loopback |
| Unbreakable blocks (#175) | Three functions disagreed about what `break_time: None` meant |
| Spawn point (#176) | Scanned the whole world height and let the last write win |
| `ChunkTracker` sentinel (#174) | `0` meant both "not tracked" and a real elapsed time |
| `Lights::create` arithmetic (#174) | `w / 16 * h / 16` associates as `((w / 16) * h) / 16` |
| Headless server shutdown (#173) | No signal handling, so a dedicated server never saved |
| `static mut` (#169) | Five of them in the server core: UB, and shared between two `Server`s |
| Network thread `expect` (#170) | One malformed frame killed the server's networking thread |
| Build-dependency bloat | 21 build-dependencies compiling the whole game tree, down to 8 |
| CI gaps (#171) | No clippy, no fmt, no release build, `master` only |
| Dead `liquids` module | 271 lines never constructed — rewritten and wired into all three layers |
| Empty test stubs | Four `mod tests {}` files with nothing in them |
| Duplication and 107 clippy warnings (#178) | Cleared, and CI now runs `-D warnings` |
| Protocol and format robustness | No version handshake, no world save version |

Found while doing the above:

| Found | Was |
|---|---|
| `base_game.mod` was not reproducible | `HashMap` iteration order changed the committed artifact on every build |
| The blit was skipped on any OS that is not windows/macos/linux | Gone with the wgpu port |
| `TextInput` panicked on a right-to-left selection | Collapsing the cursor kept the larger half, which no longer existed |
| The client hung forever when the server was down or refused it | `Connected(_, false)` and `Disconnected` were both ignored while welcoming |
| World generation ignored its seed for the biome layout | `generate_biome_ids` drew from `rand::random` |
| `despawn_entity` left every despawned id resolvable | The id maps only ever grew |
| Joining a newly generated world froze the window for tens of seconds | 5.4M queued `BlockUpdateEvent`s, nothing drawn during the join, no way to cancel, and an orphaned networking thread that held the port for the rest of the process |
| A big blur in a fullscreen window cost 7.3 ms of GPU a frame | Every gaussian pass ran at full resolution over the whole frame |

## Still open

**Content that does not exist yet.** No bucket item, so a player cannot carry liquid
(`places_liquid` alongside `places_block` is the obvious shape). Liquids neither drown anyone
nor interact with lighting.

**Six UI elements predate the `UiElement` trait** and are hand-rolled — the `//TODO` comments in
`client/game/chat.rs`, `debug_menu.rs`, `inventory.rs`, `pause_menu.rs`, `respawn_screen.rs` and
`client/menus/settings_menu.rs`. Converting one is a good self-contained task, and anything
implementing the trait gets its layout and event handling tested headlessly for free.

**World generation is not reproducible from a seed end to end.** The rust half is; each biome's
lua `generator_function` decorates with `math.random`, which lua seeds per state. Making that
reproducible means seeding lua from the world seed, which changes what mods can rely on — a
decision, not a fix. The determinism test compares walls, which lua never touches.

**What a widget draws is not in `cargo test`.** `render_inner` needs a real device, so it is
covered by the golden-image suite instead; the three menus that build GPU resources from an
event handler take the `as_graphics_context()` escape hatch and those branches are skipped
headlessly. `HeadlessContext` answers the same questions as the real context but is not proof
that the window reports what we think it does.

**CI builds Ubuntu only**, on `master` and `beta-5` plus pull requests. macOS and Windows are
never built, which matters most for the renderer: it is written to be portable and the goldens
are written to be backend-independent, but only Metal has run them. A macOS job is the only way
the golden images would ever be checked in CI at all.

**There is no authentication of any kind**, and `TypeId` is not stable across compiler versions,
so client and server must be built by the same rustc. The version handshake turns the second
into a clear refusal rather than silence; the first is a design decision nobody has made.
