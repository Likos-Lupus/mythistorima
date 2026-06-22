<template>
  <Phase2PlaceholderWorkspace
      :items="['时间线事件 CRUD', '故事内时间标签和 sort_key', '绑定角色 / 地点 / 道具', '按人物和地点过滤']"
      :week="5"
      description="管理故事内事件顺序、参与角色、地点和关联章节。"
      title="时间线"
  />
</template>
<script lang="ts" setup>
import Phase2PlaceholderWorkspace from '~/components/phase2/Phase2PlaceholderWorkspace.vue'
</script>
