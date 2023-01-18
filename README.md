# Gittite

![Gittite screenshots](./screenshot/gittite.png)

Git gui client based on tauri framework.

This was started as a toy project to study tauri and rust,
but I'm going to continue this for personal use.

---

## Project Roadmap

### v0.2.x

- [ ] branch
  - [ ] branch detail - ahead/behind
  - [ ] remote branch
- [x] stash
  - [x] handle IPC response in tagged json format for rust enum 
  - [ ] ui - show file list and diff
  - [ ] ui - list/apply/drop 
- [ ] unitest
- [ ] amend
- [ ] repository preference
- [x] open repository page
- [x] devtools menu
- [x] popup menu with right button click

### v0.3.0

- [ ] tag
- [ ] preference
- [ ] diff
  - [ ] option
  - [ ] full file
  - [ ] diff between commit/branch

### next

- [ ] push, fetch
- [ ] authentication
- [ ] merge
- [ ] file browser
- [ ] zoom in/out - if tauri support
- [ ] badge on stage/history
- [ ] branch merge graph
- [ ] frequently use case

### v0.2.0

- [x] branch operation
- [x] repository information
- [x] changes and history layout
- [x] refresh button
- [x] infinite scroll for big log list

## Compile

### Prerequisites

- https://tauri.app/v1/guides/getting-started/prerequisites

### Compile

- `cargo install tauri-cli`
- `cargo tauri dev` : Start app in dev mode
- `yarn tauri dev`

### Build Package

- `yarn tauri build`
- `cargo tauri build`

### Lint

- `yarn eslint`
- `yarn type-check`

## Log trace

- `export RUST_LOG=trace`
- `cargo test -- --nocapture`

## Reference

- tauri
- git2-rs
- gitui
- vue
- quasar
