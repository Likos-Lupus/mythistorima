---
version: alpha
name: Mythistorima-design-analysis
description: >-
  Mythistorima is a local-first desktop writing workspace for novel authors. Its interface should feel like a quiet
  creative IDE: project chrome is precise, compact and keyboard-friendly; writing surfaces are calm and paper-like;
  story-planning panels are structured but not visually heavy. The brand atmosphere is literary, nocturnal and focused,
  with a restrained amber accent used for the active route, primary action, writing progress and selected story items.
  The UI must be built with Nuxt UI primitives wherever possible, while custom rendering is reserved for the editor
  canvas, graph/timeline canvases and tool-window orchestration.
colors:
  primary: '#f2b827'
  primary-active: '#d99b0b'
  primary-soft: 'rgba(242, 184, 39, 0.14)'
  ink: '#f4eadf'
  ink-strong: '#fff7ed'
  body: '#d6cabc'
  muted: '#a9a19a'
  muted-soft: '#7b746e'
  hairline: 'rgba(244, 234, 223, 0.14)'
  hairline-strong: 'rgba(244, 234, 223, 0.24)'
  app-canvas: '#17191d'
  app-canvas-deep: '#101216'
  titlebar: '#1b1c20'
  activity-surface: '#202126'
  panel-surface: '#25272c'
  panel-raised: '#2d3036'
  editor-canvas: '#202226'
  editor-paper: '#202226'
  floating-surface: '#2a2d33'
  light-canvas: '#f4f0e8'
  light-panel: '#fbf8f1'
  light-paper: '#f7f3eb'
  on-primary: '#1b1707'
  semantic-success: '#58d77d'
  semantic-warning: '#f2b827'
  semantic-error: '#ef6f75'
  semantic-info: '#7ab7ff'
typography:
  ui-title:
    fontFamily: "Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif"
    fontSize: 16px
    fontWeight: 700
    lineHeight: 1.25
    letterSpacing: -0.01em
  ui-body:
    fontFamily: "Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif"
    fontSize: 14px
    fontWeight: 500
    lineHeight: 1.45
    letterSpacing: 0
  ui-caption:
    fontFamily: "Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif"
    fontSize: 12px
    fontWeight: 500
    lineHeight: 1.35
    letterSpacing: 0.01em
  ui-eyebrow:
    fontFamily: "Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif"
    fontSize: 11px
    fontWeight: 700
    lineHeight: 1.2
    letterSpacing: 0.08em
    textTransform: uppercase
  editor-prose:
    fontFamily: "ui-serif, 'Songti SC', 'Noto Serif CJK SC', 'Source Han Serif SC', Georgia, serif"
    fontSize: 18px
    fontWeight: 400
    lineHeight: 1.85
    letterSpacing: 0
  editor-ui:
    fontFamily: "Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif"
    fontSize: 13px
    fontWeight: 500
    lineHeight: 1.35
    letterSpacing: 0
  code:
    fontFamily: "ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', monospace"
    fontSize: 13px
    fontWeight: 400
    lineHeight: 1.5
    letterSpacing: 0
rounded:
  none: 0px
  sm: 4px
  md: 6px
  lg: 8px
  xl: 12px
  pill: 9999px
spacing:
  xxs: 4px
  xs: 8px
  sm: 12px
  base: 16px
  md: 20px
  lg: 24px
  xl: 32px
  xxl: 48px
  chrome: 3rem
components:
  project-titlebar:
    backgroundColor: '{colors.titlebar}'
    textColor: '{colors.body}'
    height: '{spacing.chrome}'
    rounded: '{rounded.none}'
  titlebar-view-switcher:
    backgroundColor: '{colors.activity-surface}'
    textColor: '{colors.muted}'
    rounded: '{rounded.lg}'
    height: 2.35rem
    padding: 4px
  primary-view-tab-active:
    backgroundColor: '{colors.primary}'
    textColor: '{colors.on-primary}'
    rounded: '{rounded.lg}'
    height: 1.85rem
  secondary-view-rail:
    backgroundColor: '{colors.primary-soft}'
    textColor: '{colors.body}'
    rounded: '{rounded.lg}'
    height: 1.85rem
  tool-window:
    backgroundColor: '{colors.panel-surface}'
    textColor: '{colors.body}'
    rounded: '{rounded.lg}'
    borderColor: '{colors.hairline}'
  editor-canvas:
    backgroundColor: '{colors.editor-canvas}'
    textColor: '{colors.ink}'
  editor-toolbar:
    backgroundColor: '{colors.floating-surface}'
    textColor: '{colors.body}'
    rounded: '{rounded.lg}'
  document-tab:
    backgroundColor: '{colors.panel-surface}'
    textColor: '{colors.body}'
    rounded: '{rounded.lg}'
  input-control:
    backgroundColor: 'transparent'
    textColor: '{colors.ink}'
    rounded: '{rounded.lg}'
    borderColor: '{colors.hairline-strong}'
