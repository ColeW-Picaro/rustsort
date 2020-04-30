# rustsort
Quicksort implementations and comparisons in rust

### Building and Running
Ensure unstable features are enabled. You might need to reinstall rust with rustup.
If you don't have rust installed or need to reinstall, run `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

#### Cargo:
build and run: `cargo run -- -C link-args=-Wl,-zstack-size=838860` to build and run.

just build: `cargo rustc -- -C link-args=-Wl,-zstack-size=838860`.


