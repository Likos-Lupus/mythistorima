<template>
  <div class="app-shell">
    <AppTopbar
        :status="status"
        :subtitle="subtitle"
        :title="title"
        @home="$emit('home')"
    />

    <main class="app-main">
      <slot/>
    </main>
  </div>
</template>

<script lang="ts" setup>
import AppTopbar from '~/components/layout/AppTopbar.vue'

defineProps<{
  title: string
  subtitle?: string
  status?: string
}>()

defineEmits<{
  home: []
}>()
</script>