---

# Mythistorima DESIGN.md

This file defines the visual and interaction language for Mythistorima. It is written for coding agents, design agents
and future contributors. Use it as the source of truth when creating or revising UI. The goal is not to mimic a generic
dashboard; the product should read as a **quiet creative IDE for novel authors**.

## Overview

Mythistorima is a local desktop writing workspace for long-form fiction. The interface must support a slow, concentrated
workflow: planning a story, organizing worldbuilding, writing chapters, revising text and exporting a manuscript. It
should combine the spatial clarity of an IDE with the softness of a writing environment.

Key characteristics:

- **Local-first creative workspace.** The UI should feel durable, private and offline-safe.
- **IDE-like project chrome.** Titlebar navigation, tool windows, document tabs and status bars are compact and precise.
- **Writing-first center.** The editor is visually continuous with the background; do not make it look like a boxed
  form.
- **Floating support panels.** Tool windows carry depth and structure; the writing area stays quiet.
- **Restrained amber accent.** Amber is for current view, selected tabs, primary actions and progress. It must not flood
  the page.
- **Nuxt UI first.** Buttons, inputs, selects, tabs, empty states, modals, menus and feedback use Nuxt UI. Custom
  rendering is limited to the editor, graphs, timelines and tool-window layout engine.

## Visual Theme & Atmosphere

The visual theme is **nocturnal editorial workspace**:

- Dark mode is the primary product expression. It should feel like a late-night writing room, not a high-contrast
  terminal.
- Light and paper themes are supported, but they should preserve the same structure and density.
- Surfaces are layered through subtle value changes, thin borders and carefully scoped shadows.
- The writing surface should blend with the center canvas. The supporting tool windows are what appear elevated.
- The product voice is professional and calm. Avoid playful gradients, oversized radii, loud cards and decorative
  illustrations.

Use this mental model:

```txt
Titlebar chrome       = command/navigation layer
Tool windows          = floating workspace instruments
Editor center         = quiet manuscript space
Status bar            = unobtrusive telemetry
Dialogs / menus       = temporary elevated layers
```

## Color Palette & Roles

Use semantic Nuxt UI tokens in components. The hex values above describe the design intent; do not inline them in Vue
templates. Prefer `text-primary`, `bg-default`, `bg-muted`, `bg-elevated`, `border-default`, `text-muted`,
`text-highlighted`, `text-error`, `text-success` and `text-warning`.

### Brand & Accent

- **Mythic Amber** (`{colors.primary}`): current primary view, primary buttons, selected story item border, progress
  line.
- **Amber Active** (`{colors.primary-active}`): pressed state, active drag/drop edge.
- **Amber Soft** (`{colors.primary-soft}`): secondary view rail, selected low-emphasis filter, editor references.
- **On Primary** (`{colors.on-primary}`): text/icon color on solid amber.

Amber is scarce. A page should usually have one dominant amber element: active navigation, selected card, or primary
action.

### Dark Surfaces

- **App Canvas** (`{colors.app-canvas}`): main background and central writing void.
- **App Canvas Deep** (`{colors.app-canvas-deep}`): deepest window edge and full-bleed background.
- **Titlebar** (`{colors.titlebar}`): application titlebar and chrome.
- **Activity Surface** (`{colors.activity-surface}`): titlebar navigation shell and tool-strip lanes.
- **Panel Surface** (`{colors.panel-surface}`): tool windows, inspectors, side panels.
- **Panel Raised** (`{colors.panel-raised}`): selected or grouped controls inside panels.
- **Editor Canvas / Paper** (`{colors.editor-canvas}`, `{colors.editor-paper}`): writing area. In dark mode these
  intentionally match or nearly match.
- **Floating Surface** (`{colors.floating-surface}`): editor toolbar, dropdown bodies, popovers.

### Light / Paper Surfaces

