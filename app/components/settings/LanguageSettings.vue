<template>
  <section class="settings-section">
    <div class="settings-section-header">
      <h3>语言</h3>
      <p>Week 8 先完成中文 / 英文文案结构和语言设置持久化。</p>
    </div>

    <div class="language-option-list">
      <button
          v-for="option in settingsStore.languageOptions"
          :key="option.value"
          :class="['language-option-card', { 'is-active': settingsStore.language === option.value }]"
          type="button"
          @click="setLanguage(option.value)"
      >
        <strong>{{ option.label }}</strong>
        <span>{{ option.description }}</span>
      </button>
    </div>
  </section>
</template>

<script lang="ts" setup>
import type {AppLanguage} from '~/types/settings'

const settingsStore = useSettingsStore()
const {locale} = useI18n()

async function setLanguage(language: AppLanguage) {
  locale.value = language
  await settingsStore.setLanguage(language)
}
</script>
