import {storeToRefs} from 'pinia'
import type {ProjectPrimaryView, ProjectWorkspaceMode} from '~/constants/projectViews'
import {toAppErrorMessage} from '~/utils/appError'
import {appCommandRegistry} from '~/constants/commandRegistry'
import {cardTypeLabel, defaultCardName, defaultFieldsJson} from '~/types/card'
import type {AppCommandId, CommandPaletteItem} from '~/types/command'
import type {ProjectStats, TodayWritingStats} from '~/types/stats'
import type {SaveState} from '~/composables/useAutoSave'
import {useAppShortcuts} from '~/composables/useAppShortcuts'
import type {DocumentCreatePayload, DocumentStatus, DocumentType, MoveDocumentInput} from '~/types/document'
import type {UpdateProjectInput} from '~/types/project'
import type {EditorSessionSnapshot, EditorSettings} from '~/types/editor'
import type {CreativeNote, EditorParagraphNoteRequest, NoteType} from '~/types/note'
import type {OpenTarget} from '~/types/navigation'

export function useProjectWorkspaceController() {
    const route = useRoute()
    const router = useRouter()
    const projectStore = useProjectStore()
    const documentStore = useDocumentStore()
    const settingsStore = useSettingsStore()
    const timerStore = useTimerStore()
    const noteStore = useNoteStore()
    const cardStore = useCardStore()
    const proofreadingStore = useProofreadingStore()
    const exportStore = useExportStore()
    const commandStore = useCommandStore()
    const navigationStore = useNavigationStore()
    const outlineStore = useOutlineStore()
    const timelineStore = useTimelineStore()
    const relationStore = useRelationStore()
    const foreshadowStore = useForeshadowStore()
    const exportTemplateStore = useExportTemplateStore()
    const {call} = useTauriInvoke()
    const {locale} = useI18n()

    const projectId = computed(() => String(route.params.id))
    const shellStore = useProjectShellStore()
    const {
        workspaceMode,
        focusMode,
        feedback: commandFeedback
    } = storeToRefs(shellStore)

    const projectStats = ref<ProjectStats | null>(null)
    const todayStats = ref<TodayWritingStats | null>(null)
    const pageError = ref<string | null>(null)
    const targetDraft = ref<number | null>(null)
    const currentDocumentNotes = ref<CreativeNote[]>([])
    const projectSaving = ref(false)
    let backupInterval: ReturnType<typeof setInterval> | null = null

    const editorSnapshot = reactive({
        saveState: 'idle' as SaveState,
        characterCount: 0,
        lastSavedAt: null as number | null,
        errorMessage: null as string | null,
        sessionElapsedMs: 0,
        sessionDelta: 0
    })

    const activeDocument = computed(() => documentStore.activeDocument)
    const activeDocumentTarget = computed(() => getDocumentTarget(activeDocument.value?.metadataJson))

    watch(activeDocumentTarget, value => {
        targetDraft.value = value ?? null
    }, {immediate: true})

    watch(() => documentStore.activeDocumentId, async () => {
        await refreshCurrentDocumentNotes()
    })

    const saveStateLabel = computed(() => {
        switch (editorSnapshot.saveState) {
            case 'dirty':
                return '有未保存修改'
            case 'saving':
                return '保存中'
            case 'saved':
                return '已保存'
            case 'failed':
                return '保存失败'
            default:
                return '等待输入'
        }
    })

    const themeLabel = computed(() => {
        return settingsStore.themeOptions.find(option => option.value === settingsStore.theme)?.label ?? settingsStore.theme
    })


    const commandPaletteItems = computed<CommandPaletteItem[]>(() => {
        const commandItems: CommandPaletteItem[] = appCommandRegistry
            .filter(command => command.id !== 'commandPalette.open')
            .map(command => ({
                id: `command:${command.id}`,
                kind: 'command',
                title: command.title,
                subtitle: command.description,
                group: command.group,
                keywords: command.keywords,
                shortcut: commandStore.shortcutFor(command.id),
                action: {type: 'command', commandId: command.id}
            }))

        const documentItems: CommandPaletteItem[] = documentStore.documents.map(document => ({
            id: `document:${document.id}`,
            kind: 'document',
            title: document.title,
            subtitle: `${documentTypeLabel(document.type)} · ${documentStatusLabel(document.status)} · ${document.characterCount} 字`,
            group: '打开章节',
            keywords: ['章节', '卷', '场景', document.type, document.status],
            action: {type: 'openDocument', targetId: document.id}
        }))

        const cardItems: CommandPaletteItem[] = cardStore.cards.map(card => ({
            id: `card:${card.id}`,
            kind: 'card',
            title: card.name,
            subtitle: `${cardTypeLabel(card.type)} · ${card.description || '暂无简介'}`,
            group: '打开设定',
            keywords: [card.type, card.aliasesJson, card.description],
            action: {type: 'openCard', targetId: card.id}
        }))

        const noteItems: CommandPaletteItem[] = noteStore.notes.map(note => ({
            id: `note:${note.id}`,
            kind: 'note',
            title: note.title,
            subtitle: `${noteTypeLabel(note.type)} · ${note.status === 'done' ? '已完成' : '未完成'}${note.documentTitle ? ` · ${note.documentTitle}` : ''}`,
            group: '打开事项',
            keywords: [note.type, note.status, note.priority, note.body],
            action: {type: 'openNote', targetId: note.id}
        }))

        return [...commandItems, ...documentItems, ...cardItems, ...noteItems]
    })

    watch(
        commandPaletteItems,
        items => shellStore.setCommandContext(items, executeCommandPaletteItem),
        {immediate: true}
    )

    useAppShortcuts({
        'commandPalette.open': toggleCommandPalette,
        'document.createChapter': () => executeAppCommandSafely('document.createChapter'),
        'card.createCharacter': () => executeAppCommandSafely('card.createCharacter'),
        'editor.toggleFocus': () => executeAppCommandSafely('editor.toggleFocus'),
        'proofreading.runCurrent': () => executeAppCommandSafely('proofreading.runCurrent'),
        'export.currentDocument': () => executeAppCommandSafely('export.currentDocument'),
        'theme.cycle': () => executeAppCommandSafely('theme.cycle'),
        'navigation.settings': () => executeAppCommandSafely('navigation.settings')
    })

    onMounted(async () => {
        await loadProjectWorkspace()
        commandStore.loadShortcuts()
        startBackupInterval()
    })

    onBeforeUnmount(() => {
        stopBackupInterval()
        commandStore.closePalette()
        shellStore.reset()
        timerStore.finishSession()
    })

    async function loadProjectWorkspace() {
        pageError.value = null
        try {
            await settingsStore.loadAppSettings()
            locale.value = settingsStore.language
            await projectStore.loadProject(projectId.value)
            await documentStore.loadDocuments(projectId.value)
            await refreshStats()
            await refreshTodayStats()
            await refreshCurrentDocumentNotes()
            await createStartupBackup()
        } catch (error) {
            pageError.value = toAppErrorMessage(error, '加载项目失败')
        }
    }

    async function refreshStats() {
        projectStats.value = await projectStore.loadProjectStats(projectId.value)
        shellStore.setProjectCharacterCount(projectStats.value.characterCount)
    }

    async function refreshTodayStats() {
        const range = getTodayRange()
        todayStats.value = await call<TodayWritingStats>('get_today_writing_stats', {
            projectId: projectId.value,
            dayStart: range.dayStart,
            dayEnd: range.dayEnd
        })
        timerStore.setTodayStats(todayStats.value)
    }


    async function refreshCurrentDocumentNotes() {
        if (!documentStore.activeDocumentId) {
            currentDocumentNotes.value = []
            return
        }
        try {
            currentDocumentNotes.value = (await noteStore.loadDocumentNotes(projectId.value, documentStore.activeDocumentId, null))
                .filter(note => note.status === 'open' || note.status === 'doing')
        } catch (error) {
            console.warn('[notes] failed to refresh current document notes', error)
            currentDocumentNotes.value = []
        }
    }

    function selectPrimaryView(viewId: ProjectPrimaryView) {
        shellStore.selectPrimaryView(viewId)
    }

    function selectWorkspaceMode(mode: ProjectWorkspaceMode) {
        shellStore.selectWorkspaceMode(mode)
    }

    async function openCommandPalette() {
        commandStore.openPalette()
        await Promise.allSettled([
            cardStore.loadCards(projectId.value),
            noteStore.loadNotes({projectId: projectId.value, type: 'all', status: 'all'})
        ])
    }

    function toggleCommandPalette() {
        if (commandStore.isPaletteOpen) {
            commandStore.closePalette()
            return
        }
        void openCommandPalette()
    }

    async function executeCommandPaletteItem(item: CommandPaletteItem) {
        commandStore.closePalette()
        pageError.value = null
        try {
            if (item.action.type === 'command') {
                await executeAppCommandSafely(item.action.commandId)
                return
            }
            if (item.action.type === 'openDocument') {
                await openTarget({type: 'document', targetId: item.action.targetId, source: 'direct'})
                return
            }
            if (item.action.type === 'openCard') {
                await openTarget({type: 'card', targetId: item.action.targetId})
                return
            }
            if (item.action.type === 'openNote') {
                await openTarget({type: 'note', targetId: item.action.targetId})
            }
        } catch (error) {
            pageError.value = toAppErrorMessage(error, '执行命令失败')
        }
    }

    async function executeAppCommandSafely(commandId: AppCommandId) {
        pageError.value = null
        try {
            await executeAppCommand(commandId)
        } catch (error) {
            const message = toAppErrorMessage(error, '执行命令失败')
            pageError.value = message
            showCommandFeedback(message, 8000)
        }
    }

    async function executeAppCommand(commandId: AppCommandId) {
        switch (commandId) {
            case 'commandPalette.open':
                await openCommandPalette()
                break
            case 'document.createChapter':
                await commandCreateChapter()
                break
            case 'card.createCharacter':
                await commandCreateCharacter()
                break
            case 'editor.toggleFocus':
                commandToggleFocusMode()
                break
            case 'proofreading.runCurrent':
                await commandRunProofreading()
                break
            case 'export.currentDocument':
                await commandExportCurrentDocument()
                break
            case 'theme.cycle':
                await commandCycleTheme()
                break
            case 'navigation.settings':
                commandOpenSettings()
                break
        }
    }

    async function commandCreateChapter() {
        const parentId = resolveChapterParentId()
        const document = await documentStore.createDocument({
            projectId: projectId.value,
            parentId,
            type: 'chapter',
            title: '新章节',
            sortOrder: null
        })
        workspaceMode.value = 'writing'
        await refreshStats()
        showCommandFeedback(`已创建“${document.title}”。`)
    }

    function resolveChapterParentId() {
        let cursor = activeDocument.value
        const visited = new Set<string>()
        while (cursor && !visited.has(cursor.id)) {
            visited.add(cursor.id)
            if (cursor.type === 'volume') return cursor.id
            const parentId = cursor.parentId
            cursor = parentId
                ? documentStore.documents.find(document => document.id === parentId) ?? null
                : null
        }
        return null
    }

    async function commandCreateCharacter() {
        if (!cardStore.cards.length) await cardStore.loadCards(projectId.value)
        const card = await cardStore.createCard({
            projectId: projectId.value,
            type: 'character',
            name: defaultCardName('character'),
            aliasesJson: '[]',
            description: '',
            fieldsJson: defaultFieldsJson('character'),
            avatarAssetId: null
        })
        cardStore.setTypeFilter('character')
        cardStore.selectCard(card.id)
        workspaceMode.value = 'cards'
        showCommandFeedback('已创建人物设定卡。')
    }

    function commandToggleFocusMode() {
        shellStore.toggleFocusMode()
        showCommandFeedback(focusMode.value ? '已进入专注模式。' : '已退出专注模式。')
    }

    async function commandRunProofreading() {
        if (!documentStore.activeDocumentId) {
            throw new Error('请先选择需要校对的章节或场景')
        }
        await proofreadingStore.loadRules({
            projectId: projectId.value,
            includeBuiltin: true
        })
        const issues = await proofreadingStore.runOnDocument({
            documentId: documentStore.activeDocumentId,
            enabledOnly: true,
            ruleIds: null
        })
        workspaceMode.value = 'proofreading'
        showCommandFeedback(`校对完成，共发现 ${issues.length} 个问题。`)
    }

    async function commandExportCurrentDocument() {
        if (!documentStore.activeDocumentId) {
            throw new Error('请先选择需要导出的章节或场景')
        }
        const result = await exportStore.exportDocuments({
            projectId: projectId.value,
            format: 'txt',
            range: 'current',
            documentId: documentStore.activeDocumentId,
            documentIds: null,
            outputPath: null
        })
        showCommandFeedback(`当前文档已导出到：${result.path}`, 8000)
    }

    async function commandCycleTheme() {
        const themes = settingsStore.themeOptions.map(option => option.value)
        const index = themes.indexOf(settingsStore.theme)
        const next = themes[(index + 1) % themes.length] ?? 'paper'
        await settingsStore.setTheme(next)
        showCommandFeedback(`已切换为“${settingsStore.themeOptions.find(option => option.value === next)?.label ?? next}”主题。`)
    }

    function commandOpenSettings() {
        shellStore.openSettings()
        showCommandFeedback('已打开设置。')
    }

    function showCommandFeedback(message: string, duration = 4000) {
        shellStore.showFeedback(message, duration)
    }


    async function createChapterNote(type: Exclude<NoteType, 'all'> = 'todo') {
        if (!documentStore.activeDocumentId) return
        pageError.value = null
        try {
            await noteStore.createNote({
                projectId: projectId.value,
                documentId: documentStore.activeDocumentId,
                paragraphId: null,
                type,
                title: type === 'foreshadow' ? '新的伏笔' : type === 'memo' ? '新的备忘' : '新的待办',
                body: '',
                priority: type === 'foreshadow' ? 'high' : 'normal'
            })
            await refreshCurrentDocumentNotes()
        } catch (error) {
            pageError.value = toAppErrorMessage(error, '创建事项失败')
        }
    }

    async function handleParagraphNote(payload: EditorParagraphNoteRequest) {
        if (!documentStore.activeDocumentId) return
        pageError.value = null
        try {
            await noteStore.createNote({
                projectId: projectId.value,
                documentId: documentStore.activeDocumentId,
                paragraphId: payload.paragraphId,
                type: payload.type,
                title: paragraphNoteTitle(payload),
                body: payload.selectedText,
                priority: payload.type === 'foreshadow' ? 'high' : 'normal'
            })
            await refreshCurrentDocumentNotes()
        } catch (error) {
            pageError.value = toAppErrorMessage(error, '创建段落事项失败')
        }
    }

    async function markNoteDone(noteId: string) {
        pageError.value = null
        try {
            await noteStore.updateNoteStatus(noteId, 'done')
            await refreshCurrentDocumentNotes()
        } catch (error) {
            pageError.value = toAppErrorMessage(error, '更新事项失败')
        }
    }

    async function createStartupBackup() {
        try {
            await exportStore.createBackup(projectId.value)
            await exportStore.listBackups(projectId.value)
        } catch (error) {
            console.warn('[backup] startup backup failed', error)
        }
    }

    function startBackupInterval() {
        stopBackupInterval()
        backupInterval = setInterval(() => {
            void exportStore.createBackup(projectId.value)
                .then(() => exportStore.listBackups(projectId.value))
                .catch(error => console.warn('[backup] scheduled backup failed', error))
        }, 15 * 60 * 1000)
    }

    function stopBackupInterval() {
        if (backupInterval) {
            clearInterval(backupInterval)
            backupInterval = null
        }
    }

    async function createManualBackup() {
        pageError.value = null
        try {
            await exportStore.createBackup(projectId.value)
            await exportStore.listBackups(projectId.value)
        } catch (error) {
            pageError.value = toAppErrorMessage(error, '创建备份失败')
        }
    }

    async function updateProjectInfo(input: UpdateProjectInput) {
        projectSaving.value = true
        pageError.value = null
        try {
            await projectStore.updateProject(input)
            await projectStore.loadProjects()
        } catch (error) {
            pageError.value = toAppErrorMessage(error, '保存项目信息失败')
        } finally {
            projectSaving.value = false
        }
    }

    async function deleteCurrentProject() {
        pageError.value = null
        try {
            await projectStore.deleteProject(projectId.value)
            await router.push('/')
        } catch (error) {
            pageError.value = toAppErrorMessage(error, '删除项目失败')
        }
    }

    async function handleImportedDocument() {
        await documentStore.loadDocuments(projectId.value)
        await refreshStats()
    }

    async function openTarget(target: OpenTarget) {
        pageError.value = null
        navigationStore.rememberTarget(target)

        try {
            switch (target.type) {
                case 'workspace':
                    focusMode.value = false
                    workspaceMode.value = target.workspace
                    break

                case 'document':
                    if (!documentStore.documents.some(document => document.id === target.targetId)) {
                        await documentStore.loadDocuments(projectId.value)
                    }
                    if (!documentStore.documents.some(document => document.id === target.targetId)) {
                        throw new Error('目标文档不存在或已被删除')
                    }
                    navigationStore.requestDocumentNavigation(target)
                    documentStore.selectDocument(target.targetId)
                    workspaceMode.value = 'writing'
                    break

                case 'card':
                    cardStore.setTypeFilter('all')
                    await cardStore.loadCards(projectId.value)
                    if (!cardStore.cards.some(card => card.id === target.targetId)) {
                        throw new Error('目标设定卡不存在或已被删除')
                    }
                    cardStore.selectCard(target.targetId)
                    workspaceMode.value = 'cards'
                    break

                case 'note':
                    noteStore.setTypeFilter('all')
                    noteStore.setStatusFilter('all')
                    await noteStore.loadNotes({projectId: projectId.value, type: 'all', status: 'all'})
                    if (!noteStore.notes.some(note => note.id === target.targetId)) {
                        throw new Error('目标事项不存在或已被删除')
                    }
                    noteStore.selectNote(target.targetId)
                    workspaceMode.value = 'notes'
                    break

                case 'outline':
                    await outlineStore.loadOutlineNodes(projectId.value)
                    outlineStore.selectOutlineNode(target.targetId)
                    workspaceMode.value = 'outline'
                    break

                case 'timeline':
                    timelineStore.setFilters(null, null)
                    await timelineStore.loadTimeline(projectId.value)
                    timelineStore.selectEvent(target.targetId)
                    workspaceMode.value = 'timeline'
                    break

                case 'relation':
                    await relationStore.loadGraph(projectId.value)
                    relationStore.selectRelation(target.targetId)
                    workspaceMode.value = 'relations'
                    break

                case 'foreshadow':
                    foreshadowStore.setFilters('all', 'all', false)
                    await foreshadowStore.loadThreads(projectId.value)
                    foreshadowStore.selectThread(target.targetId)
                    workspaceMode.value = 'foreshadow'
                    break

                case 'proofreadingRule':
                    await proofreadingStore.loadRules({projectId: projectId.value, includeBuiltin: true})
                    proofreadingStore.selectRule(target.targetId)
                    workspaceMode.value = 'proofreading'
                    break

                case 'proofreadingIssue':
                    proofreadingStore.selectIssue(target.issue.id)
                    await openTarget({
                        type: 'document',
                        targetId: target.issue.documentId,
                        paragraphId: target.issue.paragraphId,
                        startOffset: target.issue.startOffset,
                        endOffset: target.issue.endOffset,
                        source: 'proofreading',
                        label: target.issue.message
                    })
                    return

                case 'exportTemplate':
                    await exportTemplateStore.loadTemplates({projectId: projectId.value, includeBuiltin: true})
                    exportTemplateStore.selectTemplate(target.targetId)
                    workspaceMode.value = 'export'
                    break
            }

            navigationStore.setNavigationMessage(targetMessage(target))
            showCommandFeedback(targetMessage(target))
        } catch (error) {
            const message = toAppErrorMessage(error, '无法打开目标')
            pageError.value = message
            navigationStore.setNavigationMessage(message)
            showCommandFeedback(message, 8000)
        }
    }

    function targetMessage(target: OpenTarget) {
        switch (target.type) {
            case 'document':
                return target.paragraphId || target.startOffset != null ? '已定位到正文位置。' : '已打开文档。'
            case 'card':
                return '已打开设定卡。'
            case 'note':
                return '已打开事项。'
            case 'outline':
                return '已打开大纲节点。'
            case 'timeline':
                return '已打开时间线事件。'
            case 'relation':
                return '已打开设定关系。'
            case 'foreshadow':
                return '已打开伏笔线程。'
            case 'proofreadingRule':
                return '已打开校对规则。'
            case 'proofreadingIssue':
                return '已定位校对问题。'
            case 'exportTemplate':
                return '已打开导出模板。'
            case 'workspace':
                return '已切换工作区。'
        }
    }


    async function createDocument(payload: DocumentCreatePayload) {
        pageError.value = null
        try {
            await documentStore.createDocument({
                projectId: projectId.value,
                parentId: payload.parentId ?? null,
                type: payload.type,
                title: payload.title,
                sortOrder: payload.sortOrder ?? null
            })
            await refreshStats()
        } catch (error) {
            pageError.value = toAppErrorMessage(error, '创建文档失败')
        }
    }

    function selectDocument(documentId: string) {
        documentStore.selectDocument(documentId)
    }

    async function renameDocument(documentId: string, title: string) {
        pageError.value = null
        try {
            await documentStore.renameDocument(documentId, title)
            await refreshStats()
        } catch (error) {
            pageError.value = toAppErrorMessage(error, '重命名文档失败')
        }
    }

    async function deleteDocument(documentId: string) {
        pageError.value = null
        try {
            await documentStore.deleteDocument(documentId)
            await refreshStats()
        } catch (error) {
            pageError.value = toAppErrorMessage(error, '删除文档失败')
        }
    }

    async function moveDocument(input: MoveDocumentInput) {
        pageError.value = null
        try {
            await documentStore.moveDocument(input)
            await refreshStats()
        } catch (error) {
            pageError.value = toAppErrorMessage(error, '移动文档失败')
        }
    }

    async function updateDocumentStatus(documentId: string, status: DocumentStatus) {
        pageError.value = null
        try {
            await documentStore.updateDocumentStatus({documentId, status})
            await refreshStats()
        } catch (error) {
            pageError.value = toAppErrorMessage(error, '更新文档状态失败')
        }
    }

    async function saveDocumentTarget() {
        if (!activeDocument.value) return
        pageError.value = null
        try {
            const target = typeof targetDraft.value === 'number' && targetDraft.value > 0
                ? Math.round(targetDraft.value)
                : null
            await documentStore.updateDocumentGoal({
                documentId: activeDocument.value.id,
                targetCharacterCount: target
            })
        } catch (error) {
            pageError.value = toAppErrorMessage(error, '更新目标字数失败')
        }
    }

    async function handleSaved() {
        await refreshStats()
        await refreshTodayStats()
        await refreshCurrentDocumentNotes()
    }

    function handleEditorSession(payload: EditorSessionSnapshot) {
        editorSnapshot.sessionElapsedMs = payload.elapsedMs
        editorSnapshot.sessionDelta = payload.sessionDelta
    }

    function handleEditorStatus(payload: {
        saveState: SaveState
        characterCount: number
        lastSavedAt: number | null
        errorMessage: string | null
        sessionElapsedMs: number
        sessionDelta: number
    }) {
        editorSnapshot.saveState = payload.saveState
        editorSnapshot.characterCount = payload.characterCount
        editorSnapshot.lastSavedAt = payload.lastSavedAt
        editorSnapshot.errorMessage = payload.errorMessage
        editorSnapshot.sessionElapsedMs = payload.sessionElapsedMs
        editorSnapshot.sessionDelta = payload.sessionDelta
    }

    function toggleFocusMode() {
        shellStore.toggleFocusMode()
    }

    async function updateEditorSetting(key: keyof EditorSettings, event: Event) {
        const target = event.target as HTMLInputElement | HTMLSelectElement
        try {
            if (key === 'fontFamily') {
                await settingsStore.updateEditorSetting('fontFamily', target.value as EditorSettings['fontFamily'])
                return
            }

            const value = Number(target.value)
            if (!Number.isFinite(value)) return

            if (key === 'fontSize') await settingsStore.updateEditorSetting('fontSize', value)
            else if (key === 'lineHeight') await settingsStore.updateEditorSetting('lineHeight', value)
            else if (key === 'pageWidth') await settingsStore.updateEditorSetting('pageWidth', value)
            else if (key === 'autosaveIntervalMs') await settingsStore.updateEditorSetting('autosaveIntervalMs', value)
        } catch (error) {
            pageError.value = toAppErrorMessage(error, '保存编辑器设置失败')
        }
    }

    function getDocumentTarget(metadataJson?: string | null) {
        if (!metadataJson) return null
        try {
            const parsed = JSON.parse(metadataJson) as { targetCharacterCount?: unknown }
            return typeof parsed.targetCharacterCount === 'number' && parsed.targetCharacterCount > 0
                ? parsed.targetCharacterCount
                : null
        } catch {
            return null
        }
    }

    function getTodayRange() {
        const now = new Date()
        const start = new Date(now.getFullYear(), now.getMonth(), now.getDate())
        const end = new Date(start)
        end.setDate(start.getDate() + 1)
        return {
            dayStart: start.getTime(),
            dayEnd: end.getTime()
        }
    }

    function paragraphNoteTitle(payload: EditorParagraphNoteRequest) {
        const prefix = payload.type === 'foreshadow'
            ? '段落伏笔'
            : payload.type === 'todo'
                ? '段落待办'
                : '段落备忘'
        const selected = payload.selectedText.trim().replace(/\s+/g, ' ').slice(0, 28)
        return selected ? `${prefix}：${selected}` : prefix
    }

    function noteTypeLabel(type: string) {
        switch (type) {
            case 'memo':
                return '备忘'
            case 'todo':
                return '待办'
            case 'foreshadow':
                return '伏笔'
            case 'issue':
                return '问题'
            case 'idea':
                return '灵感'
            default:
                return type
        }
    }

    function notePriorityLabel(priority: string) {
        switch (priority) {
            case 'high':
                return '高'
            case 'low':
                return '低'
            default:
                return '普通'
        }
    }

    function documentTypeLabel(type: DocumentType) {
        switch (type) {
            case 'volume':
                return '卷'
            case 'scene':
                return '场景'
            case 'chapter':
                return '章节'
            default:
                return '文档'
        }
    }

    function documentStatusLabel(status: DocumentStatus) {
        switch (status) {
            case 'writing':
                return '写作中'
            case 'done':
                return '完成'
            case 'draft':
                return '草稿'
            case 'revised':
                return '已修订'
            case 'archived':
                return '已归档'
            default:
                return String(status)
        }
    }


    function formatDate(timestamp: number) {
        return new Intl.DateTimeFormat('zh-CN', {
            month: '2-digit',
            day: '2-digit',
            hour: '2-digit',
            minute: '2-digit',
            second: '2-digit'
        }).format(new Date(timestamp))
    }

    function formatDuration(ms: number) {
        const totalSeconds = Math.floor(ms / 1000)
        const hours = Math.floor(totalSeconds / 3600)
        const minutes = Math.floor((totalSeconds % 3600) / 60)
        const seconds = totalSeconds % 60
        if (hours > 0) return `${hours}:${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}`
        return `${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}`
    }


    return {
        projectId,
        projectStats,
        todayStats,
        pageError,
        commandFeedback,
        focusMode,
        workspaceMode,
        targetDraft,
        currentDocumentNotes,
        projectSaving,
        editorSnapshot,
        projectStore,
        documentStore,
        settingsStore,
        timerStore,
        noteStore,
        exportStore,
        shellStore,
        activeDocument,
        activeDocumentTarget,
        saveStateLabel,
        themeLabel,
        selectPrimaryView,
        selectWorkspaceMode,
        openCommandPalette,
        executeCommandPaletteItem,
        createChapterNote,
        handleParagraphNote,
        markNoteDone,
        createManualBackup,
        updateProjectInfo,
        deleteCurrentProject,
        handleImportedDocument,
        openTarget,
        createDocument,
        selectDocument,
        renameDocument,
        deleteDocument,
        moveDocument,
        updateDocumentStatus,
        saveDocumentTarget,
        handleSaved,
        handleEditorSession,
        handleEditorStatus,
        toggleFocusMode,
        updateEditorSetting,
        noteTypeLabel,
        notePriorityLabel,
        documentTypeLabel,
        documentStatusLabel,
        formatDate,
        formatDuration
    }
}
