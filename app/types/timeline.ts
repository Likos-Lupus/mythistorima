import type {SettingCard} from '~/types/card'

export type TimelineParticipantRole = 'participant' | 'observer' | 'owner' | 'affected' | string

export interface TimelineEvent {
    id: string
    projectId: string
    linkedCardId?: string | null
    linkedDocumentId?: string | null
    title: string
    description: string
    startsAtLabel?: string | null
    endsAtLabel?: string | null
    sortKey: number
    locationCardId?: string | null
    metadataJson: string
    createdAt: number
    updatedAt: number
}

export interface TimelineEventCard {
    id: string
    projectId: string
    timelineEventId: string
    cardId: string
    role: TimelineParticipantRole
}

export interface CreateTimelineEventInput {
    projectId: string
    linkedCardId?: string | null
    linkedDocumentId?: string | null
    title: string
    description?: string | null
    startsAtLabel?: string | null
    endsAtLabel?: string | null
    sortKey?: number | null
    locationCardId?: string | null
    metadataJson?: string | null
}

export interface UpdateTimelineEventInput {
    timelineEventId: string
    linkedCardId?: string | null
    linkedDocumentId?: string | null
    title?: string | null
    description?: string | null
    startsAtLabel?: string | null
    endsAtLabel?: string | null
    sortKey?: number | null
    locationCardId?: string | null
    metadataJson?: string | null
}

export interface AttachTimelineEventCardInput {
    projectId: string
    timelineEventId: string
    cardId: string
    role?: TimelineParticipantRole | null
}

export interface ReorderTimelineEventInput {
    timelineEventId: string
    sortKey: number
}

export interface TimelineEventWithCards extends TimelineEvent {
    cards: TimelineEventCard[]
    participants: SettingCard[]
}

export interface TimelineRoleOption {
    value: TimelineParticipantRole
    label: string
}

export const timelineRoleOptions: TimelineRoleOption[] = [
    {value: 'participant', label: '参与'},
    {value: 'observer', label: '见证'},
    {value: 'owner', label: '持有 / 关键归属'},
    {value: 'affected', label: '受影响'}
]

export function timelineRoleLabel(role: string) {
    return timelineRoleOptions.find(option => option.value === role)?.label ?? role
}

export function timelineDateLabel(event: Pick<TimelineEvent, 'startsAtLabel' | 'endsAtLabel'>) {
    if (event.startsAtLabel && event.endsAtLabel) return `${event.startsAtLabel} → ${event.endsAtLabel}`
    return event.startsAtLabel || event.endsAtLabel || '未设置显示时间'
}
