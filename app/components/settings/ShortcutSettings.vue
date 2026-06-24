<template>
  <section class="settings-card shortcut-settings">
    <header class="settings-card-header">
      <div>
        <p class="workspace-eyebrow">Application shortcuts</p>
        <h3>快捷键</h3>
        <p>快捷键只在 Mythistorima 窗口内生效。创建类快捷键在正文输入时自动避让。</p>
      </div>
      <button :disabled="commandStore.saving" class="secondary-button" type="button" @click="resetAll">
        恢复默认
      </button>
    </header>

    <ErrorBanner :message="errorMessage" title="快捷键保存失败" @dismiss="errorMessage = null"/>

    <div v-if="commandStore.conflicts.length" class="shortcut-conflict-banner">
      <strong>检测到快捷键冲突</strong>
      <span v-for="conflict in commandStore.conflicts" :key="conflict.shortcut">
        {{ formatShortcut(conflict.shortcut) }}：{{ conflict.commandIds.map(commandTitle).join('、') }}
      </span>
    </div>

    <div class="shortcut-setting-list">
      <article v-for="command in commands" :key="command.id" class="shortcut-setting-row">
        <div class="shortcut-setting-copy">
          <strong>{{ command.title }}</strong>
          <small>{{ command.description }}</small>
        </div>

        <div class="shortcut-setting-controls">
          <CommandShortcutHint :shortcut="commandStore.shortcutFor(command.id)"/>
          <button
              :class="{ 'is-recording': recordingId === command.id }"
              class="secondary-button compact-button shortcut-record-button"
              type="button"
              @click="startRecording(command.id)"
          >
            {{ recordingId === command.id ? '请按组合键…' : '录制' }}
          </button>
          <button class="text-button" type="button" @click="clear(command.id)">清除</button>
          <button class="text-button" type="button" @click="reset(command.id)">默认</button>
        </div>
      </article>
    </div>

    <p class="shortcut-setting-help">
      录制时按 Esc 取消；按 Backspace 或 Delete 清除当前快捷键。冲突组合不会保存。
    </p>
  </section>
</template>

<script lang="ts" setup>
import CommandShortcutHint from '~/components/command/CommandShortcutHint.vue'
import ErrorBanner from '~/components/common/ErrorBanner.vue'
import {appCommandRegistry, commandDefinitionMap} from '~/constants/commandRegistry'
import type {AppCommandId} from '~/types/command'
import {formatShortcut, shortcutFromKeyboardEvent} from '~/utils/shortcut'
import {toAppErrorMessage} from '~/utils/appError'
import {useCommandStore} from '~/stores/command.store'

const commandStore = useCommandStore()
const commands = appCommandRegistry
const recordingId = ref<AppCommandId | null>(null)
const errorMessage = ref<string | null>(null)

onMounted(() => {
  if (!commandStore.loaded) commandStore.loadShortcuts()
  window.addEventListener('keydown', captureShortcut, {capture: true})
})

onBeforeUnmount(() => {
  commandStore.endShortcutRecording()
  window.removeEventListener('keydown', captureShortcut, {capture: true})
})

function startRecording(commandId: AppCommandId) {
  errorMessage.value = null
  recordingId.value = recordingId.value === commandId ? null : commandId
  if (recordingId.value) commandStore.beginShortcutRecording(recordingId.value)
  else commandStore.endShortcutRecording()
}

async function captureShortcut(event: KeyboardEvent) {
  if (!recordingId.value) return
  event.preventDefault()
  event.stopPropagation()

  if (event.key === 'Escape') {
    recordingId.value = null
    commandStore.endShortcutRecording()
    return
  }
  if (event.key === 'Backspace' || event.key === 'Delete') {
    await clear(recordingId.value)
    recordingId.value = null
    commandStore.endShortcutRecording()
    return
  }

  const shortcut = shortcutFromKeyboardEvent(event)
  if (!shortcut) return

  const conflict = commandStore.findConflict(recordingId.value, shortcut)
  if (conflict) {
    errorMessage.value = `“${formatShortcut(shortcut)}”已被“${commandTitle(conflict)}”使用。`
    return
  }

  const commandId = recordingId.value
  recordingId.value = null
  commandStore.endShortcutRecording()
  try {
    await commandStore.setShortcut(commandId, shortcut)
  } catch (error) {
    errorMessage.value = toAppErrorMessage(error, '保存快捷键失败')
  }
}

async function clear(commandId: AppCommandId) {
  errorMessage.value = null
  try {
    await commandStore.clearShortcut(commandId)
  } catch (error) {
    errorMessage.value = toAppErrorMessage(error, '清除快捷键失败')
  }
}

async function reset(commandId: AppCommandId) {
  errorMessage.value = null
  try {
    await commandStore.resetShortcut(commandId)
  } catch (error) {
    errorMessage.value = toAppErrorMessage(error, '恢复默认快捷键失败')
  }
}

async function resetAll() {
  if (!window.confirm('确定恢复全部默认快捷键吗？')) return
  errorMessage.value = null
  try {
    await commandStore.resetAllShortcuts()
  } catch (error) {
    errorMessage.value = toAppErrorMessage(error, '恢复默认快捷键失败')
  }
}

function commandTitle(commandId: AppCommandId) {
  return commandDefinitionMap.get(commandId)?.title ?? commandId
}
</script>
