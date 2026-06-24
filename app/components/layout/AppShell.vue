<template>
  <div class="app-shell">
    <AppTopbar
        :command-shortcut="commandShortcut"
        :show-command="showCommand"
        :show-settings="showSettings"
        :status="status"
        :subtitle="subtitle"
        :title="title"
        @command="$emit('command')"
        @home="$emit('home')"
        @settings="$emit('settings')"
    />

    <main class="app-main">
      <slot/>
    </main>
  </div>
</template>

<script lang="ts" setup>
import AppTopbar from '~/components/layout/AppTopbar.vue'

withDefaults(defineProps<{
  title: string
  subtitle?: string
  status?: string
  showCommand?: boolean
  showSettings?: boolean
  commandShortcut?: string
}>(), {
  subtitle: '',
  status: '',
  showCommand: false,
  showSettings: true,
  commandShortcut: ''
})

defineEmits<{
  home: []
  command: []
  settings: []
}>()
</script>
