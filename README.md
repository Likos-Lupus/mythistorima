# Mythistorima

Mythistorima is a local desktop writing workspace for novel authors. The current implementation in this repository is *
*Phase 1 - Usable Novel Workspace MVP**: a Nuxt 4 + Tauri 2 + Rust + SQLite + Tiptap application for local-first novel
drafting, worldbuilding, notes, search, export, backup and theme customization.

## Current Phase 1 Capabilities

- Nuxt 4 frontend with Nuxt UI / Tailwind paper-style interface.
- Tauri 2 desktop shell with Rust command/service/db layered architecture.
- SQLite local database with versioned migrations.
- Project creation, recent project list, editing, delete confirmation and project dashboard.
- Novel document tree with volume / chapter / scene hierarchy.
- Document create, rename, delete, move up/down and status updates.
- Tiptap rich text editor with autosave.
- Paragraph IDs for paragraph-level anchors.
- Current-document find and replace.
- Focus mode.
- Writing timer, today writing stats and current document targets.
- Setting cards for characters, locations and concepts.
- `@` setting references in the editor with hover previews.
- Automatic extraction of setting references on save.
- Creative notes: memo, todo, foreshadow, issue and idea.
- Notes can bind to projects, documents and paragraphs.
- Search workspace for documents, setting cards and notes.
- Search index rebuild.
- Export to TXT / Markdown / HTML / project package.
- Import TXT / Markdown / HTML as new chapters.
- Manual, startup and scheduled local SQLite backups.
- Paper / light / dark themes.
- Font family, font size, line height, page width and autosave interval settings.
- Language setting foundation for `zh-CN` and `en`.
- Shared empty states and error banners.

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

Phase 1 uses a single SQLite database:

```txt
AppData/Mythistorima/mythistorima.sqlite
```

The actual path can be viewed on the home screen status.

## Phase 1 Acceptance Path

```txt
Launch Mythistorima
→ Create a new project
→ Open the project dashboard and edit title / author / description / targets
→ Create a volume, chapters and scenes
→ Write rich text in the editor
→ Use find / replace and focus mode
→ Create character / location / concept cards
→ Insert a setting card in the editor with @
→ Hover the setting reference and see its preview
→ Add a memo / todo / foreshadow note to a document or paragraph
→ Search body text, cards and notes
→ Export TXT / Markdown / HTML
→ Export a project package
→ Import TXT / Markdown / HTML as a chapter
→ Create a manual backup and confirm startup/scheduled backup behavior
→ Switch themes and editor settings
→ Close and reopen the app
→ Confirm project, documents, cards, notes, references and settings persist
```

## Verification

```bash
bun install
bun run typecheck
cd src-tauri
cargo check
cd ..
bun run tauri:dev
```

## Phase 1 Documentation

- `PHASE1.md`: original Phase 1 implementation plan.
- `PHASE1_FINAL_AUDIT.md`: final completion audit and acceptance checklist.

## Later Phases

The following remain intentionally out of Phase 1 and should be handled in Phase 2 or later:

- docx / epub / pdf / Pixiv-specific export templates.
- Mermaid and mind-map outline views.
- Advanced proofreading rules.
- Cloud sync, collaboration and project encryption.
- Plugin system.

## Phase 2 Week 1

The data baseline and workspace skeleton for Phase 2 have been integrated:

- `MIGRATION_0003` creates tables related to outlines, relationship diagrams, timelines, foreshadowing threads,
  character appearance statistics, export templates, and proofreading rules.
- The project page navigation has been upgraded to the grouping "Writing / Materials / Review / Output".
- Placeholder entries are now provided for the Outline, Kanban, Timeline, Relationship Diagram, Statistics,
  Foreshadowing, and Proofreading workspaces.
- The search index rebuild has reserved scope for the new Phase 2 objects.

In the following Week 2, concrete interactions will be implemented starting with outline tree and chapter binding.

## Phase 2 Week 2 – Outline Tree & Chapter Binding

The first version of the independent outline system is complete: it supports Plot, Conflict, Twist, Event, Subplot,
Theme, and Note node types; multi-level trees, status filtering, move up/down, and summary editing are available. Nodes
can be bound to chapters or scenes and then jump to the writing workspace. Outline nodes are synced into the search
index.

## Phase 2 Week 3 – Outline Board & Mermaid View

The outline system now has a planning board: outline nodes are grouped into Planned, Drafting, and Done columns, and
their status can be changed either by dragging cards between columns or by using quick action buttons. The board keeps
chapter / scene links visible and can jump back to the writing workspace. A Mermaid flowchart is generated from the
outline tree, with a readable in-app preview and a copy button for the Mermaid source text.

## Phase 2 Week 4 – Setting Card Expansion & Relation Graph

The setting system has been upgraded from a card list into a worldbuilding graph. Setting cards now support Character,
Location, Organization, Item, Event, and Concept types. The backend exposes `create_card_relation`,
`update_card_relation`, `delete_card_relation`, `list_card_relations`, and `list_card_graph` commands backed by the
`card_relations` table.

The Relationship Diagram workspace is now active: cards render as graph nodes, relations render as clickable edges,
node type and relation type filters are available, clicking a node opens the corresponding setting card, and clicking an
edge opens the relation editor so the relation type, direction, and description can be updated.

## Phase 2 Week 5 – Timeline System

The Timeline workspace is now active. The backend exposes `create_timeline_event`, `update_timeline_event`,
`delete_timeline_event`, `list_timeline_events`, `attach_card_to_timeline_event`,
`detach_card_from_timeline_event`, `list_timeline_event_cards`, and `reorder_timeline_event` commands backed by
`timeline_events` and `timeline_event_cards`. Timeline events are synced into the full-text search index.

Authors can create story events, set a human-readable `starts_at_label`, adjust `sort_key`, bind an event to a
chapter / scene, assign a location card, and attach participating characters, organizations, items, events, or concepts.
The workspace provides list and lane views, participant and location filters, quick reordering, and jumps to related
chapters or setting cards.
