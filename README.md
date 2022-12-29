# Gittite

![Gittite screenshots](./screenshot/gittite.png)

Git gui client based on tauri framework.

This was started as a toy project to study tauri and rust,
but I'm going to continue this for personal use.

---

## Project Roadmap

### v0.2.0

- [x] branch
  - [x] branch operation
  - [ ] branch detail
  - [ ] remote branch
- [ ] tag
  - [ ] handle IPC response in tagged json format for rust enum 
- [ ] unitest
- [ ] repository information
- [x] layout
- [ ] refresh with notify or periodically or button

### v0.3.0

- [ ] stash
- [ ] preference
- [ ] diff
  - [ ] option
  - [ ] full file
  - [ ] diff between commit/branch
- [ ] many log

### next

- [ ] push, fetch
- [ ] authentication
- [ ] merge
- [ ] file browser
- [ ] zoom in/out
- [ ] badge on stage/history
- [ ] branch merge graph
- [ ] git-flow
- [ ] frequently use case

## Compile

### Prerequisites

- https://tauri.app/v1/guides/getting-started/prerequisites

### Compile

- `cargo install tauri-cli`
- `cargo tauri dev` : Start app in dev mode
- `yarn tauri dev`
- `cargo tauri build`: Build

## Log trace

- `export RUST_LOG=trace`
- `cargo test -- --nocapture`

## Reference

- tauri
- git2-rs
- gitui
- vue
- quasar
