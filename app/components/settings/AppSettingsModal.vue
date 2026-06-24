<template>
  <UModal
      v-model:open="model"
      :ui="{ content: 'max-w-6xl', body: 'p-0' }"
      description="外观、编辑器、自动保存、语言和快捷键"
      title="设置"
  >
    <template #body>
      <SettingsWorkspace/>
    </template>
  </UModal>
</template>

<script lang="ts" setup>
import SettingsWorkspace from '~/components/settings/SettingsWorkspace.vue'

const model = defineModel<boolean>('open', {default: false})
</script>
