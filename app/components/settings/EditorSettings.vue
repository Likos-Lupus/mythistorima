<template>
  <section class="settings-section">
    <div class="settings-section-header">
      <h3>编辑器</h3>
      <p>调整纸张宽度、字体、字号和行距。</p>
    </div>

    <div class="settings-form-grid">
      <label class="settings-field">
        <span>字体</span>
        <select
            :value="settingsStore.editorSettings.fontFamily"
            class="form-field"
            @change="handleFontFamilyChange"
        >
          <option v-for="option in settingsStore.fontFamilyOptions" :key="option.value" :value="option.value">
            {{ option.label }}
          </option>
        </select>
      </label>

      <label class="settings-field">
        <span>字号：{{ settingsStore.editorSettings.fontSize }}px</span>
        <input
            :value="settingsStore.editorSettings.fontSize"
            class="range-field"
            max="28"
            min="12"
            type="range"
            @input="handleNumberChange('fontSize', $event)"
        >
      </label>

      <label class="settings-field">
        <span>行距：{{ settingsStore.editorSettings.lineHeight }}</span>
        <input
            :value="settingsStore.editorSettings.lineHeight"
            class="range-field"
            max="2.8"
            min="1.3"
            step="0.05"
            type="range"
            @input="handleNumberChange('lineHeight', $event)"
        >
      </label>

      <label class="settings-field">
        <span>页面宽度：{{ settingsStore.editorSettings.pageWidth }}px</span>
        <input
            :value="settingsStore.editorSettings.pageWidth"
            class="range-field"
            max="1100"
            min="560"
            step="10"
            type="range"
            @input="handleNumberChange('pageWidth', $event)"
        >
      </label>
    </div>
  </section>
</template>

<script lang="ts" setup>
import type {EditorSettings} from '~/types/editor'
import type {EditorFontFamily} from '~/types/settings'

const settingsStore = useSettingsStore()

function handleFontFamilyChange(event: Event) {
  const value = (event.target as HTMLSelectElement).value as EditorFontFamily
  void settingsStore.updateEditorSetting('fontFamily', value)
}

function handleNumberChange(key: keyof Pick<EditorSettings, 'fontSize' | 'lineHeight' | 'pageWidth'>, event: Event) {
  const value = Number((event.target as HTMLInputElement).value)
  if (Number.isFinite(value)) void settingsStore.updateEditorSetting(key, value)
}
</script>
