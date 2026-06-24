import type {AppInfo, ProjectOverview, ProjectOverviewInput, ProjectStats} from '~/types/stats'
import type {CreateProjectInput, Project, UpdateProjectInput} from '~/types/project'

export const useProjectStore = defineStore('project', () => {
    const projects = ref<Project[]>([])
    const currentProject = ref<Project | null>(null)
    const appInfo = ref<AppInfo | null>(null)
    const databaseReady = ref(false)
    const loading = ref(false)
    const projectListStats = ref<Record<string, ProjectStats>>({})
    const currentOverview = ref<ProjectOverview | null>(null)

    const {call} = useTauriInvoke()

    async function ping() {
        return call<string>('app_ping')
    }

    async function loadAppInfo() {
        appInfo.value = await call<AppInfo>('get_app_info')
        return appInfo.value
    }

    async function checkDatabase() {
        databaseReady.value = await call<boolean>('db_health_check')
        return databaseReady.value
    }

    async function loadProjects() {
        loading.value = true
        try {
            projects.value = await call<Project[]>('list_projects')
            return projects.value
        } finally {
            loading.value = false
        }
    }


    async function loadProjectListStats() {
        const entries = await Promise.all(projects.value.map(async project => {
            try {
                const stats = await call<ProjectStats>('get_project_stats', {projectId: project.id})
                return [project.id, stats] as const
            } catch {
                return null
            }
        }))
        projectListStats.value = Object.fromEntries(entries.filter(Boolean) as Array<readonly [string, ProjectStats]>)
        return projectListStats.value
    }

    async function loadProjectOverview(input: ProjectOverviewInput) {
        currentOverview.value = await call<ProjectOverview>('get_project_overview', {input})
        return currentOverview.value
    }

    async function loadProject(projectId: string) {
        currentProject.value = await call<Project>('get_project', {projectId})
        return currentProject.value
    }

    async function createProject(input: CreateProjectInput) {
        const project = await call<Project>('create_project', {input})
        projects.value = [project, ...projects.value.filter(item => item.id !== project.id)]
        projectListStats.value = {
            ...projectListStats.value,
            [project.id]: {
                projectId: project.id,
                documentCount: 1,
                characterCount: 0,
                updatedAt: project.updatedAt
            }
        }
        currentProject.value = project
        return project
    }

    async function updateProject(input: UpdateProjectInput) {
        const project = await call<Project>('update_project', {input})
        projects.value = projects.value.map(item => item.id === project.id ? project : item)
        if (currentProject.value?.id === project.id) currentProject.value = project
        return project
    }

    async function deleteProject(projectId: string) {
        await call<boolean>('delete_project', {projectId})
        projects.value = projects.value.filter(item => item.id !== projectId)
        const nextStats = {...projectListStats.value}
        delete nextStats[projectId]
        projectListStats.value = nextStats
        if (currentProject.value?.id === projectId) {
            currentProject.value = null
            currentOverview.value = null
        }
    }

    async function loadProjectStats(projectId: string) {
        return call<ProjectStats>('get_project_stats', {projectId})
    }

    return {
        projects,
        currentProject,
        appInfo,
        databaseReady,
        loading,
        projectListStats,
        currentOverview,
        ping,
        loadAppInfo,
        checkDatabase,
        loadProjects,
        loadProjectListStats,
        loadProjectOverview,
        loadProject,
        createProject,
        updateProject,
        deleteProject,
        loadProjectStats
    }
})
