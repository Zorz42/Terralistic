# Dependency status

State after the update pass on 2026-08-11. Everything is on its latest usable version
except the two items under "Blocked", which need a decision rather than a version bump.

## Updated

| Crate | From | To | Notes |
|---|---|---|---|
| png | 0.17 | 0.18 | needs `BufRead`; `output_buffer_size` now returns `Option` |
| darklua | 0.18 | 0.19 | build script only |
| hecs | 0.10 | 0.11 | query iterators yield `Q::Item`, not `(Entity, Q::Item)` |
| rand | 0.8 | 0.10 | `RngCore` renamed to `Rng`; old `Rng` is now `RngExt` |
| everything else | | | moved within its existing range via `cargo update` |

Already at the newest release and left alone: `rlua` 0.20.1, `noise` 0.9.0, `gl` 0.14.0,
`winres` 0.1.12, `kvptree` 0.1.0, `sdl2` 0.38.0 (pinned with `=`).

## Blocked

### bincode — the newest version is a tombstone, do not upgrade

`bincode` 3.0.0 is the latest release and it **cannot be used**. It ships no code: its
`src/lib.rs` is one line.

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

So the project stays on **1.3**, which works. The real decision is not a version bump:

- The last release with actual code is **2.0.1** (March 2025). Moving 1.3 to 2.0.1 is a
  genuine API migration — 2.x replaced `serialize`/`deserialize` with
  `encode_to_vec`/`decode_from_slice` plus a config, and its default encoding is varint
  where 1.x was fixed-width, so it needs `bincode::config::legacy()` to keep the bytes
  compatible. That buys a newer version of a crate that is dead either way.
- bincode's own README points at [`wincode`](https://crates.io/crates/wincode) as a
  bincode-compatible alternative.

This matters more here than in most projects, because bincode defines three things at
once: the world save format, the network wire format, and the committed `base_game.mod`.
Any move needs the format held stable, or a version bump on all three — the save version
added in `shared/versions.rs` now makes at least the world side detectable.

My suggestion: skip 2.0.1. The format-compatibility work is the same whether the target is
2.0.1 or a maintained alternative, so doing 1.3 to 2.0.1 first means doing it twice.

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
