import {Mark, mergeAttributes} from '@tiptap/core'

export interface SettingReferenceAttrs {
    cardId: string
    cardType?: string | null
    displayName?: string | null
}

declare module '@tiptap/core' {
    interface Commands<ReturnType> {
        settingReference: {
            setSettingReference: (attrs: SettingReferenceAttrs) => ReturnType
            unsetSettingReference: () => ReturnType
        }
    }
}

export const SettingReference = Mark.create({
    name: 'settingReference',

    inclusive: false,

    addAttributes() {
        return {
            cardId: {
                default: null,
                parseHTML: element => element.getAttribute('data-card-id'),
                renderHTML: attributes => attributes.cardId ? {'data-card-id': attributes.cardId} : {}
            },
            cardType: {
                default: null,
                parseHTML: element => element.getAttribute('data-card-type'),
                renderHTML: attributes => attributes.cardType ? {'data-card-type': attributes.cardType} : {}
            },
            displayName: {
                default: null,
                parseHTML: element => element.getAttribute('data-display-name'),
                renderHTML: attributes => attributes.displayName ? {'data-display-name': attributes.displayName} : {}
            }
        }
    },

    parseHTML() {
        return [
            {
                tag: 'span[data-setting-reference]'
            }
        ]
    },

    renderHTML({HTMLAttributes}) {
        return [
            'span',
            mergeAttributes(HTMLAttributes, {
                'data-setting-reference': 'true',
                class: 'setting-reference'
            }),
            0
        ]
    },

    addCommands() {
        return {
            setSettingReference: attributes => ({commands}) => {
                return commands.setMark(this.name, attributes)
            },
            unsetSettingReference: () => ({commands}) => {
                return commands.unsetMark(this.name)
            }
        }
    }
})
