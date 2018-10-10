Screeps in Rust (via WASM)
==========================

Tools for creating [Screeps] AIs written in Rust.


`screeps-game-api` is a Rust binding to the JavaScript APIs for programs compiled to WASM using
[`stdweb`].

`cargo screeps` is a binary program which wraps `cargo web` and lets one directly upload Rust WASM
code to the Screeps server.

These two tools go together well, but do not depend on eachother. `cargo-screeps` can compile and
upload any screeps WASM project buildable with `stdweb`'s `cargo-web`, and `screeps-game-api` is
usable in any project built with `cargo-web`.

Writing Screeps code in Rust can be nice, but it can also be annoying. If you have tips, tricks, or
other things you'd like to share, make an issue! We need to write more documentation, and if we have
enough ideas, we can start an mdbook for this repository.

- [cargo screeps usage docs](cargo-screeps/README.md)
- [screeps-game-api api docs](https://docs.rs/screeps-game-api/)
- [screeps-starter-rust example project](https://github.com/daboross/screeps-starter-rust/)

---

Here's a quickstart for what you *need* to get going. More documentation will be made in the future.

```sh
# clone:

git clone https://github.com/daboross/screeps-starter-rust.git
cd screeps-starter-rust

# cli dependencies:

cargo install cargo-screeps
cargo install cargo-web

# configure for uploading:

cp example-screeps.toml screeps.toml
nano screeps.toml

# build tool:

cargo screeps --help
```

[screeps]: https://screeps.com/
[`stdweb`]: https://github.com/koute/stdweb
