# Novel Reader

Tauri 2 desktop novel reader. `.txt` (auto encoding + chapter split) + `.epub` (native TOC).

## Dev

```bash
pnpm install
pnpm tauri dev
```

## Build

```bash
pnpm tauri build
```

Artifacts land in `src-tauri/target/release/bundle/`.

## Features

- **Library** — import / list / delete `.txt` / `.epub`
- **Reader** — TOC nav, auto-save progress (book+chapter+scroll), font-size slider, light/dark theme
- **Search** — FTS5 full-text across all books, click to jump

## Architecture

- Backend (Rust / Tauri 2): file I/O, SQLite + FTS5 persistence, txt encoding detection (chardetng), chapter regex, epub parsing. All commands exposed via `#[tauri::command]`.
- Frontend (Vue 3 TS / Pinia / Naive UI / vue-router): three views (Library / Reader / Search) — no business logic, only `invoke` wrappers.

## Tests

```bash
cd src-tauri && cargo test
```

14 unit tests cover encoding detection, chapter splitting (CN/EN/preface), epub parsing, and all DB modules (books/chapters/progress/settings/FTS5 search).
