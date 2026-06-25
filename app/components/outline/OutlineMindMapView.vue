<template>
  <section :style="{'--mind-zoom': zoom}" class="outline-mind-map">
    <svg :viewBox="viewBox" aria-label="大纲图谱" role="img">
      <defs>
        <marker id="outline-mind-arrow" markerHeight="8" markerWidth="8" orient="auto" refX="6" refY="4">
          <path d="M0,0 L8,4 L0,8 z"/>
        </marker>
      </defs>
      <g :transform="`scale(${zoom})`">
        <line
            v-for="edge in edges"
            :key="edge.key"
            :x1="edge.source.x"
            :x2="edge.target.x"
            :y1="edge.source.y"
            :y2="edge.target.y"
            marker-end="url(#outline-mind-arrow)"
        />
        <g
            v-for="node in nodes"
            :key="node.item.id"
            :class="['outline-mind-node', `is-${node.item.status}`, {'is-active': node.item.id === activeOutlineNodeId}]"
            :transform="`translate(${node.x}, ${node.y})`"
            @click="$emit('select', node.item.id)"
        >
          <rect height="72" rx="10" width="190" x="-95" y="-36"/>
          <text class="outline-mind-node-type" text-anchor="middle" x="0" y="-12">{{ outlineTypeLabel(node.item.type) }}
            · {{ outlineStatusLabel(node.item.status) }}
          </text>
          <text class="outline-mind-node-title" text-anchor="middle" x="0" y="10">{{
              truncate(node.item.title, 16)
            }}
          </text>
          <text v-if="node.item.linkedDocument" class="outline-mind-node-doc" text-anchor="middle" x="0" y="29">
            {{ truncate(node.item.linkedDocument.title, 18) }}
          </text>
        </g>
      </g>
    </svg>
    <UEmpty
        v-if="!items.length"
        class="outline-mind-empty"
        description="创建大纲节点后，图谱会按父子结构自动布局。"
        icon="i-lucide-git-fork"
        title="暂无图谱节点"
    />
  </section>
</template>

<script lang="ts" setup>
import type {NovelDocument} from '~/types/document'
import type {OutlineTreeNode} from '~/types/outline'
import {outlineStatusLabel, outlineTypeLabel} from '~/utils/outlinePresentation'

const props = withDefaults(defineProps<{
  items: OutlineTreeNode[]
  documents: NovelDocument[]
  activeOutlineNodeId?: string | null
  zoom?: number
}>(), {
  zoom: 1
})

defineEmits<{
  select: [outlineNodeId: string]
  openDocument: [documentId: string]
}>()

const viewBox = computed(() => `0 0 ${Math.max(980, columns.value * 260 + 160)} ${Math.max(620, rows.value * 118 + 160)}`)
const rows = computed(() => Math.max(1, ...props.items.map(item => item.depth + 1)))
const columns = computed(() => Math.max(1, Math.ceil(props.items.length / 3)))
const siblingIndex = computed(() => {
  const counts = new Map<number, number>()
  const result = new Map<string, number>()
  for (const item of props.items) {
    const current = counts.get(item.depth) ?? 0
    result.set(item.id, current)
    counts.set(item.depth, current + 1)
  }
  return result
})
const nodes = computed(() => props.items.map(item => {
  const index = siblingIndex.value.get(item.id) ?? 0
  return {
    item,
    x: 140 + item.depth * 250,
    y: 90 + index * 115
  }
}))
const nodesById = computed(() => new Map(nodes.value.map(node => [node.item.id, node])))
const edges = computed(() => nodes.value
    .map(node => {
      const parent = node.item.parentId ? nodesById.value.get(node.item.parentId) : null
      if (!parent) return null
      return {key: `${parent.item.id}-${node.item.id}`, source: parent, target: node}
    })
    .filter(Boolean) as Array<{ key: string; source: typeof nodes.value[number]; target: typeof nodes.value[number] }>)

function truncate(value: string, max: number) {
  return value.length > max ? `${value.slice(0, max)}…` : value
}
</script>
