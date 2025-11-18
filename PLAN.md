## Qshr Improvement Plan

### Shell Core Improvements

- Implement iterator traits (`ExactSizeIterator`, `DoubleEndedIterator`) where applicable so adapters compose better.
### Docs & Examples

- Add a “patterns” section for pipelines, parallel chunking, watchers, etc., so users can copy/paste common scripts.

### Testing

- Add high-level integration tests that exercise command/pipeline/watcher combos (possibly via `cargo test -- --ignored`).
### Performance/Robustness

- Add a non-blocking watcher option that spawns `notify` on its own thread, allowing `watch()` to be dropped cleanly.
- Cache glob metadata in `glob_entries` to avoid repeated `fs::metadata` calls when traversing large trees.
