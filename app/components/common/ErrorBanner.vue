<template>
  <div v-if="message" class="error-banner" role="alert">
    <strong>{{ title }}</strong>
    <span>{{ message }}</span>
    <button v-if="dismissible" type="button" @click="$emit('dismiss')">关闭</button>
  </div>
</template>

<script lang="ts" setup>
withDefaults(defineProps<{
  message?: string | null
  title?: string
  dismissible?: boolean
}>(), {
  title: '提示',
  dismissible: true
})

defineEmits<{
  dismiss: []
}>()
</script>
