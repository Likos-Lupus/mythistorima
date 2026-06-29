<template>
  <section class="settings-section">
    <div class="settings-section-header">
      <h3>主题</h3>
      <p>Light、Paper、Dark 三套主题使用同一套 Nuxt UI 语义色，便于发布前检查一致性。</p>
    </div>

    <div class="theme-option-grid">
      <UButton
          v-for="option in settingsStore.themeOptions"
          :key="option.value"
          :aria-pressed="settingsStore.theme === option.value"
          :class="['theme-option-card', `theme-preview-${option.value}`, { 'is-active': settingsStore.theme === option.value }]"
          :color="settingsStore.theme === option.value ? 'primary' : 'neutral'"
          :variant="settingsStore.theme === option.value ? 'soft' : 'ghost'"
          block
          size="sm"
          @click="settingsStore.setTheme(option.value)"
      >
        <span class="theme-option-content">
          <span>
            <strong>{{ option.label }}</strong>
            <small>{{ option.description }}</small>
          </span>
          <UBadge v-if="settingsStore.theme === option.value" color="primary" size="sm" variant="soft">当前</UBadge>
        </span>
      </UButton>
    </div>
  </section>
</template>

<script lang="ts" setup>
const settingsStore = useSettingsStore()
</script>
