import fs from 'node:fs'
import path from 'node:path'

const root = process.cwd()
const read = file => fs.readFileSync(path.join(root, file), 'utf8')
const exists = file => fs.existsSync(path.join(root, file))
const listVue = dir => fs.readdirSync(path.join(root, dir), {withFileTypes: true}).flatMap(entry => {
    const rel = path.join(dir, entry.name)
    if (entry.isDirectory()) return listVue(rel)
    return entry.isFile() && entry.name.endsWith('.vue') ? [rel] : []
})

const sprint4SurfaceFiles = [
    'app/components/outline/OutlinePlanningWorkspace.vue',
    'app/components/outline/OutlineNodeInspector.vue',
    'app/components/outline/TimelineEventInspector.vue',
    'app/components/resources/ResourcesWorkspace.vue',
    'app/components/resources/SettingCardDetail.vue',
    'app/components/resources/CardTypeDefinitionInspector.vue',
    'app/components/resources/RelationInspector.vue',
    'app/components/resources/RelationTypeDefinitionInspector.vue',
    'app/components/shell/ProjectContextToolbar.vue',
    'app/assets/css/sprint4.css'
]
const sprint4SurfaceText = sprint4SurfaceFiles.map(read).join('\n')
const allVueText = listVue('app/components').map(read).join('\n')

const checks = [
    {
        name: 'Outline and resources workspaces are mounted under the five-view shell',
        pass: read('app/components/workspace/ProjectWorkspaceContent.vue').includes('OutlinePlanningWorkspace')
            && read('app/components/workspace/ProjectWorkspaceContent.vue').includes('ResourcesWorkspace')
            && read('app/constants/projectViews.ts').includes("label: '图谱'")
            && read('app/constants/projectViews.ts').includes("label: '关系'")
    },
    {
        name: 'Secondary mode switching lives in the context toolbar, not duplicated inside the page body',
        pass: read('app/components/shell/ProjectContextToolbar.vue').includes('<UTabs')
            && !read('app/components/outline/OutlinePlanningWorkspace.vue').includes('<UTabs')
            && !read('app/components/resources/ResourcesWorkspace.vue').includes('<UTabs')
            && !read('app/components/outline/OutlinePlanningWorkspace.vue').includes('story-page-header')
            && !read('app/components/resources/ResourcesWorkspace.vue').includes('story-page-header')
    },
    {
        name: 'Outline view provides board, graph and timeline in one workspace',
        pass: exists('app/components/outline/OutlinePlanningWorkspace.vue')
            && read('app/components/outline/OutlinePlanningWorkspace.vue').includes("modeToTab")
            && read('app/components/outline/OutlinePlanningWorkspace.vue').includes("tabToMode")
            && read('app/components/outline/OutlinePlanningWorkspace.vue').includes("'board'")
            && read('app/components/outline/OutlinePlanningWorkspace.vue').includes("'mind'")
            && read('app/components/outline/OutlinePlanningWorkspace.vue').includes("'timeline'")
            && read('app/components/outline/OutlinePlanningWorkspace.vue').includes('dropOnColumn')
            && read('app/components/outline/TimelineEventInspector.vue').includes('<UForm')
    },
    {
        name: 'Graph and relation canvases remain custom renderers with Nuxt UI controls around them',
        pass: exists('app/components/outline/OutlineMindMapView.vue')
            && read('app/components/outline/OutlineMindMapView.vue').includes('<svg')
            && read('app/components/outline/OutlinePlanningWorkspace.vue').includes('适应窗口')
            && read('app/components/outline/OutlinePlanningWorkspace.vue').includes('复制 Mermaid')
            && read('app/components/resources/ResourcesWorkspace.vue').includes('RelationGraph')
    },
    {
        name: 'Resources view uses master-detail structure for cards and relations',
        pass: exists('app/components/resources/ResourcesWorkspace.vue')
            && read('app/components/resources/ResourcesWorkspace.vue').includes('SettingCardDetail')
            && read('app/components/resources/ResourcesWorkspace.vue').includes('RelationInspector')
            && read('app/components/resources/ResourcesWorkspace.vue').includes('RelationTypeDefinitionInspector')
    },
    {
        name: 'Custom setting and relation type definition layer exists without exposing table names in UI',
        pass: exists('app/stores/resourceDefinitions.store.ts')
            && read('app/types/card.ts').includes('interface CardTypeDefinition')
            && read('app/types/card.ts').includes('parseCardSchemaFields')
            && read('app/types/relation.ts').includes('interface RelationTypeDefinition')
            && read('app/components/resources/CardTypeDefinitionInspector.vue').includes('自定义设定类型')
            && read('app/components/resources/RelationTypeDefinitionInspector.vue').includes('自定义关系类型')
            && !sprint4SurfaceText.includes('card_type_definitions')
            && !sprint4SurfaceText.includes('relation_type_definitions')
    },
    {
        name: 'Regular controls on the new surfaces use Nuxt UI components',
        pass: read('app/components/resources/SettingCardDetail.vue').includes('<UForm')
            && read('app/components/resources/SettingCardDetail.vue').includes('<UInputTags')
            && read('app/components/resources/CardTypeDefinitionInspector.vue').includes('<USelect')
            && read('app/components/resources/CardTypeDefinitionInspector.vue').includes('<UButton')
            && read('app/components/resources/RelationInspector.vue').includes('<USelect')
            && read('app/components/resources/RelationTypeDefinitionInspector.vue').includes('<UInputTags')
            && !sprint4SurfaceText.includes('<input')
            && !sprint4SurfaceText.includes('<select')
            && !read('app/components/resources/ResourcesWorkspace.vue').includes('<button')
            && !read('app/components/outline/OutlinePlanningWorkspace.vue').includes('<button')
    },
    {
        name: 'Right-side Context panel is not mounted for project workspaces',
        pass: !read('app/components/workspace/ProjectWorkspaceRoot.vue').includes('ProjectWorkspaceInspector')
            && !read('app/components/workspace/ProjectWorkspaceRoot.vue').includes('has-inspector')
    },
    {
        name: 'CSS uses product-domain class names and semantic UI tokens',
        pass: read('nuxt.config.ts').includes('~/assets/css/sprint4.css')
            && read('app/assets/css/sprint4.css').includes('var(--ui-bg-muted)')
            && read('app/assets/css/sprint4.css').includes('var(--ui-bg-elevated)')
            && read('app/assets/css/sprint4.css').includes('var(--ui-border)')
            && !sprint4SurfaceText.includes('sprint4-')
            && !/#(?:[0-9a-fA-F]{3}){1,2}/.test(read('app/assets/css/sprint4.css'))
    },
    {
        name: 'No development-stage wording is exposed in component surfaces',
        pass: !allVueText.includes('Phase ')
            && !allVueText.includes('Week ')
            && !sprint4SurfaceText.includes('Sprint 4')
            && !sprint4SurfaceText.includes('sprint4-workspace')
            && !sprint4SurfaceText.includes('Inspector</strong>')
    },
    {
        name: 'uiref marks Sprint 4 as completed with implementation notes',
        pass: read('uiref.md').includes('[✓] 大纲看板')
            && read('uiref.md').includes('[✓] 自定义关系类型')
            && read('uiref.md').includes('Sprint 4 已完成')
    }
]

let failed = false
for (const check of checks) {
    if (check.pass) console.log(`✓ ${check.name}`)
    else {
        failed = true
        console.error(`✗ ${check.name}`)
    }
}

if (failed) process.exit(1)
console.log('Sprint 4 UI audit passed.')
