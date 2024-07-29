<p align="center">
    <img src="/assets/logo.png" alt="LLM-Tools Logo" height="200">
</p>

# Leptos Starter Template

This is a template for use with the [Leptos](https://github.com/leptos-rs/leptos) web framework and the [cargo-leptos](https://github.com/akesson/cargo-leptos) tool.

## Creating your template repo

If you don't have `cargo-leptos` installed you can install it with

`cargo install cargo-leptos --locked`

Then run

`cargo leptos new --git https://github.com/wzzju/leptos-app-template`

to generate a new project template (you will be prompted to enter a project name).

`cd {{project-name}}`

to go to your newly created project.

Of course, you should explore around the project structure, but the best place to start with your application code is in `src/app.rs`.

## Running your project

`cargo leptos watch`
By default, you can access your local project at `http://localhost:3000`

## Installing Additional Tools

By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
3. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
4. `npm install -g sass` - install `dart-sass` (should be optional in future)

## Executing a Server on a Remote Machine Without the Toolchain
After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The deployment directory structure should be:
```text
{{project-name}}
site/
```
Set the following environment variables (updating for your project as needed):
```sh
export LEPTOS_OUTPUT_NAME="{{project-name}}"
export LEPTOS_SITE_ROOT="site"
export LEPTOS_SITE_PKG_DIR="pkg"
export LEPTOS_SITE_ADDR="127.0.0.1:3000"
export LEPTOS_RELOAD_PORT="3001"
```
Finally, run the server binary.

## Notes about CSR and Trunk:
Although it is not recommended, you can also run your project without server integration using the feature `csr` and `trunk serve`:

`trunk serve --open --features csr`

This may be useful for integrating external tools which require a static site, e.g. `tauri`.

## Licensing

This template itself is released under the Unlicense. You should replace the LICENSE for your own application with an appropriate license if you plan to release it publicly.
