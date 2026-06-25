import fs from 'node:fs'
import path from 'node:path'

const root = process.cwd()
const read = file => fs.readFileSync(path.join(root, file), 'utf8')

const packageJson = JSON.parse(read('package.json'))
const bunLockText = read('bun.lock')
const tiptapDirectPackages = [
    '@tiptap/core',
    '@tiptap/extension-paragraph',
    '@tiptap/pm',
    '@tiptap/starter-kit',
    '@tiptap/vue-3'
]
const isTiptapV3Range = value => typeof value === 'string' && /(?:\^|~)?3\./.test(value)
const hasTiptapV2Locks = [
    '@tiptap/core',
    '@tiptap/pm',
    '@tiptap/starter-kit',
    '@tiptap/vue-3',
    '@tiptap/extension-paragraph'
].some(name => new RegExp(`\"${name.replace('/', '\\/')}@2\\.`).test(bunLockText))
const workspaceStore = read('app/stores/workspaceLayout.store.ts')
const writingWorkspace = read('app/components/writing/WritingWorkspace.vue')
const toolWindow = read('app/components/workspace/WorkspaceToolWindow.vue')
const toolStrip = read('app/components/workspace/WorkspaceToolStrip.vue')
const projectShellCss = read('app/assets/css/project-shell.css')
const checks = [
    {
        name: 'TipTap dependencies are aligned with Nuxt UI Editor v3',
        pass: tiptapDirectPackages.every(name => isTiptapV3Range(packageJson.dependencies?.[name]))
            && tiptapDirectPackages.every(name => packageJson.overrides?.[name] === '3.27.1')
            && tiptapDirectPackages.every(name => packageJson.resolutions?.[name] === '3.27.1')
            && !hasTiptapV2Locks
            && read('nuxt.config.ts').includes('resolve:')
            && read('nuxt.config.ts').includes('dedupe:')
            && read('nuxt.config.ts').includes("'@tiptap/extension-node-range'")
    },
    {
        name: 'Document tabs store and compact Nuxt UI tabs component exist',
        pass: fs.existsSync(path.join(root, 'app/stores/documentTabs.store.ts'))
            && read('app/components/writing/DocumentTabs.vue').includes('UTabs')
            && projectShellCss.includes('height: var(--workspace-tool-strip-size)')
            && projectShellCss.includes('font-size: 0.72rem')
            && read('app/components/writing/DocumentTabs.vue').includes('document-tabs-placeholder')
            && !read('app/components/writing/DocumentTabs.vue').includes('UEmpty')
            && !read('app/components/writing/DocumentTabs.vue').includes('document-tabs-actions')
            && !read('app/components/writing/DocumentTabs.vue').includes('尚未打开文档')
            && !read('app/components/writing/DocumentTabs.vue').includes('min-w-32')
    },
    {
        name: 'ToolWindow uses JetBrains-style side/position slots instead of fixed/floating modes',
        pass: workspaceStore.includes("export type ToolWindowSide = 'left' | 'right'")
            && workspaceStore.includes("export type ToolWindowPosition = 'top' | 'center' | 'bottom'")
            && workspaceStore.includes('closeSlotPeers')
            && !workspaceStore.includes('pinned:')
            && !workspaceStore.includes('pinnedWindows')
            && !writingWorkspace.includes('floatingWindows')
            && !writingWorkspace.includes('pinnedWindows')
            && !toolWindow.includes('toggle-pin')
            && !toolWindow.includes('is-floating')
            && toolWindow.includes('set-slot')
            && toolWindow.includes('隐藏窗口')
            && !toolWindow.includes('自动隐藏')
            && workspaceStore.includes('workspace-layout:v7')
    },
    {
        name: 'Tool strips are persistent left/right rails with three vertical zones',
        pass: toolStrip.includes("side: ToolWindowSide")
            && toolStrip.includes("const positions: ToolWindowPosition[] = ['top', 'center', 'bottom']")
            && toolStrip.includes("`is-${side}`")
            && writingWorkspace.includes('layoutStore.leftWindows')
            && writingWorkspace.includes('layoutStore.rightWindows')
    },
    {
        name: 'Writing workspace reserves side and bottom collision zones for visible tool windows',
        pass: writingWorkspace.includes('leftSideSlotWindows')
            && writingWorkspace.includes('rightSideSlotWindows')
            && writingWorkspace.includes('bottomSlotWindows')
            && writingWorkspace.includes('startSideSplitResize')
            && writingWorkspace.includes('startBottomSplitResize')
            && writingWorkspace.includes('--writing-left-tool-size')
            && writingWorkspace.includes('--writing-bottom-tool-size')
            && projectShellCss.includes('grid-template-areas:')
            && projectShellCss.includes('"left-strip left-tools editor right-tools right-strip"')
            && projectShellCss.includes('"left-strip bottom bottom bottom right-strip"')
            && projectShellCss.includes('grid-template-rows: minmax(0, 1fr) clamp(15rem, var(--writing-bottom-tool-size), 50%)')
            && projectShellCss.includes('grid-area: bottom')
            && projectShellCss.includes('height: 100%')
            && projectShellCss.includes('.writing-tool-slot.is-single')
            && projectShellCss.includes('.workspace-tool-window.is-side-dock,')
            && projectShellCss.includes('scrollbar-gutter: stable')
            && projectShellCss.includes('width: min(var(--editor-page-width), calc(100% - 2rem))')
            && projectShellCss.includes('overflow-x: clip')
            && projectShellCss.includes('editor-bridge-toolbar-row')
            && projectShellCss.includes('flex: 0 0 2.75rem')
            && !projectShellCss.includes('top: 3rem;')
            && !projectShellCss.includes('calc(100vw - 9rem)')
    },
    {
        name: 'Document tree migrated to Nuxt UI tree/context menu',
        pass: read('app/components/project/DocumentTree.vue').includes('<UTree')
            && read('app/components/project/DocumentTree.vue').includes('<UContextMenu')
            && !read('app/components/project/DocumentTree.vue').includes('<select')
    },
    {
        name: 'Find/replace is floating and shortcut-ready',
        pass: read('app/components/editor/EditorFindReplace.vue').includes('Transition')
            && read('app/components/writing/EditorBridge.vue').includes("key === 'f'")
            && read('app/components/writing/EditorBridge.vue').includes("key === 'h'")
    },
    {
        name: 'Editor adapter uses Nuxt UI UEditor and removes visible frame chrome',
        pass: read('app/components/writing/EditorBridge.vue').includes('<UEditor')
            && read('app/components/writing/EditorBridge.vue').includes('<UEditorToolbar')
            && read('app/components/writing/EditorBridge.vue').includes('<UEditorDragHandle')
            && read('app/components/writing/EditorBridge.vue').includes('editor-bridge-drag-handle')
            && read('app/components/writing/EditorBridge.vue').includes('<UEditorSuggestionMenu')
            && read('app/components/writing/EditorBridge.vue').includes('EditorToolbarItem[][]')
            && read('app/components/writing/EditorBridge.vue').includes('@update:model-value')
            && read('app/components/writing/EditorBridge.vue').includes('ParagraphId')
            && read('app/components/writing/EditorBridge.vue').includes('SettingReference')
            && !read('app/components/writing/EditorBridge.vue').includes('NovelEditor')
            && !read('app/components/writing/EditorBridge.vue').includes(':on-create')
            && !read('app/components/writing/EditorBridge.vue').includes(':on-update')
            && projectShellCss.includes('.editor-bridge-root')
            && projectShellCss.includes('.editor-bridge-toolbar-row')
            && projectShellCss.includes('border: 0;')
            && projectShellCss.includes('background: transparent;')
    },

    {
        name: 'Writing workspace uses one shared backing surface behind editor and tool windows',
        pass: projectShellCss.includes('--writing-workspace-surface')
            && projectShellCss.includes('.writing-editor-surface')
            && projectShellCss.includes('.writing-tool-column')
            && projectShellCss.includes('.writing-bottom-dock')
            && projectShellCss.includes('background: var(--writing-workspace-surface);')
            && projectShellCss.includes('.workspace-tool-window {')
            && projectShellCss.includes('background: color-mix(in oklab, var(--ui-bg-elevated) 95%, var(--ui-bg));')
    },
    {
        name: 'Writing secondary tabs are collapsed into tool windows',
        pass: read('app/constants/projectViews.ts').includes("primaryView === 'writing'")
    }
]

let failed = false
for (const check of checks) {
    if (check.pass) {
        console.log(`✓ ${check.name}`)
    } else {
        failed = true
        console.error(`✗ ${check.name}`)
    }
}

if (failed) process.exit(1)
console.log('Sprint 3 UI audit passed.')
