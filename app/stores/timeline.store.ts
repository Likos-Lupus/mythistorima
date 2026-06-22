import type {
    AttachTimelineEventCardInput,
    CreateTimelineEventInput,
    ReorderTimelineEventInput,
    TimelineEvent,
    TimelineEventCard,
    UpdateTimelineEventInput
} from '~/types/timeline'

function sortEvents(events: TimelineEvent[]) {
    return [...events].sort((a, b) => a.sortKey - b.sortKey || a.createdAt - b.createdAt || a.title.localeCompare(b.title))
}

export const useTimelineStore = defineStore('timeline', () => {
    const events = ref<TimelineEvent[]>([])
    const eventCards = ref<TimelineEventCard[]>([])
    const activeEventId = ref<string | null>(null)
    const cardFilterId = ref<string | null>(null)
    const locationFilterId = ref<string | null>(null)
    const loading = ref(false)
    const saving = ref(false)

    const {call} = useTauriInvoke()

    const activeEvent = computed(() => events.value.find(event => event.id === activeEventId.value) ?? null)
    const participantsByEventId = computed(() => {
        const result = new Map<string, TimelineEventCard[]>()
        for (const item of eventCards.value) {
            const list = result.get(item.timelineEventId) ?? []
            list.push(item)
            result.set(item.timelineEventId, list)
        }
        return result
    })
    const activeEventCards = computed(() => activeEventId.value ? participantsByEventId.value.get(activeEventId.value) ?? [] : [])

    async function loadTimeline(projectId: string) {
        loading.value = true
        try {
            const [timelineEvents, timelineCards] = await Promise.all([
                call<TimelineEvent[]>('list_timeline_events', {
                    projectId,
                    cardId: cardFilterId.value,
                    locationCardId: locationFilterId.value
                }),
                call<TimelineEventCard[]>('list_timeline_event_cards', {
                    projectId,
                    timelineEventId: null
                })
            ])
            events.value = sortEvents(timelineEvents)
            eventCards.value = timelineCards
            if (!activeEventId.value || !events.value.some(event => event.id === activeEventId.value)) {
                activeEventId.value = events.value[0]?.id ?? null
            }
            return events.value
        } finally {
            loading.value = false
        }
    }

    async function loadEventCards(projectId: string) {
        eventCards.value = await call<TimelineEventCard[]>('list_timeline_event_cards', {
            projectId,
            timelineEventId: null
        })
        return eventCards.value
    }

    async function createEvent(input: CreateTimelineEventInput) {
        saving.value = true
        try {
            const event = await call<TimelineEvent>('create_timeline_event', {input})
            events.value = sortEvents([event, ...events.value.filter(item => item.id !== event.id)])
            activeEventId.value = event.id
            return event
        } finally {
            saving.value = false
        }
    }

    async function updateEvent(input: UpdateTimelineEventInput) {
        saving.value = true
        try {
            const event = await call<TimelineEvent>('update_timeline_event', {input})
            const index = events.value.findIndex(item => item.id === event.id)
            if (index >= 0) events.value[index] = event
            else events.value.push(event)
            events.value = sortEvents(events.value)
            activeEventId.value = event.id
            return event
        } finally {
            saving.value = false
        }
    }

    async function deleteEvent(timelineEventId: string) {
        await call<boolean>('delete_timeline_event', {timelineEventId})
        events.value = events.value.filter(event => event.id !== timelineEventId)
        eventCards.value = eventCards.value.filter(item => item.timelineEventId !== timelineEventId)
        if (activeEventId.value === timelineEventId) activeEventId.value = events.value[0]?.id ?? null
    }

    async function attachCard(input: AttachTimelineEventCardInput) {
        saving.value = true
        try {
            const card = await call<TimelineEventCard>('attach_card_to_timeline_event', {input})
            eventCards.value = [card, ...eventCards.value.filter(item => item.id !== card.id && !(item.timelineEventId === card.timelineEventId && item.cardId === card.cardId))]
            return card
        } finally {
            saving.value = false
        }
    }

    async function detachCard(timelineEventId: string, cardId: string) {
        await call<boolean>('detach_card_from_timeline_event', {timelineEventId, cardId})
        eventCards.value = eventCards.value.filter(item => !(item.timelineEventId === timelineEventId && item.cardId === cardId))
    }

    async function reorderEvent(input: ReorderTimelineEventInput) {
        events.value = sortEvents(await call<TimelineEvent[]>('reorder_timeline_event', {input}))
        activeEventId.value = input.timelineEventId
        return events.value
    }

    function selectEvent(timelineEventId: string | null) {
        activeEventId.value = timelineEventId
    }

    function setFilters(cardId: string | null, locationId: string | null) {
        cardFilterId.value = cardId
        locationFilterId.value = locationId
    }

    return {
        events,
        eventCards,
        activeEventId,
        activeEvent,
        activeEventCards,
        participantsByEventId,
        cardFilterId,
        locationFilterId,
        loading,
        saving,
        loadTimeline,
        loadEventCards,
        createEvent,
        updateEvent,
        deleteEvent,
        attachCard,
        detachCard,
        reorderEvent,
        selectEvent,
        setFilters
    }
})
