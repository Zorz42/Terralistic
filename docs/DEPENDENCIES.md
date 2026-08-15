# Dependency status

Everything is on its latest usable version. What follows is the part that is not obvious from
`Cargo.toml`: where a crate is unmaintained, and what a migration would cost.

## rlua — deprecated in favour of mlua

`rlua` 0.20 is its newest release, so there is nothing to bump, but its own README says:

> *rlua is now deprecated in favour of mlua* [...] `rlua` is now a thin transitional wrapper
> around [`mlua`](https://github.com/mlua-rs/mlua); it is recommended to use mlua directly for
> new projects and to migrate to it when convenient.

It pins `mlua ^0.9.5` while mlua is on 0.12, so the mod system runs through a deprecated shim
over a Lua binding several minor versions behind. Migrating means touching
`libraries/scripting/` and the four `mod_interface.rs` files — a change to the mod API surface,
not a dependency bump.

## postcard — chosen after bincode was withdrawn

The project used bincode until its maintainers published 3.0.0 as a tombstone: no code, a
README saying development has ceased, and a `lib.rs` containing only
`compile_error!("https://xkcd.com/2347/")`. 2.0.1 was the last release with code in it, and it
was already unmaintained.

postcard was picked because it is serde-driven — so the 59 types deriving `Serialize` did not
change — and because it has a written, versioned wire specification with stability promised
since 1.0. That matters most for world saves, which carry no description of their own layout.
`bitcode` is smaller and faster but lists format stability as a *non-goal*.

`libraries/serialization/` exists to make a future move a one-file change: every call site goes
through it, which is what made this one nearly a one-file change.

## Notes

- `noise` 0.9 is the latest and still depends on `rand` 0.8, so both 0.8 and 0.10 are in the
  tree. Upstream's constraint, not something to fix here.
- `gl`, `sdl2`, `sdl2-sys` and `raw-window-handle` are gone: the OpenGL backend was replaced by
  `wgpu` 30 and the window by `winit` 0.30, which wgpu can build a surface from directly. That
  removed the three target-specific `[dependencies]` blocks and the last system library the
  build needed.
- `kvptree`, `rustls`, `webpki-roots` and `rustls-pki-types` went with the account client that
  was their only user. `kvptree` in particular had 1513 all-time downloads and one release,
  which was a real supply-chain question the game no longer has to answer.
- `[build-dependencies]` is deliberately narrow — 8 crates, not the game's tree. See the
  `build_main.rs` gotcha in `CLAUDE.md`.
