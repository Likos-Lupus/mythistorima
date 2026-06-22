<template>
  <Phase2PlaceholderWorkspace
      :items="['伏笔线程 CRUD', '设置提出位置和回收位置', '未回收伏笔面板', '按优先级和状态筛选']"
      :week="6"
      description="把 Phase 1 的伏笔事项升级为可追踪的提出 / 回收线程。"
      title="伏笔"
  />
</template>
<script lang="ts" setup>
import Phase2PlaceholderWorkspace from '~/components/phase2/Phase2PlaceholderWorkspace.vue'
</script>
