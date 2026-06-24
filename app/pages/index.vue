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
          <h2>最近项目</h2>
          <p>继续创作，或创建一个新的小说项目。</p>
        </div>

        <div class="project-hub-actions">
          <UButton
              :loading="projectStore.loading"
              color="neutral"
              icon="i-lucide-refresh-cw"
              label="刷新"
              size="sm"
              variant="ghost"
              @click="refresh"
          />
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

      <UCard :ui="{ body: 'p-3 sm:p-4' }">
        <ProjectList
            :loading="projectStore.loading"
            :projects="projectStore.projects"
            @delete="deleteProject"
            @open="openProject"
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

    <AppSettingsModal v-model:open="showSettings"/>
  </AppShell>
</template>

<script lang="ts" setup>
import AppShell from '~/components/layout/AppShell.vue'
import AppSettingsModal from '~/components/settings/AppSettingsModal.vue'
import ProjectCreateModal from '~/components/project/ProjectCreateModal.vue'
import ProjectList from '~/components/project/ProjectList.vue'
import type {CreateProjectInput} from '~/types/project'
import {toAppErrorMessage} from '~/utils/appError'

const router = useRouter()
const projectStore = useProjectStore()

const showCreate = ref(false)
const showSettings = ref(false)
const creating = ref(false)
const createError = ref<string | null>(null)
const loadError = ref<string | null>(null)

onMounted(refresh)

async function refresh() {
  loadError.value = null
  try {
    await projectStore.ping()
    await projectStore.checkDatabase()
    await projectStore.loadAppInfo()
    await projectStore.loadProjects()
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

async function deleteProject(projectId: string) {
  const project = projectStore.projects.find(item => item.id === projectId)
  const confirmed = window.confirm(`确定删除项目“${project?.title ?? '未命名项目'}”吗？此操作会删除项目内的全部内容。`)
  if (!confirmed) return

  try {
    await projectStore.deleteProject(projectId)
  } catch (error) {
    loadError.value = toAppErrorMessage(error, '删除项目失败')
  }
}
</script>