- **Light Canvas** (`{colors.light-canvas}`): application background in paper/light theme.
- **Light Panel** (`{colors.light-panel}`): side panels and tool windows.
- **Light Paper** (`{colors.light-paper}`): center writing canvas.

Light mode should still avoid pure white dominance. It should feel like warm paper and stone, not a web admin dashboard.

### Text

- **Ink Strong** (`{colors.ink-strong}`): active page title, selected headings, editor text.
- **Ink** (`{colors.ink}`): editor prose and highlighted UI labels.
- **Body** (`{colors.body}`): normal UI labels.
- **Muted** (`{colors.muted}`): secondary labels and helper text.
- **Muted Soft** (`{colors.muted-soft}`): disabled text and inactive icons.

### Hairlines

- **Hairline** (`{colors.hairline}`): normal panel outlines and dividers.
- **Hairline Strong** (`{colors.hairline-strong}`): focused controls and important split boundaries.

Do not use heavy borders. Panels should be separated by one-pixel hairlines and value shifts.

### Semantic Colors

- **Success**: saved state, completed node, backup success.
- **Warning**: target progress, unresolved items, export warnings.
- **Error**: validation and destructive confirmation.
- **Info**: search hints, imported data, neutral discovery.

Semantic colors are not brand colors. They must never replace amber as navigation/selection identity.

## Typography Rules

### Font Families

- UI: `Inter`, system UI fallbacks.
- Editor prose: user-configurable; default is a CJK-friendly serif stack.
- Code / technical surfaces: system monospace.

Do not bundle font files unless the project license explicitly allows it.

### Type Scale

| Token                       | Size | Weight | Line Height | Use                                  |
|-----------------------------|-----:|-------:|------------:|--------------------------------------|
| `{typography.ui-title}`     | 16px |    700 |        1.25 | panel titles, selected view labels   |
| `{typography.ui-body}`      | 14px |    500 |        1.45 | default controls, lists, tab labels  |
| `{typography.ui-caption}`   | 12px |    500 |        1.35 | helper text, metadata, status labels |
| `{typography.ui-eyebrow}`   | 11px |    700 |         1.2 | section eyebrows such as INSPECTOR   |
| `{typography.editor-prose}` | 18px |    400 |        1.85 | manuscript text                      |
| `{typography.editor-ui}`    | 13px |    500 |        1.35 | editor toolbar, document tabs        |
| `{typography.code}`         | 13px |    400 |         1.5 | shortcuts, exports, template code    |

### Typography Principles

- Keep UI labels compact. The writing text should be the largest quiet element.
- Main titlebar navigation may use 15–16px labels for the selected primary view; inactive primary views are icon-only.
- Secondary view tabs are visually subordinate: smaller type, lower contrast, lighter treatment.
- Do not use all-caps for normal Chinese UI labels. Use uppercase only for tiny metadata labels like `INSPECTOR`.
- Editor prose must not inherit the UI font unless the user selects a sans-serif writing font.

## Shape System

The standard radius is `--ui-radius: 0.5rem`, equivalent to `{rounded.lg}` / 8px. This is the default for panels,
controls, titlebar navigation, document tabs and tool windows.

| Token            |  Value | Use                                            |
|------------------|-------:|------------------------------------------------|
| `{rounded.none}` |    0px | hard window edges, full-width stage boundaries |
| `{rounded.sm}`   |    4px | tiny indicators, internal selection handles    |
| `{rounded.md}`   |    6px | compact chips and dense list accents           |
| `{rounded.lg}`   |    8px | default UI controls, panels, cards, tabs       |
| `{rounded.xl}`   |   12px | rare large empty states or preview paper       |
| `{rounded.pill}` | 9999px | badges only, not general panels                |

Rules:

- Do not introduce 16–24px card radii.
- Do not mix pill and rectangular tab styles in the same navigation group.
- Use Nuxt UI defaults where possible; normalize to `rounded-lg` for app-owned panels.

## Layout Principles

### Global App Frame

The desktop shell is a fixed-height workspace.

```txt
┌──────────────────────────────── ProjectTitlebar ────────────────────────────────┐
│ breadcrumbs                 centered view switcher                 status/tools │
├──────────────────────────────── Workspace stage ────────────────────────────────┤
│ project-specific content, tool windows, editor, panels                          │
├──────────────────────────────── ProjectStatusBar ───────────────────────────────┤
│ save state · document · counts · timer · project total                          │
└─────────────────────────────────────────────────────────────────────────────────┘
```

