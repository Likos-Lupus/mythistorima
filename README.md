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

## Phase 1 Week 2 - Document Tree

Week 2 upgrades the Phase 0 single chapter list into a novel document tree.

Implemented:

- Volume / chapter / scene document types
- `parent_id` hierarchy
- Root volume and root chapter creation
- Child creation: volume → chapter, chapter → scene
- Same-level creation
- Rename
- Delete confirmation with cascading child deletion
- Same-level move up / move down
- Status switching: draft / writing / done
- Rust `move_document` and `update_document_status` commands

Acceptance path:

```txt
Open a project
→ Create a volume
→ Create a chapter under the volume
→ Create two scenes under the chapter
→ Rename one node
→ Move a scene up or down
→ Change a chapter status to writing
→ Delete a parent node and confirm child nodes are removed
```

## Phase 1 Week 3

Week 3 adds editor-focused writing features:

- Paragraph IDs for every paragraph in the Tiptap document.
- Current-chapter find and replace.
- Focus mode that hides the document tree and status panel.
- Writing timer and today writing stats backed by `writing_sessions`.
- Current document target character count stored in `documents.metadata_json`.
- Editor settings for font size, line height, and paper width stored in `app_settings`.

Recommended verification:

```bash
bun install
bun run typecheck
cd src-tauri
cargo check
cd ..
bun run tauri:dev
```

## Phase 1 Week 4 - Setting Cards

Week 4 adds the setting-card workspace for novel worldbuilding.

Implemented:

- Rust `CardDto`, `CreateCardInput`, `UpdateCardInput`, and `CardReferenceDto`.
- Rust card service and Tauri commands:
    - `create_card`
    - `update_card`
    - `delete_card`
    - `get_card`
    - `list_cards`
    - `search_cards`
    - `list_card_references`
- Setting card types:
    - character
    - location
    - concept
- Card workspace UI:
    - `CardWorkspace`
    - `CardList`
    - `CardEditor`
    - `CardTypeTabs`
- Alias editing via comma or newline-separated input.
- Basic `fields_json` templates for each card type.
- Reserved card reference list area for Week 5 `@` insertion integration.
- Card saves update the Phase 1 `search_index` foundation.

Recommended verification:

```bash
bun install
bun run typecheck
cd src-tauri
cargo check
cd ..
bun run tauri:dev
```

Acceptance path:

```txt
Open a project
→ Switch the left workspace mode from Writing to Setting
→ Create a character card
→ Edit name, aliases, description, role, motivation, and notes
→ Create a location card
→ Create a concept card
→ Filter by type
→ Restart the app and confirm all cards persist
```
