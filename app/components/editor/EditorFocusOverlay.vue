<template>
  <div class="editor-focus-overlay" role="status">
    <span>专注模式</span>
    <button class="tree-action-button" type="button" @click="$emit('exit')">退出</button>
  </div>
</template>

<script lang="ts" setup>
defineEmits<{
  exit: []
}>()
</script>
