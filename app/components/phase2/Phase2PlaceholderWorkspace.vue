<template>
  <section class="workspace-placeholder glass-panel">
    <header class="workspace-placeholder-header">
      <div>
        <p class="eyebrow">工作区</p>
        <h2>{{ title }}</h2>
        <p>{{ description }}</p>
      </div>
    </header>

    <UEmpty
        description="该视图正在整理界面结构，现有数据和功能不会受到影响。"
        icon="i-lucide-panels-top-left"
        title="界面整理中"
    />
  </section>
</template>

<script lang="ts" setup>
defineProps<{
  title: string
  description: string
}>()
</script>
