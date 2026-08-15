# Extracting libraries

What came out of the game into `libraries/`, and what deliberately did not. The rule itself is
in `libraries/README.md`; this is the record behind it.

## What was extracted

| Library | What it replaced |
|---|---|
| `grid` | Four hand-rolled grids over one `WorldMap`, in four styles |
| `registry` | 20 near-identical register/get/by-name functions across six registries |
| `timing` | Five hand-rolled clocks, plus a bare `Instant` used as a budget |
| `net` | Two networking halves duplicating thread, timer and retry loop |
| `scripting` | `mod_manager.rs` + `mod_data.rs`, and six lua handle impls |
| `ui` | The widget half of `graphics`, plus `MenuStack`, `ListPage`, `Dock` |
| `config` | `client/settings.rs`, verbatim |
| `container_file` | The save header and four `snap(postcard(x))` pairs |
| `log` | Two verbatim copies of `format_timestamp`, and a global sender |
| `procgen` | `world_generator/noise.rs` and the biome walk's weighted pick |
| `testing` | `free_port` / `wait_until` / `TempDir`, written twice |

Game code went from 16,271 lines to ~13,900, and the login system — 861 lines commented out of
the module tree that did not compile — is gone entirely.

**The point was never the line count.** What changed is that ~3,500 lines stopped being game
code an agent has to hold in its head alongside the game, and became library code behind a
stated contract it can use without reading. The lines that genuinely disappeared were
duplication between the four grid owners, the four registries and the two networking halves —
and duplication is where two copies drift.

## Not extracted, and why

- **`scheduler`** (a shared "wake your neighbours" update set). The two users do not line up.
  `Liquids` drains its whole `BTreeSet` each step and re-inserts what still has somewhere to
  go, with the iteration order load-bearing. `Lights` never drains: it keeps a per-cell flag
  *and a per-chunk count*, and the client reads that count to decide whether a chunk's light
  mesh needs rebuilding — which a set cannot answer without a scan. One name over two
  mechanisms would fit neither.
- **`aabb`** (the swept collision against a solid grid). Physics must agree bit-for-bit between
  client and server, and it does because both call `shared/`. Adding a boundary to a
  correctness-critical path is a poor trade for the lines it would save.

## What must never move

- Block, wall, liquid, item and tool **semantics** — breaking rules, drops, recipes, inventory
  rules, connected-texture rules, multiblock growth.
- The **handshake** and every packet type. `WelcomeCompletePacket` is not a transport concept.
- **World generation policy** — the biome graph's meaning, the order of the six steps,
  `FLOODED_COLUMN_FRACTION`, the fill-from-the-sky rule.
- **Physics constants** and the 5 ms tick's meaning.
- Anything whose name has to mention a block, a player, a world or a mod to make sense.

The test when unsure: *could this library's doc comment be written without naming anything from
the game?* If not, it is game logic wearing a library's clothes.

## Enforcement — still open

"No game imports in a library" holds today and nothing checks it. A `#[test]` that walks
`libraries/` and fails on any line matching `crate::(shared|client|server)` is ten lines and
catches the regression on the commit that introduces it.

Making each library a real crate in a workspace would let the compiler enforce it, but costs
more than it currently returns: the ~136 crate-wide clippy lints would move to
`[workspace.lints]`, every `#[cfg(test)]` helper that crosses a module boundary today
(`HeadlessContext`, `Font::new_headless`, `Texture::new_sized`, `TextInput::new_headless`, the
`#[cfg(test)]` accessors on `Server`) would need a `testing` feature instead, and the
golden-image harness would still have to be reachable from the binary crate. Worth revisiting
only once the boundaries have stopped moving.
