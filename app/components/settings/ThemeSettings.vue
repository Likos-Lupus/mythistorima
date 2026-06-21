<template>
  <section class="settings-section">
    <div class="settings-section-header">
      <h3>主题</h3>
      <p>切换适合不同写作场景的视觉风格。</p>
    </div>

    <div class="theme-option-grid">
      <button
          v-for="option in settingsStore.themeOptions"
          :key="option.value"
          :class="['theme-option-card', `theme-preview-${option.value}`, { 'is-active': settingsStore.theme === option.value }]"
          type="button"
          @click="settingsStore.setTheme(option.value)"
      >
        <span class="theme-option-title">{{ option.label }}</span>
        <span class="theme-option-description">{{ option.description }}</span>
      </button>
    </div>
  </section>
</template>

<script lang="ts" setup>
const settingsStore = useSettingsStore()
</script>
