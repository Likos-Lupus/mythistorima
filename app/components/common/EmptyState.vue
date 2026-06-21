<template>
  <section class="empty-state">
    <div aria-hidden="true" class="empty-state-icon">{{ icon }}</div>
    <h3>{{ title }}</h3>
    <p>{{ description }}</p>
    <slot/>
  </section>
</template>

<script lang="ts" setup>
withDefaults(defineProps<{
  title: string
  description: string
  icon?: string
}>(), {
  icon: '✦'
})
</script>
