<template>
  <UApp>
    <NuxtLayout>
      <NuxtPage/>
    </NuxtLayout>
  </UApp>
</template>

<script lang="ts" setup>
const settingsStore = useSettingsStore()
const {locale} = useI18n()

onMounted(async () => {
  try {
    await settingsStore.loadAppSettings()
    locale.value = settingsStore.language
  } catch (error) {
    console.warn('[settings] failed to load app settings at startup', error)
    settingsStore.applyTheme()
    settingsStore.applyLanguage()
    settingsStore.applyEditorSettingsToDocument()
  }
})
</script>
