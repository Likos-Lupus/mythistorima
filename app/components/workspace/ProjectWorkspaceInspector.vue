<template>
  <aside class="project-workspace-inspector">
    <WorkspaceContextPanel
        v-if="controller.workspaceMode.value !== 'writing'"
        :active-document-title="controller.activeDocument.value?.title"
        :project-character-count="controller.projectStats.value?.characterCount ?? 0"
        :project-id="controller.projectId.value"
        :workspace="controller.workspaceMode.value"
        @open-target="controller.openTarget"
    />

    <template v-else>
      <header class="project-inspector-header">
        <div>
          <p class="project-inspector-eyebrow">Inspector</p>
          <h2>写作信息</h2>
        </div>
        <UTooltip text="专注模式">
          <UButton
              aria-label="切换专注模式"
              color="neutral"
              icon="i-lucide-focus"
              size="sm"
              variant="ghost"
              @click="controller.toggleFocusMode"
          />
        </UTooltip>
      </header>

      <section class="project-inspector-section">
        <dl class="project-inspector-facts">
          <div>
            <dt>文档</dt>
            <dd>{{ activeDocumentLabel }}</dd>
          </div>
          <div>
            <dt>状态</dt>
            <dd>{{ activeDocumentStatus }}</dd>
          </div>
          <div>
            <dt>字数</dt>
            <dd>{{ controller.editorSnapshot.characterCount }}</dd>
          </div>
          <div>
            <dt>今日</dt>
            <dd>+{{ controller.timerStore.todayCharacterCount }}</dd>
          </div>
        </dl>

        <UFormField label="目标字数">
          <UInputNumber
              v-model="controller.targetDraft.value"
              :min="0"
              placeholder="未设置"
              size="sm"
              @blur="controller.saveDocumentTarget"
          />
        </UFormField>

        <UProgress
            v-if="controller.activeDocumentTarget.value"
            :max="controller.activeDocumentTarget.value"
            :model-value="controller.editorSnapshot.characterCount"
            size="sm"
        />
      </section>

      <section class="project-inspector-section">
        <div class="project-inspector-section-header">
          <h3>本章事项</h3>
          <UDropdownMenu :items="noteCreateItems">
            <UButton
                color="neutral"
                icon="i-lucide-plus"
                label="添加"
                size="xs"
                variant="ghost"
            />
          </UDropdownMenu>
        </div>

        <UEmpty
            v-if="controller.currentDocumentNotes.value.length === 0"
            class="project-inspector-empty"
            description="在正文或事项视图中添加待办、备忘和伏笔。"
            icon="i-lucide-list-checks"
            title="暂无未完成事项"
        />

        <div v-else class="project-inspector-notes">
          <article
              v-for="note in controller.currentDocumentNotes.value"
              :key="note.id"
              class="project-inspector-note"
          >
            <div>
              <strong>{{ note.title }}</strong>
              <small>
                {{ controller.noteTypeLabel(note.type) }} · {{ controller.notePriorityLabel(note.priority) }}
              </small>
            </div>
            <UTooltip text="标记完成">
              <UButton
                  aria-label="标记完成"
                  color="success"
                  icon="i-lucide-check"
                  size="xs"
                  variant="ghost"
                  @click="controller.markNoteDone(note.id)"
              />
            </UTooltip>
          </article>
        </div>
      </section>

      <section class="project-inspector-section">
        <div class="project-inspector-section-header">
          <h3>编辑器</h3>
          <UButton
              color="neutral"
              icon="i-lucide-settings-2"
              label="全部设置"
              size="xs"
              variant="ghost"
              @click="controller.shellStore.openSettings"
          />
        </div>

        <div class="project-inspector-settings">
          <UFormField label="字号">
            <UInputNumber
                :max="28"
                :min="12"
                :model-value="controller.settingsStore.editorSettings.fontSize"
                size="sm"
                @update:model-value="updateNumberSetting('fontSize', $event)"
            />
          </UFormField>

          <UFormField label="行距">
            <UInputNumber
                :max="2.8"
                :min="1.3"
                :model-value="controller.settingsStore.editorSettings.lineHeight"
                :step="0.05"
                size="sm"
                @update:model-value="updateNumberSetting('lineHeight', $event)"
            />
          </UFormField>

          <UFormField label="页面宽度">
            <UInputNumber
                :max="1100"
                :min="560"
                :model-value="controller.settingsStore.editorSettings.pageWidth"
                :step="10"
                size="sm"
                @update:model-value="updateNumberSetting('pageWidth', $event)"
            />
          </UFormField>

          <UFormField label="字体">
            <USelect
                :items="controller.settingsStore.fontFamilyOptions"
                :model-value="controller.settingsStore.editorSettings.fontFamily"
                label-key="label"
                size="sm"
                value-key="value"
                @update:model-value="updateFontFamily"
            />
          </UFormField>
        </div>
      </section>

      <UAlert
          v-if="controller.editorSnapshot.errorMessage"
          :description="controller.editorSnapshot.errorMessage"
          color="error"
          icon="i-lucide-circle-alert"
          title="编辑器错误"
          variant="subtle"
      />
    </template>
  </aside>
</template>

<script lang="ts" setup>
import WorkspaceContextPanel from '~/components/layout/WorkspaceContextPanel.vue'
import type {EditorSettings} from '~/types/editor'

const controller = useProjectWorkspaceContext()

const activeDocumentLabel = computed(() => {
  const document = controller.activeDocument.value
  return document
      ? `${controller.documentTypeLabel(document.type)} · ${document.title}`
      : '未选择'
})

const activeDocumentStatus = computed(() => {
  const document = controller.activeDocument.value
  return document ? controller.documentStatusLabel(document.status) : '未选择'
})

const noteCreateItems = computed(() => [[
  {
    label: '待办',
    icon: 'i-lucide-square-check-big',
    onSelect: () => controller.createChapterNote('todo')
  },
  {
    label: '备忘',
    icon: 'i-lucide-sticky-note',
    onSelect: () => controller.createChapterNote('memo')
  },
  {
    label: '伏笔',
    icon: 'i-lucide-sparkles',
    onSelect: () => controller.createChapterNote('foreshadow')
  }
]])

async function updateNumberSetting(
    key: 'fontSize' | 'lineHeight' | 'pageWidth',
    value: number | null | undefined
) {
  if (typeof value !== 'number' || !Number.isFinite(value)) return
  await controller.settingsStore.updateEditorSetting(key, value)
}

async function updateFontFamily(value: unknown) {
  if (typeof value !== 'string') return
  await controller.settingsStore.updateEditorSetting(
      'fontFamily',
      value as EditorSettings['fontFamily']
  )
}
</script>
