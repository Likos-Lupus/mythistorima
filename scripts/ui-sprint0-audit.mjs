import fs from 'node:fs'
import path from 'node:path'

const root = process.cwd()
const failures = []

function read(relativePath) {
    return fs.readFileSync(path.join(root, relativePath), 'utf8')
}

function walk(directory, files = []) {
    for (const entry of fs.readdirSync(directory, {withFileTypes: true})) {
        const target = path.join(directory, entry.name)
        if (entry.isDirectory()) walk(target, files)
        else files.push(target)
    }
    return files
}

const packageJson = JSON.parse(read('package.json'))
if (packageJson.dependencies?.['@nuxt/ui'] !== '4.9.0') {
    failures.push('package.json must lock @nuxt/ui to 4.9.0')
}
if (packageJson.dependencies?.nuxt !== '4.4.8') {
    failures.push('package.json must lock Nuxt to the verified 4.4.8 baseline')
}
if (!fs.existsSync(path.join(root, 'app/app.config.ts'))) {
    failures.push('app/app.config.ts is missing')
}

const viewSource = read('app/constants/projectViews.ts')
const primaryViewBlock = viewSource.match(/projectPrimaryViews[\s\S]*?=\s*\[([\s\S]*?)\]\n\nexport const projectWorkspaceDefinitions/)
const primaryIds = [...(primaryViewBlock?.[1] ?? '').matchAll(/\bid:\s*'([^']+)'/g)].map(match => match[1])
const expectedViews = ['overview', 'outline', 'resources', 'writing', 'export']
if (JSON.stringify(primaryIds) !== JSON.stringify(expectedViews)) {
    failures.push(`primary views must be exactly: ${expectedViews.join(', ')}`)
}

const forbiddenVisible = [
    /\bPhase\s*\d/iu,
    /\bWeek\s*\d/iu,
    /\bFoundation\b/iu,
    /\bMVP\b/iu,
    /项目\s*ID/iu,
    /Rust\s*(已|未|连接|状态)/iu,
    /SQLite\s*(正常|异常|未|状态)/iu,
    /数据库\s*(正常|未就绪|状态)/iu
]

for (const file of walk(path.join(root, 'app')).filter(file => file.endsWith('.vue'))) {
    const source = fs.readFileSync(file, 'utf8')
    const template = source.split(/<script\b/i, 1)[0]
    const visibleText = template
        .replace(/<!--[\s\S]*?-->/g, ' ')
        .replace(/<[^>]*>/g, ' ')
        .replace(/\{\{[\s\S]*?\}\}/g, ' ')
        .replace(/\s+/g, ' ')

    for (const pattern of forbiddenVisible) {
        if (pattern.test(visibleText)) {
            failures.push(`${path.relative(root, file)} contains forbidden visible text: ${pattern}`)
        }
    }
}

const projectPage = read('app/pages/project/[id].vue')
if (/workspaceMode\s*===\s*['"]settings['"]/.test(projectPage)) {
    failures.push('settings must not be a primary workspace')
}
const projectLayout = fs.existsSync(path.join(root, 'app/layouts/project.vue'))
    ? read('app/layouts/project.vue')
    : ''
if (!projectPage.includes('<AppSettingsModal') && !projectLayout.includes('<AppSettingsModal')) {
    failures.push('application settings modal is not connected')
}

if (failures.length) {
    console.error('UI Sprint 0 audit failed:')
    for (const failure of failures) console.error(`- ${failure}`)
    process.exit(1)
}

console.log('UI Sprint 0 audit passed.')
console.log('- Five primary views: 概览 / 大纲 / 资料 / 写作 / 导出')
console.log('- Nuxt UI: 4.9.0')
console.log('- Nuxt: 4.4.8')
console.log('- Development and technical status text hidden')
