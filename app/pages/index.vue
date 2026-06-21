<template>
  <AppShell
      :status="databaseStatus"
      subtitle="为小说作者设计的本地写作工具"
      title="Mythistorima"
  >
    <div class="mx-auto grid max-w-295 gap-6 lg:grid-cols-[1fr_420px]">
      <section class="paper-card rounded-4xl p-8 md:p-10">
        <p class="mb-3 text-sm font-semibold uppercase tracking-[0.28em] text-[#7b4f2c]">Phase 0 · Local Writing
          Loop</p>
        <h2 class="max-w-3xl text-4xl font-bold leading-tight md:text-5xl">
          像纸一样安静，像数据库一样可靠。
        </h2>
        <p class="mt-5 max-w-2xl text-lg leading-8 text-[#5f5348]">
          当前版本验证桌面端本地写作闭环：创建项目、创建章节、富文本写作、SQLite 自动保存与重启恢复。
        </p>

        <div class="mt-8 flex flex-wrap gap-3">
          <button class="rounded-full bg-[#6d4325] px-6 py-3 font-semibold text-white shadow-xl shadow-[#6d4325]/20"
                  type="button" @click="showCreate = true">
            新建项目
          </button>
          <button
              class="rounded-full border border-(--line-soft) bg-white/45 px-6 py-3 font-semibold text-[#5f5348] hover:bg-white/70"
              type="button" @click="refresh">
            刷新状态
          </button>
        </div>

        <div class="mt-8 grid gap-3 md:grid-cols-3">
          <div class="rounded-3xl bg-[#7b4f2c]/10 p-4">
            <p class="text-xs font-semibold uppercase tracking-wider text-[#7b4f2c]">Rust</p>
            <p class="mt-2 text-sm text-[#5f5348]">{{ rustStatus }}</p>
          </div>
          <div class="rounded-3xl bg-[#7b4f2c]/10 p-4">
            <p class="text-xs font-semibold uppercase tracking-wider text-[#7b4f2c]">SQLite</p>
            <p class="mt-2 text-sm text-[#5f5348]">{{ databaseStatus }}</p>
          </div>
          <div class="rounded-3xl bg-[#7b4f2c]/10 p-4">
            <p class="text-xs font-semibold uppercase tracking-wider text-[#7b4f2c]">项目数</p>
            <p class="mt-2 text-sm text-[#5f5348]">{{ projectStore.projects.length }} 个</p>
          </div>
        </div>
      </section>

      <section class="glass-panel rounded-4xl p-5">
        <div class="mb-4 flex items-center justify-between gap-3">
          <div>
            <h2 class="text-xl font-bold">最近项目</h2>
            <p class="text-sm text-muted-paper">保存在本地 SQLite 中</p>
          </div>
          <span v-if="projectStore.loading" class="text-sm text-muted-paper">加载中…</span>
        </div>
        <ProjectList :projects="projectStore.projects" @open="openProject"/>
      </section>
    </div>

    <ProjectCreateModal
        :error="createError"
        :open="showCreate"
        :submitting="creating"
        @close="showCreate = false"
        @submit="createProject"
    />
  </AppShell>
</template>

<script lang="ts" setup>
import AppShell from '~/components/layout/AppShell.vue'
import ProjectCreateModal from '~/components/project/ProjectCreateModal.vue'
import ProjectList from '~/components/project/ProjectList.vue'
import type {CreateProjectInput} from '~/types/project'

const router = useRouter()
const projectStore = useProjectStore()

const showCreate = ref(false)
const creating = ref(false)
const createError = ref<string | null>(null)
const rustStatus = ref('待连接')

const databaseStatus = computed(() => projectStore.databaseReady ? '数据库正常' : '数据库未就绪')

onMounted(async () => {
  await refresh()
})

async function refresh() {
  try {
    rustStatus.value = await projectStore.ping()
    await projectStore.checkDatabase()
    await projectStore.loadAppInfo()
    await projectStore.loadProjects()
  } catch (error) {
    rustStatus.value = '连接失败：请在 Tauri 环境中启动'
    console.error(error)
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
    createError.value = typeof error === 'object' && error && 'message' in error
        ? String((error as { message?: string }).message)
        : '创建项目失败'
  } finally {
    creating.value = false
  }
}

async function openProject(projectId: string) {
  await router.push(`/project/${projectId}`)
}
</script>
