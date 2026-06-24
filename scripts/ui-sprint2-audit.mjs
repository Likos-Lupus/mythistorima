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
    execFileSync(process.execPath, ['scripts/ui-sprint1-audit.mjs'], {
        cwd: root,
        stdio: 'pipe'
    })
} catch {
    failures.push('Sprint 1 baseline audit failed')
}

const requiredFiles = [
    'app/components/project/ProjectCreateModal.vue',
    'app/components/project/ProjectRenameModal.vue',
    'app/components/project/ProjectList.vue',
    'app/components/project/ProjectDashboard.vue',
    'app/assets/css/project-overview.css',
    'src-tauri/src/models/stats.rs',
    'src-tauri/src/services/stats_service.rs',
    'src-tauri/src/commands/stats.rs'
]
for (const file of requiredFiles) requireFile(file)

const home = read('app/pages/index.vue')
for (const required of ['<UTabs', '<UInput', '<ProjectList', '<ProjectCreateModal', '<ProjectRenameModal']) {
    if (!home.includes(required)) failures.push(`Project Hub is missing ${required}`)
}
for (const forbidden of ['像纸一样安静', 'NOVEL WORKSPACE MVP', 'Rust 已连接', 'SQLite 正常']) {
    if (home.includes(forbidden)) failures.push(`Project Hub contains forbidden legacy text: ${forbidden}`)
}

const projectList = read('app/components/project/ProjectList.vue')
if (!projectList.includes('<UDropdownMenu')) failures.push('project list is missing the actions dropdown')
if (!projectList.includes('statsById')) failures.push('project list does not show project statistics')
if (!projectList.includes('formatRelativeDate')) failures.push('project list does not show relative update time')

const createModal = read('app/components/project/ProjectCreateModal.vue')
if (!createModal.includes('<UModal') || !createModal.includes('<UForm')) {
    failures.push('new project dialog must use UModal and UForm')
}


const projectViews = read('app/constants/projectViews.ts')
if (projectViews.includes("mode: 'stats'")) failures.push('overview must not expose a separate statistics tab')
if (!projectViews.includes("label: '概览'")) failures.push('dashboard workspace should be labelled as 概览')

const dashboard = read('app/components/project/ProjectDashboard.vue')
for (const required of [
    '<UPageHeader',
    '<UForm',
    '<UInputTags',
    '最近文档',
    '未完成事项',
    '未回收伏笔',
    '人物出场',
    '最近 14 天写作趋势'
]) {
    if (!dashboard.includes(required)) failures.push(`project overview is missing: ${required}`)
}
if (/<input\b|<select\b|<textarea\b|<button\b/i.test(dashboard)) {
    failures.push('project overview contains native form/button controls instead of Nuxt UI components')
}

const projectTypes = read('app/types/project.ts')
if (!projectTypes.includes('export interface ProjectMetadata')) {
    failures.push('ProjectMetadata type is missing')
}
if (!projectTypes.includes('metadataJson?: string | null')) {
    failures.push('UpdateProjectInput cannot persist metadataJson')
}

const statsService = read('src-tauri/src/services/stats_service.rs')
if (!statsService.includes('pub async fn get_project_overview')) {
    failures.push('get_project_overview service is missing')
}
if (/sqlx::query(?:_as|_scalar)?(?:::<[^>]+>)?\(\s*&[A-Za-z_]/.test(statsService)) {
    failures.push('stats service contains dynamic SQL incompatible with sqlx 0.9')
}

const lib = read('src-tauri/src/lib.rs')
if (!lib.includes('commands::stats::get_project_overview')) {
    failures.push('get_project_overview is not registered in the Tauri handler')
}

const nuxtConfig = read('nuxt.config.ts')
if (!nuxtConfig.includes("'~/assets/css/project-overview.css'")) {
    failures.push('project-overview.css is not registered')
}

if (failures.length) {
    console.error('UI Sprint 2 audit failed:')
    for (const failure of failures) console.error(`- ${failure}`)
    process.exit(1)
}

console.log('UI Sprint 2 audit passed.')
console.log('- Project Hub')
console.log('- compact recent project list')
console.log('- Nuxt UI create/rename modals')
console.log('- single project overview workspace')
console.log('- metadata form')
console.log('- merged statistics and project-level settings')
