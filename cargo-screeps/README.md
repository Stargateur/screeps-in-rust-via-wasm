cargo-screeps
=============

[![crates.io version badge][cratesio-badge]][crate]

Build tool for deploying Rust WASM code to Screeps game servers.

Best used with [`screeps-game-api`].

This implements type-safe WASM bindings to the Screeps in-game API.

This is not fully tested, but feel free to use! Issues are welcome.

---

# Build Options

### `build`:

1. during build stage, shell out to https://github.com/koute/cargo-web for actual building the rust code.
2. strip off header / surrounding function `cargo-web` generates for a generic JS file, since we know we're deploying to node
3. append call to `__initialize` function which cargo-web generates, using `require('compiled')` to get the WASM bytes
4. create `target/main.js` containing processed JS and copy WASM file to `target/compiled.wasm`

### `upload`:

1. run build.
2. read `target/*.js` and `target/*.wasm`, keeping track of filenames
3. read `screeps.toml` for upload options
4. upload all read files to server, using filenames as the filenames on the server.

### `copy`:

1. run build.
2. copy compiled main file and WASM file (default `main.js` and `compiled.wasm`) from `target/` to `<destination directory>/<branch name>/`
3. if pruning is enabled, delete all other files in `<destination directory>/<branch name>/`

### `deploy`:

1. run build.
2. run `upload` or `copy` depending on the `default_deploy_mode` configuration option.

### `check`:

1. perform type checking / lifetime checking without compiling code
  - runs `cargo web --check --target=wasm32-unknown-unknown` which is fairly similar to
    `cargo check`.

# Configuration Options

## No namespace

- `default_deploy_mode`: what `cargo screeps deploy` does: use `"copy"` or `"upload"`

## `[upload]`

This configures options specific to the `cargo screeps upload` deploy mode.

- `username`: your Screeps username or email
- `password`: your Screeps password. For private servers set a password using [screepsmod-auth].
- `branch`: the branch on the server to upload files to
- `ptr`: if true, upload to the "ptr" realm
- `hostname`: the hostname to upload to. For example, `screeps.com` or `localhost` or `server1.screepsplu.us`
- `ssl`: whether to connect to the server using ssl. Should be false for private servers
- `port`: port to connect to server with. Should generally be `21025` for private servers

## `[copy]`

This configures options specific to the `cargo screeps copy` deploy mode.

- `destination`: the directory to copy files into. can be relative to `screeps.toml` or absolute
- `branch`: the "branch" to copy into. This is a subdirectory of `destination` which the js/wasm files will be copied into.
- `prune`: if true, any files in the destination directory which were not just copied will be deleted after copying.

## `[build]`

This configures general build options.

- `output_js_file`: the javascript file to export bindings and bootstrapping as (default `"main.js"`)
- `output_wasm_file`: the WASM file to rename compile WASM to (default `"compiled.wasm"`)
- `initialize_header_file`: a file containing the JavaScript for starting the WASM instance. See
  [overriding the default initialization header](#overriding-the-default-initialization-header)

## Overriding the default initialization header

`cargo-screeps` tries to make a reasonable `main.js` file to load the WASM. However, it's pretty
basic, and you might find you want to do some things in JavaScript before loading the WASM module.

Luckily, you can override this initialization! Set `build.initialize_header_file` to a file containing the JavaScript initialization code.

The code will have access to two utility functions, `wasm_fetch_module_bytes` and `wasm_create_stdweb_vars`:

```js
/**
 * Fetches WASM bytes for the wasm module.
 *
 * These should be given to `new WebAssembly.Module` to instantiate a WebAssembly module.
 */
function wasm_fetch_module_bytes() { /* ... */ }

/**
 * Creates the stdweb wrapper for a module instance.
 *
 * It has two properties, `imports` and `initialize`. `imports` should be passed as the second
 * argument to `new WebAssembly.Instance` to provide the WASM module with imports, and `initialize`
 * should be called on the resulting WebAssembly instance.
 *
 * Calling `initialize` will finish associating the imports with the wasm module, and will call the
 * rust module's main function.
 */
function wasm_create_stdweb_vars() { /* ... */ }
```

When you override the initialization, you will need to perform it yourself. Here is the default initialzation header for reference:

```js
"use strict";
let wasm_module = null;

function wasm_initialize() {
    if (wasm_module == null) {
        let wasm_bytes = wasm_fetch_module_bytes();
        wasm_module = new WebAssembly.Module(wasm_bytes);
    }
    let stdweb_vars = wasm_create_stdweb_vars();
    let wasm_instance = new WebAssembly.Instance(wasm_module, stdweb_vars.imports);
    stdweb_vars.initialize(wasm_instance);
}

module.exports.loop = wasm_initialize;
```

# Updating `cargo screeps`

As it parses the unstable output of `cargo-web`, `cargo-screeps` is highly dependent on `cargo-web`
version. It is recommended to upgrade both together.

Installing a version of `cargo-web` newer than what `cargo-screeps` supports will cause it to
output an error on build. If this happens, please create an issue on this repository and we can
update `cargo-screeps`. Updating it is simple, but it needs to be done every time `cargo-web`
changes the output format, and we might not realize that has happened.

After updating, you'll want to do a full `cargo clean` to remove any old artifacts which were built
using the older version of `cargo-web`.

```sh
cargo install -f cargo-web
cargo install -f cargo-screeps
cargo clean
cargo screeps build
```

[cratesio-badge]: http://meritbadge.herokuapp.com/cargo-screeps
[crate]: https://crates.io/crates/cargo-screeps/
[`screeps-game-api`]: https://github.com/daboross/screeps-in-rust-via-wasm/
[screepsmod-auth]: https://www.npmjs.com/package/screepsmod-auth

