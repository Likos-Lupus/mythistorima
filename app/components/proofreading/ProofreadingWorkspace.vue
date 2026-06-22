<template>
  <Phase2PlaceholderWorkspace
      :items="['校对规则 CRUD', '当前章节检查', '全项目检查', '校对结果跳转段落']"
      :week="7"
      description="运行本地规则，检查重复词、标点、超长句、敏感词和名称一致性。"
      title="校对"
  />
</template>
<script lang="ts" setup>
import Phase2PlaceholderWorkspace from '~/components/phase2/Phase2PlaceholderWorkspace.vue'
</script>
