<template>
  <section class="phase2-placeholder glass-panel">
    <header class="phase2-placeholder-header">
      <div>
        <p class="eyebrow">Phase 2 · Week {{ week }}</p>
        <h2>{{ title }}</h2>
        <p>{{ description }}</p>
      </div>
      <span class="phase2-status-pill">{{ statusLabel }}</span>
    </header>

    <div class="phase2-placeholder-body">
      <article class="phase2-placeholder-card">
        <h3>本周骨架已就绪</h3>
        <p>该工作区入口、导航分组、样式和数据库基础已接入。后续周次会在这里逐步实现完整交互。</p>
      </article>

      <article class="phase2-placeholder-card">
        <h3>后续实现重点</h3>
        <ul>
          <li v-for="item in items" :key="item">{{ item }}</li>
        </ul>
      </article>
    </div>
  </section>
</template>

<script lang="ts" setup>
const props = withDefaults(defineProps<{
  title: string
  description: string
  week: number
  items: string[]
  status?: 'foundation' | 'planned' | 'active'
}>(), {
  status: 'foundation'
})

const statusLabel = computed(() => {
  switch (props.status) {
    case 'active':
      return '可用'
    case 'planned':
      return '计划中'
    default:
      return '基础已建立'
  }
})
</script>
