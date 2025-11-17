# Crab Shell 🦀

Crab Shell is a Rust-native take on the wonderful [Turtle](https://github.com/Gabriel439/Haskell-Turtle-Library) library from the Haskell ecosystem.  It provides:

- A lazy `Shell<T>` iterator with ergonomic combinators (`map`, `filter_map`, `chunks`, `windows`, `product`, …) for building expressive data pipelines.
- Optional parallel chunk processing when enabling the `parallel` feature (`Shell::chunk_map_parallel` is backed by `rayon`).
- A thin wrapper over `std::process::Command` and multi-stage `Pipeline`s that makes it easy to spawn commands, capture/stream both stdout and stderr, and redirect output to files.
- Battery-included filesystem helpers (directory walking, globbing with metadata, copying/moving trees, temporary files, directory watchers, etc.).
- A small set of environment helpers and a convenient prelude for `use crab_shell::prelude::*` workflows.

Everything is synchronous and depends only on the standard library plus `glob`/`tempfile` for a few utilities.

## Quick start

```rust
use crab_shell::prelude::*;

fn main() -> crab_shell::Result<()> {
    // Iterate over the first few files in the workspace
    for entry in walk_files(".")?.take(5) {
        println!("{}", entry.path.display());
    }

    // Build a streaming command just like Turtle
    sh("echo hello from crab-shell")
        .stream_lines()?
        .for_each(|line| println!("stdout: {line:?}"));

    // Capture stderr while also teeing it to a file
    let temp = temp_file("stderr-log")?;
    let cmd = sh("echo warn 1>&2");
    let stderr_lines: crab_shell::Result<Vec<_>> =
        cmd.stream_stderr()?.collect();
    cmd.tee_stderr(&temp)?;
    println!("stderr: {stderr_lines:?} (written to {})", temp.display());

    // Glob with metadata and filter by extension
    for entry in filter_extension(glob_entries("src/**/*.rs")?, "rs") {
        println!("source file: {}", entry.path.display());
    }

    Ok(())
}
```

## Additional examples

- `examples/glob_walk.rs` demonstrates metadata globbing with extension filtering.
- `examples/find_large.rs` filters globbed files by extension and size.
- `examples/pipeline_stream.rs` shows how to compose commands in a pipeline while streaming stdout incrementally.
- `examples/watch.rs` polls a directory for file creations, modifications, and removals.
- `examples/watch_trigger.rs` triggers a command (here `echo`) when file events occur.
- `examples/watch_build.rs` mimics a rebuild-on-change workflow using the watch helper.
- `examples/chunk_map_parallel.rs` demonstrates how to enable the `parallel` feature and process data chunks concurrently.
- `examples/async_run.rs` uses the `async` feature to run commands via `tokio`.
- `examples/async_pipeline.rs` streams pipeline output asynchronously (requires `--features async`).
- `examples/async_watch.rs` polls directories asynchronously using `watch_async`.
- `examples/async_watch_stream.rs` consumes `watch_async_stream` as a `Stream`.
- `examples/watch_debounce.rs` shows how to filter noisy change events using `debounce_watch`.
- `examples/watch_glob.rs` filters watch events by glob pattern.

### Watch helpers

Directory watching is implemented via lightweight polling. Use `Watcher` for manual control, call `watch(path, interval, iterations)` to receive a `Shell<WatchEvent>`, chain utilities like `debounce_watch` + `watch_glob`, or call `watch_filtered` for a one-stop helper. With `--features async`, use `watch_async`, `watch_async_stream`, or `watch_filtered_async`.

Run an example with `cargo run --example glob_walk`.

## Features

- `parallel`: enables `Shell::chunk_map_parallel`, which uses `rayon` to process chunks concurrently. Activate with `cargo run --features parallel --example chunk_map_parallel` (or set `default-features = false` and opt-in within your own `Cargo.toml`). Run tests with `cargo test --features parallel` if you want to cover the parallel-only unit test.
- `async`: enables async command helpers (`Command::output_async`, `run_async`, etc.) built on `tokio`. Run `cargo run --features async --example async_run` or `cargo test --features async` to exercise them.

### Watch helpers

Directory watching is implemented via lightweight polling. Use `Watcher` for manual control, call `watch(path, interval, iterations)` to receive a `Shell<WatchEvent>`, or use `watch_async_stream` (with `--features async`) to obtain a `Stream<Item = Result<WatchEvent>>`. See `examples/watch*.rs` for sync recipes and `examples/async_watch*.rs` for async ones.

## Status

The crate is intentionally small and focused.  Contributions that keep the API ergonomic and dependency-light are welcome!  Ideas for future work include async variants, higher-level glob combinators, and more batteries for interacting with common system utilities.