- Titlebar height: `{spacing.chrome}` / 3rem.
- Status bar height: 1.5rem.
- The main stage owns scrolling. Avoid page-level body scroll in project mode.
- Each tool window and panel must scroll internally when content overflows.

### Titlebar Navigation

The main view switcher lives **inside the ProjectTitlebar center**, not as a floating overlay.

Primary views:

```txt
Overview · Outline · Resources · Writing · Export
```

Rules:

- Use icon-only tabs for inactive primary views.
- The active primary view shows icon + label and uses solid amber.
- If the active primary view has secondary views, insert the secondary rail immediately to the right of the active
  primary view, not at the end.
- The secondary rail visually extends from the active primary view but does not use the same solid selected treatment.
- Secondary tabs use a soft amber rail, smaller type, and a sliding underline/indicator.
- Transitions should imply physical tab movement: the active block slides horizontally; the workspace page follows the
  tab direction.

Example structure:

```txt
[Overview icon] [Active View] [Subview 1] [Subview 2] [Next View icon]
```

### Workspace Layout

- Overview: dashboard cards, metadata, progress, recent documents, pending work.
- Outline: one workspace with secondary modes for board, graph and timeline.
- Resources: master-detail setting cards and relation graph.
- Writing: editor-centered workspace with tool strips and document tabs.
- Export: settings panel plus live preview.

Do not keep a generic right-side `Context` panel. Context belongs inside each workspace's own inspector or detail panel.

### Writing Workspace

The writing workspace follows an IDE tool-window model:

```txt
Left tool strip | left tool windows | centered editor | right tool windows | right tool strip
```

Rules:

- Tool strips stay on screen.
- Tool windows always remain inside the viewport.
- Bottom tool windows have highest priority and occupy the bottom region; if one side has a bottom window it may span
  the bottom width, if both sides do, split the bottom.
- Top/middle side tool windows fill the available side height when bottom is absent; if both top and middle exist, split
  the side height.
- Tool windows can be resized; editor stays centered in the remaining gap.
- Tool windows are visually elevated. The editor canvas is not.

### Editor Center

- The editor width is forced by page-width settings, not by viewport width.
- The editor remains centered inside the remaining workspace area.
- The editor should not have a card border, boxed background or obvious container outline.
- The editor toolbar is fixed at the top of the editor area and does not scroll with prose.
- Prose scroll is internal and vertical only. Horizontal scroll is a bug unless a code/preformatted block explicitly
  requires it.

## Depth & Elevation

Use a low-depth hierarchy.

| Layer    | Treatment                                      | Use                                      |
|----------|------------------------------------------------|------------------------------------------|
| Canvas   | flat app background                            | central editor void, workspace floor     |
| Panel    | hairline + subtle surface shift                | side panels, inspectors, lists           |
| Floating | stronger value + narrow shadow                 | tool windows, editor toolbar, dropdowns  |
| Modal    | Nuxt UI modal elevation                        | settings, confirmations, command palette |
| Paper    | optional soft shadow only in paper/light theme | export previews, document preview paper  |

Rules:

- Ordinary panels do not use big shadows.
- Shadows are allowed for modals, popovers, dropdowns, floating tool windows and paper previews.
- The editor should not compete with tool windows. If the editor looks like a card, reduce its border/shadow.

## Component Styling

### Nuxt UI Contract

Use Nuxt UI for standard application controls:

| Need       | Component                                                        |
|------------|------------------------------------------------------------------|
| Buttons    | `UButton`, `UButtonGroup`                                        |
| Inputs     | `UInput`, `UTextarea`, `UInputNumber`                            |
| Selects    | `USelect`, `USelectMenu`                                         |
| Forms      | `UForm`, `UFormField`                                            |
| Toggles    | `USwitch`, `UCheckbox`                                           |
| Navigation | icon `UButton` for primary nav, `UTabs` for secondary nav        |
| Trees      | `UTree`                                                          |
| Menus      | `UDropdownMenu`, `UContextMenu`                                  |
| Dialogs    | `UModal`, `USlideover`                                           |
| Feedback   | `UToast`, `UAlert`, `UEmpty`, `USkeleton`, `UBadge`, `UProgress` |
| Keyboard   | `UKbd`                                                           |

Do not recreate these controls as raw `button`, `input`, `select` or custom class systems unless the UI component is
truly unavailable.

### Project Titlebar

`project-titlebar`:

