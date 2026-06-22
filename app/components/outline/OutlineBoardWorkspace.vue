<template>
  <Phase2PlaceholderWorkspace
      :items="['大纲状态看板', '节点状态切换', 'Mermaid flowchart 生成', '复制 Mermaid 文本']"
      :week="3"
      description="用 planned / drafting / done 看板管理剧情节点，并在 Week 3 生成 Mermaid 视图。"
      title="大纲看板"
  />
</template>
<script lang="ts" setup>
import Phase2PlaceholderWorkspace from '~/components/phase2/Phase2PlaceholderWorkspace.vue'
</script>
