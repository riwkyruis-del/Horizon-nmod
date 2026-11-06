# Dreadfall — experimental Horizon native mod

A first, deliberately small slice of a brutal dark-fantasy survival overhaul for
**Vasysik Horizon Android 0.7.7**. This is a source-mixin `.hnmod`, not an Android
binary or a sandboxed `.hmod`.

> **Compatibility warning:** The official 0.7.7 Android release explicitly does
> not compile or load arbitrary `.hnmod` mods. The private installer mentioned by
> the requester is undocumented, so its compatibility cannot be established.
> This bundle has **not been compiled into the game or tested on a device**.
> An installer would have to apply its mixins to the exact Android source tree
> and build a compatible client. Merely importing the archive into the stock
> app will not make it work. Native mods run with the game's privileges; only
> use a trusted compiler/loader and review source before running it.

## Gameplay prototype

- **The Dread Hunt:** in the Overworld at night, a survival player in block light
  below 8 can attract one named vanilla zombie, skeleton, or spider every 30
  seconds. Light sources protect the player. Only safe, loaded, nearby floor
  positions are considered, and a cap of six nearby zombies, skeletons, and
  spiders limits extra spawns. Peaceful difficulty blocks monster spawns.
- **Darkness-bound ore:** survival players cannot break iron, gold, diamond or
  emerald ore in deep darkness. Bring a light source (block light at least 5),
  or mine in daylight with sky light at least 8. Other ores remain vanilla.
- **Soul-fire offering:** right-click a soul campfire while holding a plain bone
  to exchange three plain bones for a golden carrot, provided inventory has
  room. This is a prototype ritual using existing vanilla content.

These are **named vanilla mobs**, not new mob types. There are no new items,
biomes, dimensions, textures or world-generation changes yet. They are deferred
until the custom loader's capabilities and build process can be verified. The
mod is designed for single-player; multiplayer and coexistence with other mods
are not tested. Back up worlds before experimentation; spawned mobs can persist
in saved chunks even after removing the mod.

## Artifact and source

`releases/dreadfall-0.1.0.hnmod` is a ZIP containing the standard source-only
`horizon-native.json` and `mixins/dreadfall/` bundle. The archive contains **no
executable**. It pins Vasysik's v0.7.7 commit
`80700a6f26afe53a368032d8b28bd364f7c3e7e4` and horizon-mixins
`d9c83d7096b7ea5b83678da2efdfd6161f74997e` (the revision in the official native
mod template at the time of writing). The latter pairing is unverified; a
build-capable tool may require another mixins version. Stock desktop build tools
may assume the *upstream* Horizon repository and not find the Android fork's
commit. They also do not automatically produce an Android APK.

To recreate the archive and run the packaging checks (Python 3.11+):

```sh
python3 tools/package.py
python3 -m unittest discover -s tests -v
```

The packaging checks validate archive structure and reproducibility, **not**
Rust compilation, mixin application, in-game behavior, or device compatibility.
Rust unit tests in `runtime.rs` also require a properly staged Horizon build.
To validate further, a developer must apply the bundle to the pinned Android
source using a compatible horizon-mixins tool, compile the Android client, and
run a fresh-world test on an arm64 device. No installation steps for the private
loader can be given until its interface or a working example is available.

## References

- [Vasysik 0.7.7 release and limitations](https://github.com/Vasysik/horizon-android/releases/tag/v0.7.7)
- [Native mod documentation at 0.7.7](https://github.com/Vasysik/horizon-android/blob/v0.7.7/docs/native-mods.md)
- [Official source-mixin template](https://github.com/horizon-rust/horizon-native-mod-template)
