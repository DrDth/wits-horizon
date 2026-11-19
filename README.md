# WITS (What is this structure?) for Horizon

A port of [WITS](https://github.com/TelepathicGrunt/WITS) by TelepathicGrunt to
[Horizon](https://github.com/horizon-rust/horizon), the Rust rewrite of Minecraft: Java Edition 26.3. Built
as a wasm mod from the original's 1.21 sources (1.3.1), the version All the Mods 10 ships.

- `/wits` lists the registry names of all structures whose start's box holds the block you stand in. Each name
  is copied to the clipboard on a click.
- `/witsop <dimension> <coordinates>` does the same for any position in any dimension. Operators only
  (permission level 2).

The mod is server side only: a server or a single-player world needs it, joining players don't.

## Differences from the original

- `/witsop` at a position whose chunk isn't loaded answers "That position is not loaded", as `/setblock` does there.
  The original loads the chunk first; a Horizon mod's reads load no chunks.

- Run from the console or a command block, the original also shows its answer to the operators in chat, the way
  vanilla echoes an admin's commands. The port answers the source only.
- `/wits` reads the block of the player who typed it, or 0 0 0 from the console. Behind `execute as`, the original
  reads the player the command runs as.

## Building

The mod needs `LevelRef::structures_at`, added to Horizon in
[horizon-rust/horizon#18505](https://github.com/horizon-rust/horizon/issues/18505). Until a Horizon release has
it, build the mod inside a Horizon checkout that does:

```sh
mkdir -p <horizon>/.local/dev-mods/wits
cp -R Cargo.toml horizon-mod.toml src LICENSE <horizon>/.local/dev-mods/wits/
cd <horizon>/.local/dev-mods/wits
cargo run --manifest-path ../../../Cargo.toml -p cargo-horizon-mod -- horizon-mod build --wasm
```

## License

MIT, as the original. Copyright (c) 2023 TelepathicGrunt; the Horizon port by DrDth. See [LICENSE](LICENSE).
