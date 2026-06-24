const MODIFIER_ORDER = ['Mod', 'Ctrl', 'Meta', 'Alt', 'Shift'] as const

const KEY_ALIASES: Record<string, string> = {
    ' ': 'Space',
    Escape: 'Escape',
    Esc: 'Escape',
    Enter: 'Enter',
    Tab: 'Tab',
    Backspace: 'Backspace',
    Delete: 'Delete',
    ArrowUp: 'ArrowUp',
    ArrowDown: 'ArrowDown',
    ArrowLeft: 'ArrowLeft',
    ArrowRight: 'ArrowRight',
    ',': ',',
    '.': '.',
    '/': '/',
    ';': ';',
    "'": "'",
    '[': '[',
    ']': ']',
    '\\': '\\',
    '-': '-',
    '=': '=',
    '`': '`'
}

export function shortcutFromKeyboardEvent(event: KeyboardEvent): string | null {
    const key = normalizeEventKey(event)
    if (!key || ['Control', 'Meta', 'Alt', 'Shift'].includes(key)) return null

    const modifiers: string[] = []
    if (event.ctrlKey || event.metaKey) modifiers.push('Mod')
    if (event.altKey) modifiers.push('Alt')
    if (event.shiftKey) modifiers.push('Shift')

    return normalizeShortcut([...modifiers, key].join('+'))
}

export function normalizeShortcut(value: string | null | undefined): string {
    if (!value) return ''
    const rawTokens = value
        .split('+')
        .map(token => token.trim())
        .filter(Boolean)

    const modifierSet = new Set<string>()
    let key = ''
    for (const token of rawTokens) {
        const normalized = normalizeToken(token)
        if (MODIFIER_ORDER.includes(normalized as typeof MODIFIER_ORDER[number])) {
            modifierSet.add(normalized)
        } else {
            key = normalized
        }
    }

    if (!key) return ''
    const modifiers = MODIFIER_ORDER.filter(token => modifierSet.has(token))
    return [...modifiers, key].join('+')
}

export function matchesShortcut(event: KeyboardEvent, shortcut: string): boolean {
    const normalized = normalizeShortcut(shortcut)
    if (!normalized) return false

    const tokens = normalized.split('+')
    const key = tokens[tokens.length - 1]
    const modifiers = new Set(tokens.slice(0, -1))
    const eventKey = normalizeEventKey(event)

    if (eventKey !== key) return false

    const requiresMod = modifiers.has('Mod')
    if (requiresMod && !(event.ctrlKey || event.metaKey)) return false
    if (!requiresMod) {
        if (modifiers.has('Ctrl') !== event.ctrlKey) return false
        if (modifiers.has('Meta') !== event.metaKey) return false
    }
    if (modifiers.has('Alt') !== event.altKey) return false
    if (modifiers.has('Shift') !== event.shiftKey) return false

    if (!requiresMod && !modifiers.has('Ctrl') && event.ctrlKey) return false
    if (!requiresMod && !modifiers.has('Meta') && event.metaKey) return false
    return true
}

export function formatShortcut(shortcut: string, compact = false): string {
    const normalized = normalizeShortcut(shortcut)
    if (!normalized) return '未设置'
    const isMac = typeof navigator !== 'undefined'
        && /Mac|iPhone|iPad/.test(navigator.platform)

    const labels: Record<string, string> = compact && isMac
        ? {Mod: '⌘', Ctrl: '⌃', Meta: '⌘', Alt: '⌥', Shift: '⇧', Enter: '↵', Escape: 'Esc', Space: 'Space'}
        : {
            Mod: isMac ? '⌘' : 'Ctrl',
            Ctrl: 'Ctrl',
            Meta: 'Meta',
            Alt: isMac ? '⌥' : 'Alt',
            Shift: isMac ? '⇧' : 'Shift'
        }

    return normalized
        .split('+')
        .map(token => labels[token] ?? displayKey(token))
        .join(compact && isMac ? '' : '+')
}

export function isEditableShortcutTarget(target: EventTarget | null): boolean {
    if (!(target instanceof HTMLElement)) return false
    return Boolean(
        target.closest('input, textarea, select, [contenteditable="true"], .ProseMirror')
    )
}

function normalizeEventKey(event: KeyboardEvent): string {
    const key = event.key.length === 1 ? event.key : (KEY_ALIASES[event.key] ?? event.key)
    return normalizeToken(key)
}

function normalizeToken(token: string): string {
    const value = token.trim()
    const lower = value.toLowerCase()
    if (['mod', 'cmdorctrl', 'commandorcontrol'].includes(lower)) return 'Mod'
    if (['ctrl', 'control'].includes(lower)) return 'Ctrl'
    if (['cmd', 'command', 'meta'].includes(lower)) return 'Meta'
    if (['option', 'alt'].includes(lower)) return 'Alt'
    if (lower === 'shift') return 'Shift'
    if (lower === 'spacebar' || lower === 'space') return 'Space'
    if (value.length === 1 && /[a-z]/i.test(value)) return value.toUpperCase()
    if (value.length === 1 && /[0-9]/.test(value)) return value
    return KEY_ALIASES[value] ?? value
}

function displayKey(key: string): string {
    const arrows: Record<string, string> = {
        ArrowUp: '↑',
        ArrowDown: '↓',
        ArrowLeft: '←',
        ArrowRight: '→'
    }
    return arrows[key] ?? key
}
