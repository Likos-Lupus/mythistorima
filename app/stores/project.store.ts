import type {AppInfo, ProjectStats} from '~/types/stats'
import type {CreateProjectInput, Project} from '~/types/project'

export const useProjectStore = defineStore('project', () => {
    const projects = ref<Project[]>([])
    const currentProject = ref<Project | null>(null)
    const appInfo = ref<AppInfo | null>(null)
    const databaseReady = ref(false)
    const loading = ref(false)

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

    async function loadProject(projectId: string) {
        currentProject.value = await call<Project>('get_project', {projectId})
        return currentProject.value
    }

    async function createProject(input: CreateProjectInput) {
        const project = await call<Project>('create_project', {input})
        projects.value = [project, ...projects.value.filter(item => item.id !== project.id)]
        currentProject.value = project
        return project
    }

    async function deleteProject(projectId: string) {
        await call<boolean>('delete_project', {projectId})
        projects.value = projects.value.filter(item => item.id !== projectId)
        if (currentProject.value?.id === projectId) currentProject.value = null
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
        ping,
        loadAppInfo,
        checkDatabase,
        loadProjects,
        loadProject,
        createProject,
        deleteProject,
        loadProjectStats
    }
})
