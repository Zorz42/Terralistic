# Login / account system (disabled)

The account system is commented out, not deleted. It talked to `home.susko.si`, which does
not resolve, so every client start printed:

```
error getting tls client:
failed to lookup address information: nodename nor servname provided, or not known
```

The client still ran — the failure was handled — but the title screen showed a permanently
red status dot next to a `Login` button that could not do anything.

## What is switched off

| Where | What |
|---|---|
| `client/menus/title_screen_renderer.rs` | the `Login` button, the connection status dot, the `TlsClient` that drove them, its event and render blocks, and `get_tls_status_color` |
| `client/menus/mod.rs` | `mod login;` and the `LoginMenu` re-export |
| `client/menus/secondary_menu.rs` | secondary menu arm `0`, which opened `LoginMenu` |
| `client/game/mod.rs` | `pub mod tls_client;` |
| `shared/mod.rs` | `pub mod tls_client;` |
| `Cargo.toml` | `rustls`, `webpki-roots`, `rustls-pki-types`, `kvptree` — only the account client used them |

No source files were deleted. `shared/tls_client.rs`, `client/game/tls_client.rs` and
`client/menus/login/` are all still there, just not compiled.

The other menu indices did not shift: singleplayer is still 1, multiplayer 2, settings 3,
mods 4, exit `usize::MAX`. Only index 0 is gone, and `open_secondary_menu` already had a
`_` arm, so an unexpected 0 just prints "menu doesn't exist".

## Restoring it

Uncomment the blocks marked `// login disabled` in the files above, plus the four
dependencies in `Cargo.toml`. Each site says what it is and points here.

Two things to know before you do:

- `shared/tls_client.rs` hardcodes `const ADDR: &str = "home.susko.si"` and
  `const PORT: u16 = 28603`. Point those at a host that exists, or the same failure returns.
- `client/menus/login/tests.rs` comes back with it. That is why `cargo test` reports 70
  rather than 71 while login is off — the email validation tests live in that module.

## Effect

The title screen now renders with no login affordance at all, and the client produces no
output on startup. Verified by launching the client and screenshotting the window: the
menu shows Singleplayer, Multiplayer, Settings, Mods and Exit, with the top left corner
empty.

Dropping `rustls` and its two companions also takes a meaningful chunk out of the build.
