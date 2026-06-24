<template>
  <nav aria-label="项目视图" class="project-activity-bar">
    <UTooltip
        v-for="view in projectPrimaryViews"
        :key="view.id"
        :content="{ side: 'right' }"
        :delay-duration="250"
        :text="view.label"
    >
      <UButton
          :aria-label="view.label"
          :class="{ 'is-active': activeView === view.id }"
          :color="activeView === view.id ? 'primary' : 'neutral'"
          :icon="view.icon"
          :variant="activeView === view.id ? 'soft' : 'ghost'"
          size="md"
          square
          @click="$emit('select', view.id)"
      />
    </UTooltip>
  </nav>
</template>

<script lang="ts" setup>
import {type ProjectPrimaryView, projectPrimaryViews} from '~/constants/projectViews'

defineProps<{
  activeView: ProjectPrimaryView
}>()

defineEmits<{
  select: [view: ProjectPrimaryView]
}>()
</script>
