<template>
  <section class="settings-section">
    <div class="settings-section-header">
      <h3>自动保存</h3>
      <p>设置停止输入多久后保存正文。范围 0.5 秒到 10 秒。</p>
    </div>

    <label class="settings-field">
      <span>自动保存间隔：{{ (settingsStore.editorSettings.autosaveIntervalMs / 1000).toFixed(1) }} 秒</span>
      <input
          :value="settingsStore.editorSettings.autosaveIntervalMs"
          class="range-field"
          max="10000"
          min="500"
          step="250"
          type="range"
          @input="handleAutosaveChange"
      >
    </label>
  </section>
</template>

<script lang="ts" setup>
const settingsStore = useSettingsStore()

function handleAutosaveChange(event: Event) {
  const value = Number((event.target as HTMLInputElement).value)
  if (Number.isFinite(value)) void settingsStore.updateEditorSetting('autosaveIntervalMs', value)
}
</script>