- Height `{spacing.chrome}`.
- Left: app/home affordance + breadcrumbs.
- Center: `titlebar-view-switcher`.
- Right: save state, command shortcut, settings.
- No generic ProjectContextToolbar below it.
- The view switcher is clipped inside the titlebar; it must not visually float above the page.

### Primary View Tabs

`primary-view-tab-active`:

- Solid amber block.
- Text is high contrast on amber.
- Standard 8px radius.
- Active block slides horizontally when switching.
- Inactive tabs show icons only and muted color.

### Secondary View Rail

`secondary-view-rail`:

- Appears immediately after the active primary view.
- Soft amber background; no outline.
- It should visually connect to the primary tab without a visible seam.
- Selected secondary tab uses underline/indicator, not a solid amber block.
- Secondary labels are smaller and lower priority than the primary active label.

### Tool Windows

`tool-window`:

- Rounded 8px.
- Hairline border.
- Header height 2.25–2.5rem.
- Header contains icon, title, layout/move controls and close/minimize.
- Body scrolls internally.
- Avoid nested cards inside tool windows. Use sections, dividers and lists.

### Document Tabs

`document-tab`:

- Height equals or is visually aligned with the tool-strip width when in writing mode.
- No empty-state card in the tab strip. If no document is open, leave reserved blank space.
- Close buttons appear only on actual tabs.
- Active document tab uses subtle raised surface and accent line, not a huge color block.

### Editor Toolbar

`editor-toolbar`:

- Fixed above the prose scroll container.
- Compact icon buttons with tooltips.
- Uses Nuxt UI buttons where possible.
- Search/find tools sit at the right end.
- Do not let drag handles or toolbar controls appear inside prose.

### Story Planning Panels

Board, graph and timeline share the same data and must look like modes of one Outline workspace.

- Board cards: compact, drag-friendly, amber outline for selected card.
- Graph canvas: custom SVG/canvas area with Nuxt UI controls around it.
- Timeline: vertical or lane-style event list with internal scroll.
- Node/event details live in the workspace inspector, not a global Context panel.

### Resources Workspace

Resources is a master-detail system:

- Left: type/list filtering.
- Center: setting card details or relation graph.
- Right: detail inspector.
- Relation editing belongs to the detail/inspector panel.
- User-facing labels should say "Setting", "Relation", "Type", "Field"; never expose database table names.

### Export Workspace

Export is a production surface:

- Left/settings: template, range, style, publish options.
- Center/right: live page preview.
- Preview regenerates after a short debounce.
- Export and preview use the same configuration path.
- Use `UAlert`, `UToast`, `USkeleton` and `UEmpty` for states.

## Motion & Interaction

Motion should make spatial relationships understandable, not decorative.

### Timing

| Motion                       |        Duration | Easing                      |
|------------------------------|----------------:|-----------------------------|
| Tab active block movement    |       180–240ms | cubic-bezier(.2, .8, .2, 1) |
| Secondary rail insert/remove |       200–260ms | cubic-bezier(.2, .8, .2, 1) |
| Workspace page slide         |       180–240ms | cubic-bezier(.2, .8, .2, 1) |
| Tool window open/close       |       160–220ms | ease-out                    |
| Modal/dropdown               | Nuxt UI default | Nuxt UI default             |

### Directional Transitions

When switching views, compare the previous tab index with the next tab index:

- Moving to a later tab: page enters from the right and exits left.
- Moving to an earlier tab: page enters from the left and exits right.
- Switching secondary tabs uses the same directional rule within the active primary view.

Avoid generic fade-only workspace changes. They make navigation feel disconnected.

### Reduced Motion

Respect `prefers-reduced-motion`:

- Disable slide transforms.
- Keep opacity changes below 100ms or remove them.
- Do not animate editor text layout.

## Responsive Behavior

Mythistorima is desktop-first, but layouts must remain usable at 1280×720.

| Breakpoint  | Behavior                                                                                                 |
|-------------|----------------------------------------------------------------------------------------------------------|
| < 900px     | Project workspace becomes single-column where possible; large inspectors collapse into slideovers.       |
| 900–1280px  | Keep titlebar navigation compact; inactive primary views are icon-only; tool windows use minimum widths. |
| 1280–1600px | Full working layout with side tool windows and centered editor.                                          |
| > 1600px    | Editor stays capped by page width; excess space belongs to tool windows or background.                   |

Touch targets:

