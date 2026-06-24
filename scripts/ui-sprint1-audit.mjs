import fs from 'node:fs'
import path from 'node:path'
import {execFileSync} from 'node:child_process'

const root = process.cwd()
const failures = []

function read(relativePath) {
    return fs.readFileSync(path.join(root, relativePath), 'utf8')
}

function requireFile(relativePath) {
    if (!fs.existsSync(path.join(root, relativePath))) {
        failures.push(`missing required file: ${relativePath}`)
        return false
    }
    return true
}

try {
    execFileSync(process.execPath, ['scripts/ui-sprint0-audit.mjs'], {
        cwd: root,
        stdio: 'pipe'
    })
} catch (error) {
    failures.push('Sprint 0 baseline audit failed')
}

const appRoot = read('app/app.vue')
if (!appRoot.includes('<NuxtLayout>') || !appRoot.includes('<NuxtPage')) {
    failures.push('app.vue must wrap NuxtPage with NuxtLayout so page layouts are mounted')
}
if (!requireFile('app/layouts/default.vue')) {
    failures.push('default passthrough layout is missing')
}

const editorSource = read('app/components/editor/NovelEditor.vue')
const workspaceContent = read('app/components/workspace/ProjectWorkspaceContent.vue')
if (!editorSource.includes('showStatusBar?: boolean')) {
    failures.push('NovelEditor does not expose the status bar visibility switch')
}
if (!workspaceContent.includes(':show-status-bar="false"')) {
    failures.push('project workspace still renders the duplicate editor status bar')
}

const requiredFiles = [
    'app/layouts/project.vue',
    'app/components/shell/ProjectTitlebar.vue',
    'app/components/shell/ProjectActivityBar.vue',
    'app/components/shell/ProjectContextToolbar.vue',
    'app/components/shell/ProjectStatusBar.vue',
    'app/components/settings/AppSettingsModal.vue',
    'app/components/command/CommandPalette.vue',
    'app/components/workspace/ProjectWorkspaceRoot.vue',
    'app/components/workspace/ProjectWorkspaceSidebar.vue',
    'app/components/workspace/ProjectWorkspaceContent.vue',
    'app/components/workspace/ProjectWorkspaceInspector.vue',
    'app/composables/useProjectWorkspaceController.ts',
    'app/composables/useProjectWorkspaceContext.ts',
    'app/stores/projectShell.store.ts',
    'app/assets/css/project-shell.css'
]
for (const file of requiredFiles) requireFile(file)

const page = read('app/pages/project/[id].vue')
const pageLineCount = page.split(/\r?\n/).length
if (pageLineCount > 40) {
    failures.push(`project route page must remain a thin composition root; found ${pageLineCount} lines`)
}
if (!page.includes("layout: 'project'")) {
    failures.push('project route does not declare the project layout')
}
if (page.includes('<AppShell')) {
    failures.push('project route still renders the legacy AppShell directly')
}

const layout = read('app/layouts/project.vue')
for (const component of [
    'ProjectTitlebar',
    'ProjectActivityBar',
    'ProjectContextToolbar',
    'ProjectStatusBar',
    'CommandPalette',
    'AppSettingsModal'
]) {
    if (!layout.includes(`<${component}`)) {
        failures.push(`project layout does not render ${component}`)
    }
}

const commandPalette = read('app/components/command/CommandPalette.vue')
if (!commandPalette.includes('<UCommandPalette')) {
    failures.push('command palette must use Nuxt UI UCommandPalette')
}
if (!commandPalette.includes('<UModal')) {
    failures.push('command palette must be application-level inside UModal')
}

const shellCss = read('app/assets/css/project-shell.css')
const requiredGeometry = [
    'grid-template-rows: 2.5rem minmax(0, 1fr) 1.5rem',
    'grid-template-columns: 3rem minmax(0, 1fr)',
    'grid-template-rows: 2.5rem minmax(0, 1fr)'
]
for (const geometry of requiredGeometry) {
    if (!shellCss.includes(geometry)) {
        failures.push(`project shell geometry is missing: ${geometry}`)
    }
}

const nuxtConfig = read('nuxt.config.ts')
if (!nuxtConfig.includes("'~/assets/css/project-shell.css'")) {
    failures.push('project-shell.css is not registered in nuxt.config.ts')
}

if (failures.length) {
    console.error('UI Sprint 1 audit failed:')
    for (const failure of failures) console.error(`- ${failure}`)
    process.exit(1)
}

console.log('UI Sprint 1 audit passed.')
console.log('- project layout')
console.log('- primary activity bar')
console.log('- titlebar and context toolbar')
console.log('- bottom status bar')
console.log('- application settings modal')
console.log('- Nuxt UI command palette')
console.log(`- project route reduced to ${pageLineCount} lines`)
