# Mythistorima

Mythistorima is a local desktop writing tool for novel authors. The current implementation in this repository is **Phase
0 - Local Writing Loop**: validating a complete local writing workflow using Nuxt 4 + Tauri 2 + Rust + SQLite + Tiptap.

## Current Phase 0 Capabilities

- Nuxt 4 frontend skeleton
- Nuxt UI / Tailwind CSS paper-style base interface
- Tauri 2 desktop shell
- Rust command/service/db layered architecture
- SQLite local database initialization with idempotent migrations
- Project creation and recent project list
- Auto-creation of "Chapter 1" when starting a new project
- Chapter creation and chapter switching
- Tiptap rich text editor
- 1-second debounce auto-save
- Word count for current chapter and total project word count
- Restore project, chapters, and content after closing and reopening

## Environment Requirements

- Bun
- Rust stable
- System dependencies required by Tauri 2

## Install Dependencies

```bash
bun install
```

## Development Launch

Standard Nuxt dev server:

```bash
bun run dev
```

Tauri desktop dev mode:

```bash
bun run tauri:dev
```

## Build

```bash
bun run generate
bun run tauri:build
```

`src-tauri/tauri.conf.json` is configured with:

- `devUrl`: `http://localhost:3000`
- `beforeDevCommand`: `bun run dev`
- `beforeBuildCommand`: `bun run generate`
- `frontendDist`: `../dist`

## Database Location

Phase 0 uses a single SQLite database:

```txt
AppData/Mythistorima/mythistorima.sqlite
```

The actual path can be viewed on the home screen status.

## Phase 0 Acceptance Path

```txt
Launch Mythistorima
→ Create a new project "Test Novel"
→ "Chapter 1" is auto-generated
→ Write content in the rich text editor
→ Auto-saves after 1 second
→ Close the application
→ Reopen the application
→ Project, chapters, and content are fully restored
```

## Not Yet Included

- Setting cards
- @Character / #Location quick insert
- Outlines, mind maps, Mermaid diagrams
- Foreshadowing / notes / to-do anchors
- docx / epub / pdf / Pixiv export
- Proofreading rules
- Cloud sync and project encryption

These belong to Phase 1 or later stages.
