<template>
  <UModal
      v-model:open="model"
      :ui="{
        content: 'max-w-6xl max-h-[calc(100dvh-2rem)]',
        body: 'min-h-0 overflow-y-auto p-0'
      }"
      description="外观、编辑器、自动保存、语言和快捷键"
      scrollable
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
