<template>
  <section class="relation-graph-panel paper-card">
    <header class="relation-panel-header">
      <div>
        <p class="relation-panel-kicker">World Graph</p>
        <h2>关系图</h2>
      </div>
      <p class="relation-graph-help">点击节点打开设定卡，点击连线编辑关系。</p>
    </header>

    <div v-if="cards.length === 0" class="relation-graph-empty">
      还没有设定卡。请先创建人物、地点、组织、道具、事件或概念。
    </div>

    <template v-else>
      <p v-if="isTruncated" class="relation-graph-performance-note">
        大项目模式：关系图仅渲染前 {{ GRAPH_NODE_LIMIT }} 个筛选节点。可使用节点类型筛选缩小范围，完整关系列表仍保留。
      </p>

      <svg
          aria-label="设定卡关系图"
          class="relation-graph-canvas"
          role="img"
          viewBox="0 0 960 620"
      >
        <defs>
          <marker id="relation-arrow" markerHeight="8" markerWidth="8" orient="auto" refX="7" refY="4">
            <path d="M 0 0 L 8 4 L 0 8 z"/>
          </marker>
        </defs>

        <g class="relation-edges">
          <g
              v-for="edge in visibleEdges"
              :key="edge.relation.id"
              :class="['relation-edge', { 'is-active': edge.relation.id === activeRelationId }]"
              @click.stop="$emit('select-relation', edge.relation.id)"
          >
            <line
                :marker-end="edge.relation.direction === 'undirected' ? undefined : 'url(#relation-arrow)'"
                :marker-start="edge.relation.direction === 'bidirectional' ? 'url(#relation-arrow)' : undefined"
                :x1="edge.source.x"
                :x2="edge.target.x"
                :y1="edge.source.y"
                :y2="edge.target.y"
            />
            <rect
                :height="24"
                :rx="12"
                :width="edge.labelWidth"
                :x="edge.labelX - edge.labelWidth / 2"
                :y="edge.labelY - 14"
            />
            <text :x="edge.labelX" :y="edge.labelY + 4" text-anchor="middle">
              {{ relationTypeLabel(edge.relation.relationType) }}
            </text>
          </g>
        </g>

        <g class="relation-nodes">
          <g
              v-for="node in visibleNodes"
              :key="node.card.id"
              :class="['relation-node', `is-${node.card.type}`, { 'is-selected': node.card.id === selectedCardId }]"
              :transform="`translate(${node.x}, ${node.y})`"
              @click.stop="$emit('open-card', node.card.id)"
          >
            <rect height="64" rx="18" width="154" x="-77" y="-32"/>
            <text class="relation-node-type" text-anchor="middle" x="0" y="-8">{{
                cardTypeLabel(node.card.type)
              }}
            </text>
            <text class="relation-node-name" text-anchor="middle" x="0" y="15">{{ truncate(node.card.name, 12) }}</text>
          </g>
        </g>
      </svg>
    </template>
  </section>
</template>

<script lang="ts" setup>
import {cardTypeLabel, type SettingCard} from '~/types/card'
import {type CardGraphCardTypeFilter, type CardRelation, relationTypeLabel} from '~/types/relation'

const props = defineProps<{
  cards: SettingCard[]
  relations: CardRelation[]
  cardTypeFilter: CardGraphCardTypeFilter
  relationTypeFilter: string
  activeRelationId?: string | null
  selectedCardId?: string | null
}>()

defineEmits<{
  'select-relation': [relationId: string]
  'open-card': [cardId: string]
}>()

interface PositionedNode {
  card: SettingCard
  x: number
  y: number
}

const GRAPH_NODE_LIMIT = 160

const filteredCards = computed(() => {
  return props.cards.filter(card => props.cardTypeFilter === 'all' || card.type === props.cardTypeFilter)
})

const isTruncated = computed(() => filteredCards.value.length > GRAPH_NODE_LIMIT)

const visibleCards = computed(() => {
  if (filteredCards.value.length <= GRAPH_NODE_LIMIT) return filteredCards.value
  const selected = props.selectedCardId
      ? filteredCards.value.find(card => card.id === props.selectedCardId)
      : null
  const limited = filteredCards.value.slice(0, GRAPH_NODE_LIMIT)
  if (selected && !limited.some(card => card.id === selected.id)) {
    limited[limited.length - 1] = selected
  }
  return limited
})

const visibleNodes = computed<PositionedNode[]>(() => {
  const count = visibleCards.value.length
  if (count === 1) {
    return [{card: visibleCards.value[0], x: 480, y: 310}]
  }
  const radiusX = count <= 6 ? 270 : 350
  const radiusY = count <= 6 ? 180 : 235
  return visibleCards.value.map((card, index) => {
    const angle = -Math.PI / 2 + (index / count) * Math.PI * 2
    return {
      card,
      x: Math.round(480 + Math.cos(angle) * radiusX),
      y: Math.round(310 + Math.sin(angle) * radiusY)
    }
  })
})

const nodeById = computed(() => new Map(visibleNodes.value.map(node => [node.card.id, node])))

const visibleEdges = computed(() => {
  return props.relations
      .filter(relation => props.relationTypeFilter === 'all' || relation.relationType === props.relationTypeFilter)
      .map(relation => {
        const source = nodeById.value.get(relation.sourceCardId)
        const target = nodeById.value.get(relation.targetCardId)
        if (!source || !target) return null
        const label = relationTypeLabel(relation.relationType)
        return {
          relation,
          source,
          target,
          labelX: Math.round((source.x + target.x) / 2),
          labelY: Math.round((source.y + target.y) / 2),
          labelWidth: Math.max(58, label.length * 18)
        }
      })
      .filter(Boolean) as Array<{
    relation: CardRelation
    source: PositionedNode
    target: PositionedNode
    labelX: number
    labelY: number
    labelWidth: number
  }>
})

function truncate(value: string, max: number) {
  return value.length > max ? `${value.slice(0, max)}…` : value
}
</script>