- Main titlebar tabs: at least 32px tall.
- Toolbar buttons: 28–32px.
- Standard forms: Nuxt UI `size="sm"` unless the field is a primary writing target.

## Accessibility

- Every icon-only button needs an accessible label or tooltip.
- Active navigation uses `aria-current="page"` or a semantically equivalent selected state.
- `UTabs` should manage keyboard navigation for secondary views.
- Text contrast must remain readable in dark, light and paper themes.
- Do not rely on color alone for document status, errors or selection.
- Empty states must tell the user what to do next.

## CSS Organization

CSS is organized by product domain. Keep `app/assets/css/main.css` as an import-only entry file.

Expected domains:

```txt
base/          global tokens, themes, radius normalization
layout/        app shell and legacy workspace boundaries
shell/         project titlebar, statusbar, navigation, tool windows, writing shell
project/       project hub, dashboard, overview
writing/       editor, document tree, writing tools
outline/       outline board and graph primitives
story/         shared story workspace surfaces
resources/     cards, relations, mentions
publication/   export settings and preview
command/       command palette
settings/      settings workspace and modal sections
```

Rules:

- No file or class names may contain sprint or phase labels.
- Avoid one-off global selectors. Scope styles by product domain.
- Prefer Nuxt UI theme tokens over fixed colors in templates.
- Keep each CSS file short enough to reason about; split when a file mixes unrelated domains.

## Do's and Don'ts

### Do

- Use Nuxt UI components for standard controls.
- Keep titlebar navigation centered inside the titlebar.
- Use amber sparingly for active route and primary action.
- Keep writing area visually quiet and centered.
- Let tool windows, popovers and modals carry elevation.
- Use 8px radius as the default across app-owned surfaces.
- Make each workspace internally responsible for its details/inspector.
- Preserve keyboard-first behavior and command palette access.

### Don't

- Don't restore a generic `ProjectContextToolbar`.
- Don't render a persistent right-side `Context` panel.
- Don't expose UUIDs, database table names, Rust/Tauri internals or sprint labels in user-facing UI.
- Don't create `.primary-button` / `.secondary-button` style systems outside Nuxt UI.
- Don't add large glassmorphism panels, warm full-page gradients or oversized rounded cards.
- Don't make the editor look like a bordered form field.
- Don't let tool windows overflow outside the viewport.
- Don't use solid amber for secondary tabs.

## Agent Prompt Guide

When asking an AI coding agent to create or change UI in this project, use prompts like:

```txt
Use DESIGN.md and UI_COMPONENT_GUIDELINES.md. Build this as a Nuxt UI-first desktop workspace.
Use semantic Nuxt UI tokens, rounded-lg / --ui-radius 0.5rem, and keep the writing canvas visually flat.
Do not add sprint/phase names, ProjectContextToolbar, generic Context panels, or custom button systems.
```

For titlebar/navigation work:

```txt
Place the primary view switcher inside ProjectTitlebar center. Inactive primary views are icon-only.
The active primary view shows icon + label in solid amber. If secondary views exist, insert a soft secondary rail immediately to the right of the active primary view. Secondary selection uses an underline/indicator, not a solid amber block. Animate page direction based on tab order.
```

For writing work:

```txt
Keep the editor centered within remaining space. The editor toolbar is fixed above the prose scroll area.
Tool windows stay inside the viewport, scroll internally, and carry elevation. The editor background blends with the center canvas.
```

For story planning/resources/export work:

```txt
Use Nuxt UI controls around custom graph/timeline/editor canvases. Keep detail panels inside the workspace.
Avoid duplicate headers and avoid a global context panel. Use empty/loading/error states from Nuxt UI.
```

## Iteration Guide

1. Start from the layout contract: titlebar, workspace stage, statusbar.
2. Choose Nuxt UI primitives before adding custom markup.
3. Apply semantic tokens and only then add domain-specific CSS.
4. Verify dark, light and paper themes.
5. Verify 1280×720 and 1920×1080.
6. Check keyboard focus and icon labels.
7. Check that no phase/sprint names or implementation details are visible.
8. Run relevant UI audit scripts before packaging.

## Known Gaps

- The exact amber value may be adjusted through Nuxt UI's `primary: 'amber'` theme mapping; components should rely on
  semantic tokens.
- Editor typography is user-configurable, so prose examples should not assume a single final font.
- Graph, relation and timeline canvases are custom-rendered and require visual QA beyond component audits.
- Full Tauri builds require the local desktop toolchain and dependencies.
