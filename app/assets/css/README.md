# CSS organization

`main.css` is the only stylesheet imported by Nuxt. It keeps Tailwind and Nuxt UI first, then imports product-domain modules in a fixed cascade order.

## Folders

- `base/`: global reset, Nuxt UI foundation overrides and theme tokens.
- `layout/`: app-level layout and legacy workspace scaffolding.
- `shell/`: project titlebar, titlebar navigation, project frame, status bar, document tabs, writing tool windows and editor bridge layout.
- `project/`: project hub, dashboard and overview surfaces.
- `workspace/`: shared workspace placeholders and integrated navigation helpers.
- `writing/`: editor, document tree and writing-specific editor styling.
- `story/`: shared story workspace frame, outline kanban, outline graph, story timeline and resource detail surfaces.
- `outline/`, `resources/`, `relations/`, `timeline/`: feature-specific story-planning modules retained for existing screens.
- `notes/`, `insights/`, `proofreading/`, `settings/`, `publication/`, `command/`: secondary product workspaces and global overlays.

## Rules

1. Keep Nuxt UI component usage in Vue; CSS should only adjust layout, density and product-specific surfaces.
2. Use semantic Nuxt UI tokens such as `--ui-bg`, `--ui-border`, `--ui-text`, `--ui-primary`.
3. Use `--ui-radius` for product surfaces. Do not introduce per-feature radius values.
4. Add new styles to the smallest product-domain module, not to `main.css`.
5. Avoid development-stage names in selectors or filenames.
