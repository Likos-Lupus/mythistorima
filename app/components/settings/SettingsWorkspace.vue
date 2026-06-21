<template>
  <section class="settings-workspace">
    <header class="workspace-panel-header">
      <div>
        <p class="workspace-eyebrow">Phase 1 Week 8</p>
        <h2>主题与设置</h2>
        <p>调整写作界面的视觉风格、编辑器体验、自动保存和语言。</p>
      </div>
      <button class="secondary-button" type="button" @click="reload">重新读取设置</button>
    </header>

    <ErrorBanner :message="errorMessage" title="设置保存失败" @dismiss="errorMessage = null"/>

    <div v-if="settingsStore.loading" class="empty-panel">正在加载设置…</div>

    <div v-else class="settings-workspace-grid">
      <ThemeSettings/>
      <EditorSettings/>
      <AutosaveSettings/>
      <LanguageSettings/>
    </div>
  </section>
</template>

<script lang="ts" setup>
import ThemeSettings from '~/components/settings/ThemeSettings.vue'
import EditorSettings from '~/components/settings/EditorSettings.vue'
import AutosaveSettings from '~/components/settings/AutosaveSettings.vue'
import LanguageSettings from '~/components/settings/LanguageSettings.vue'
import ErrorBanner from '~/components/common/ErrorBanner.vue'
import {toAppErrorMessage} from '~/utils/appError'

const settingsStore = useSettingsStore()
const errorMessage = ref<string | null>(null)

async function reload() {
  errorMessage.value = null
  try {
    await settingsStore.loadAppSettings()
  } catch (error) {
    errorMessage.value = toAppErrorMessage(error, '读取设置失败')
  }
}
</script>
