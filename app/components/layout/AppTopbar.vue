<template>
  <header class="topbar">
    <div class="topbar-inner">
      <UButton
          :aria-label="title"
          color="neutral"
          size="sm"
          variant="ghost"
          @click="$emit('home')"
      >
        <span class="topbar-title-row">
          <strong class="topbar-title">{{ title }}</strong>
          <span v-if="subtitle" class="topbar-subtitle">{{ subtitle }}</span>
        </span>
      </UButton>

      <div class="flex items-center gap-2">
        <UBadge v-if="status" color="neutral" size="sm" variant="subtle">
          {{ status }}
        </UBadge>

        <UTooltip v-if="showCommand" text="命令面板">
          <UButton
              aria-label="打开命令面板"
              color="neutral"
              icon="i-lucide-command"
              size="sm"
              variant="ghost"
              @click="$emit('command')"
          >
            <template v-if="commandShortcut" #trailing>
              <UKbd size="sm">{{ commandShortcut }}</UKbd>
            </template>
          </UButton>
        </UTooltip>

        <UTooltip v-if="showSettings" text="设置">
          <UButton
              aria-label="打开设置"
              color="neutral"
              icon="i-lucide-settings"
              size="sm"
              variant="ghost"
              @click="$emit('settings')"
          />
        </UTooltip>
      </div>
    </div>
  </header>
</template>

<script lang="ts" setup>
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
