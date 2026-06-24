import type {InjectionKey} from 'vue'
import type {useProjectWorkspaceController} from '~/composables/useProjectWorkspaceController'

export type ProjectWorkspaceController = ReturnType<typeof useProjectWorkspaceController>

const projectWorkspaceKey: InjectionKey<ProjectWorkspaceController> = Symbol('project-workspace')

export function provideProjectWorkspace(controller: ProjectWorkspaceController) {
    provide(projectWorkspaceKey, controller)
    return controller
}

export function useProjectWorkspaceContext() {
    const controller = inject(projectWorkspaceKey)
    if (!controller) {
        throw new Error('Project workspace context is not available')
    }
    return controller
}
