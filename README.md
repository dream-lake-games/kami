# TODO

## Building

For WASM:

```sh
# From root
trunk build
```

To try out ^this build^ locally:
```sh
# The -o just opens index
npx http-server dist -o
```

To upload to itch:

- Go into dist/index.html
- Change `/dfsklsdflksjdf` to `./dfsklsdflksjdf` everywhere

Then zip dist and upload.

NOTE: To play around with WASM, you can `cargo run --target wasm32-unknown-unknown`.

 /Users/mork/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bevy_core_pipeline-0.15.1/src/oit/resolve/mod.rs:59 OrderIndependentTransparencyPlugin not loaded. GPU lacks support: DownlevelFlags::FRAGMENT_WRITABLE_STORAGE.
pirate_jam_16-72824889b46af8e7.js:1260 WARN /Users/mork/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bevy_pbr-0.15.1/src/ssao/mod.rs:92 ScreenSpaceAmbientOcclusionPlugin not loaded. GPU lacks support: TextureFormat::R16Float does not support TextureUsages::STORAGE_BINDING.