<template>
  <header class="topbar">
    <div class="topbar-inner">
      <button class="topbar-home" type="button" @click="$emit('home')">
        <div class="topbar-title-row">
          <h1 class="topbar-title">{{ title }}</h1>
          <span v-if="subtitle" class="topbar-subtitle">{{ subtitle }}</span>
        </div>
      </button>

      <div v-if="status" class="topbar-status">
        {{ status }}
      </div>
    </div>
  </header>
</template>

<script lang="ts" setup>
defineProps<{
  title: string
  subtitle?: string
  status?: string
}>()

defineEmits<{
  home: []
}>()
</script>
