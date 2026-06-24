<template>
  <AppShell
      subtitle="项目"
      title="Mythistorima"
      @home="refresh"
      @settings="showSettings = true"
  >
    <section class="project-hub">
      <header class="project-hub-header">
        <div>
          <p class="project-hub-eyebrow">Project Hub</p>
          <h1>你的小说项目</h1>
          <p>继续最近的创作，或建立一个新的故事世界。</p>
        </div>

        <div class="project-hub-actions">
          <UTooltip text="刷新项目列表">
            <UButton
                :loading="projectStore.loading"
                aria-label="刷新项目列表"
                color="neutral"
                icon="i-lucide-refresh-cw"
                size="sm"
                square
                variant="ghost"
                @click="refresh"
            />
          </UTooltip>
          <UButton
              icon="i-lucide-plus"
              label="新建项目"
              size="sm"
              @click="showCreate = true"
          />
        </div>
      </header>

      <UAlert
          v-if="loadError"
          :description="loadError"
          class="mb-3"
          color="error"
          icon="i-lucide-circle-alert"
          title="无法读取项目"
          variant="subtle"
      />

      <UAlert
          v-if="operationMessage"
          :description="operationMessage"
          class="mb-3"
          color="success"
          icon="i-lucide-circle-check"
          variant="subtle"
      />

      <UCard :ui="{ body: 'p-0' }">
        <div class="project-hub-toolbar">
          <UTabs
              v-model="viewMode"
              :content="false"
              :items="viewTabs"
              size="sm"
          />
          <UInput
              v-model="query"
              class="project-hub-search"
              icon="i-lucide-search"
              placeholder="搜索项目、作者或简介"
              size="sm"
          />
        </div>

        <ProjectList
            :loading="projectStore.loading"
            :projects="visibleProjects"
            :stats-by-id="projectStore.projectListStats"
            @backup="backupProject"
            @delete="deleteProject"
            @open="openProject"
            @rename="startRename"
        />
      </UCard>
    </section>

    <ProjectCreateModal
        :error="createError"
        :open="showCreate"
        :submitting="creating"
        @close="showCreate = false"
        @submit="createProject"
    />

    <ProjectRenameModal
        :error="renameError"
        :open="Boolean(renamingProject)"
        :project="renamingProject"
        :submitting="renaming"
        @close="renamingProject = null"
        @submit="renameProject"
    />

    <AppSettingsModal v-model:open="showSettings"/>
  </AppShell>
</template>

<script lang="ts" setup>
import AppShell from '~/components/layout/AppShell.vue'
import AppSettingsModal from '~/components/settings/AppSettingsModal.vue'
import ProjectCreateModal from '~/components/project/ProjectCreateModal.vue'
import ProjectRenameModal from '~/components/project/ProjectRenameModal.vue'
import ProjectList from '~/components/project/ProjectList.vue'
import type {CreateProjectInput, Project} from '~/types/project'
import {toAppErrorMessage} from '~/utils/appError'

const router = useRouter()
const projectStore = useProjectStore()
const exportStore = useExportStore()

const showCreate = ref(false)
const showSettings = ref(false)
const creating = ref(false)
const createError = ref<string | null>(null)
const loadError = ref<string | null>(null)
const operationMessage = ref<string | null>(null)
const query = ref('')
const viewMode = ref<'recent' | 'all'>('recent')
const renamingProject = ref<Project | null>(null)
const renaming = ref(false)
const renameError = ref<string | null>(null)

const viewTabs = [
  {label: '最近', value: 'recent', icon: 'i-lucide-clock-3'},
  {label: '全部', value: 'all', icon: 'i-lucide-library'}
]

const filteredProjects = computed(() => {
  const normalized = query.value.trim().toLowerCase()
  if (!normalized) return projectStore.projects
  return projectStore.projects.filter(project =>
      [project.title, project.author, project.description]
          .filter(Boolean)
          .some(value => String(value).toLowerCase().includes(normalized))
  )
})

const visibleProjects = computed(() => {
  const projects = filteredProjects.value
  return viewMode.value === 'recent' ? projects.slice(0, 6) : projects
})

onMounted(refresh)

async function refresh() {
  loadError.value = null
  operationMessage.value = null
  try {
    await projectStore.ping()
    await projectStore.checkDatabase()
    await projectStore.loadAppInfo()
    await projectStore.loadProjects()
    await projectStore.loadProjectListStats()
  } catch (error) {
    loadError.value = toAppErrorMessage(error, '请确认应用已正常启动后重试')
  }
}

async function createProject(input: CreateProjectInput) {
  creating.value = true
  createError.value = null

  try {
    const project = await projectStore.createProject(input)
    showCreate.value = false
    await router.push(`/project/${project.id}`)
  } catch (error) {
    createError.value = toAppErrorMessage(error, '创建项目失败')
  } finally {
    creating.value = false
  }
}

async function openProject(projectId: string) {
  await router.push(`/project/${projectId}`)
}

function startRename(project: Project) {
  renameError.value = null
  renamingProject.value = project
}

async function renameProject(title: string) {
  const project = renamingProject.value
  if (!project) return
  renaming.value = true
  renameError.value = null
  try {
    await projectStore.updateProject({
      projectId: project.id,
      title,
      author: project.author ?? null,
      description: project.description ?? null,
      language: project.language,
      status: project.status,
      targetCharacterCount: project.targetCharacterCount ?? null,
      dailyTargetCount: project.dailyTargetCount ?? null,
      metadataJson: project.metadataJson
    })
    operationMessage.value = `项目已重命名为“${title}”。`
    renamingProject.value = null
  } catch (error) {
    renameError.value = toAppErrorMessage(error, '重命名项目失败')
  } finally {
    renaming.value = false
  }
}

async function backupProject(projectId: string) {
  operationMessage.value = null
  loadError.value = null
  try {
    await exportStore.createBackup(projectId)
    operationMessage.value = '本地备份已创建。'
  } catch (error) {
    loadError.value = toAppErrorMessage(error, '创建备份失败')
  }
}

async function deleteProject(projectId: string) {
  const project = projectStore.projects.find(item => item.id === projectId)
  const confirmed = window.confirm(`确定删除项目“${project?.title ?? '未命名项目'}”吗？此操作会删除项目内的全部内容。`)
  if (!confirmed) return

  try {
    await projectStore.deleteProject(projectId)
    operationMessage.value = '项目已删除。'
  } catch (error) {
    loadError.value = toAppErrorMessage(error, '删除项目失败')
  }
}
</script>
