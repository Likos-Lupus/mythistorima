<template>
  <section class="outline-mermaid-view">
    <header class="outline-mermaid-header">
      <div>
        <p class="card-editor-kicker">Mermaid Flowchart</p>
        <h2>剧情流程图</h2>
        <p>从当前大纲自动生成 Mermaid flowchart，可复制到文档、Issue 或支持 Mermaid 的编辑器中。</p>
      </div>
      <div class="outline-mermaid-actions">
        <button class="secondary-button" type="button" @click="copyMermaidText">
          {{ copied ? '已复制' : '复制 Mermaid 文本' }}
        </button>
      </div>
    </header>

    <div class="outline-mermaid-grid">
      <div aria-label="Mermaid 剧情流程预览" class="outline-mermaid-preview">
        <div class="outline-flow-root">故事大纲</div>
        <div v-if="previewItems.length" class="outline-flow-lanes">
          <article
              v-for="item in previewItems"
              :key="item.id"
              :class="['outline-flow-node', `is-${item.status}`]"
              :style="{ '--outline-depth': Math.min(item.depth, 4) }"
          >
            <span>{{ outlineTypeLabel(item.type) }}</span>
            <strong>{{ item.title }}</strong>
            <small>
              {{ outlineStatusLabel(item.status) }}
              <template v-if="item.linkedDocument"> · {{
                  documentTypeLabel(item.linkedDocument.type)
                }}《{{ item.linkedDocument.title }}》
              </template>
            </small>
          </article>
        </div>
        <div v-else class="outline-board-empty">暂无可生成流程图的大纲节点。</div>
      </div>

      <pre class="outline-mermaid-code"><code>{{ mermaidText }}</code></pre>
    </div>
  </section>
</template>

<script lang="ts" setup>
import type {OutlineTreeNode} from '~/types/outline'
import {
  documentTypeLabel,
  flattenOutlineTree,
  generateOutlineMermaid,
  outlineStatusLabel,
  outlineTypeLabel
} from '~/utils/outlinePresentation'
import type {NovelDocument} from '~/types/document'

const props = defineProps<{
  items: OutlineTreeNode[]
  documents: NovelDocument[]
}>()

const copied = ref(false)
const previewItems = computed(() => flattenOutlineTree(props.items).filter(item => item.status !== 'archived'))
const mermaidText = computed(() => generateOutlineMermaid(previewItems.value, props.documents))
let copyTimer: ReturnType<typeof setTimeout> | null = null

async function copyMermaidText() {
  copied.value = false
  try {
    await navigator.clipboard.writeText(mermaidText.value)
    copied.value = true
  } catch {
    fallbackCopy(mermaidText.value)
    copied.value = true
  }

  if (copyTimer) clearTimeout(copyTimer)
  copyTimer = setTimeout(() => {
    copied.value = false
  }, 1600)
}

function fallbackCopy(value: string) {
  const textarea = document.createElement('textarea')
  textarea.value = value
  textarea.setAttribute('readonly', 'true')
  textarea.style.position = 'fixed'
  textarea.style.opacity = '0'
  document.body.appendChild(textarea)
  textarea.select()
  document.execCommand('copy')
  document.body.removeChild(textarea)
}
</script>
