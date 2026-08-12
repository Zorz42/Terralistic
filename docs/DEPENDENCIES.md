# Dependency status

State after the update pass on 2026-08-11. Everything is on its latest usable version.
The two items under "Notable" are on their newest usable release but are unmaintained
upstream, which is worth knowing even though there is nothing to bump.

## Updated

| Crate | From | To | Notes |
|---|---|---|---|
| png | 0.17 | 0.18 | needs `BufRead`; `output_buffer_size` now returns `Option` |
| darklua | 0.18 | 0.19 | build script only |
| bincode | 1.3 | 2.0.1 | new API and a different encoding; see below |
| hecs | 0.10 | 0.11 | query iterators yield `Q::Item`, not `(Entity, Q::Item)` |
| rand | 0.8 | 0.10 | `RngCore` renamed to `Rng`; old `Rng` is now `RngExt` |
| everything else | | | moved within its existing range via `cargo update` |

Already at the newest release and left alone: `rlua` 0.20.1, `noise` 0.9.0,
`winres` 0.1.12, `kvptree` 0.1.0.

**Since this pass:** `gl` and `sdl2`/`sdl2-sys` are gone. The OpenGL backend was replaced by
`wgpu` 30, and the window and input by `winit` 0.30; `raw-window-handle` went with them,
because wgpu can build a surface straight from an `Arc<winit::window::Window>`. That also
removed the three target-specific `[dependencies]` blocks and the last system library the
build needed.

## Notable

### bincode — on 2.0.1; 3.0.0 is a tombstone and must not be used

The project is on **2.0.1**, the last release that contains code.

`bincode` 3.0.0 is the latest published version and it **cannot be used**. It ships no
code: its `src/lib.rs` is one line.

```rust
compile_error!("https://xkcd.com/2347/");
```

From its README:

> Bincode is now unmaintained. Due to a doxxing and harassment incident, development on
> bincode has ceased. No further releases will be published on crates.io.
>
> As crates.io [...] lacks the ability to mark a project as archive or remove the last
> maintainer, this final release is being published containing only this README, as well
> as a lib.rs containing only a compiler error, to inform potential users of the
> maintenance status of this crate.

So `Cargo.toml` pins `2.0`, and it should stay there. Bumping to 3 fails to resolve, then
fails to compile by design.

bincode is still unmaintained at 2.0.1, so a future move off it is worth considering; its
README suggests [`wincode`](https://crates.io/crates/wincode) as a compatible alternative.
`libraries/serialization.rs` exists partly to make that a one file change: every call site
goes through it, so swapping the backend does not mean touching 25 places again.

### rlua — deprecated in favour of mlua

`rlua` 0.20.1 is the newest release, so there is nothing to bump, but its own README says:

> *rlua is now deprecated in favour of mlua* [...] `rlua` is now a thin transitional
> wrapper around [`mlua`](https://github.com/mlua-rs/mlua); it is recommended to use mlua
> directly for new projects and to migrate to it when convenient.

It pins `mlua ^0.9.5`, while mlua is on 0.12.0. So the mod system is running through a
deprecated shim over a Lua binding that is several minor versions behind.

Migrating means touching `shared/mod_manager.rs` and the four `mod_interface.rs` files.
Worth doing, but it is a change to the mod API surface, not a dependency bump.

## Notes

- `noise` 0.9.0 is the latest and still depends on `rand` 0.8, so both 0.8 and 0.10 are in
  the tree. Upstream's constraint, not something to fix here.
- `kvptree` has 1513 all-time downloads and one release. It is only used by
  `shared/tls_client.rs`. Worth knowing if that code ever gets more load-bearing.
