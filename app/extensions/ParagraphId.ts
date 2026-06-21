import Paragraph from '@tiptap/extension-paragraph'
import {Plugin, PluginKey} from '@tiptap/pm/state'
import {createParagraphId} from '~/utils/tiptapDocument'

export const ParagraphId = Paragraph.extend({
    addAttributes() {
        return {
            ...this.parent?.(),
            pid: {
                default: null,
                parseHTML: element => element.getAttribute('data-pid'),
                renderHTML: attributes => {
                    if (!attributes.pid) return {}
                    return {
                        'data-pid': attributes.pid
                    }
                }
            }
        }
    },

    addProseMirrorPlugins() {
        return [
            new Plugin({
                key: new PluginKey('paragraphId'),
                appendTransaction: (_transactions, _oldState, newState) => {
                    let transaction = newState.tr
                    let changed = false

                    newState.doc.descendants((node, position) => {
                        if (node.type.name !== 'paragraph') return true
                        if (typeof node.attrs.pid === 'string' && node.attrs.pid.length > 0) return true

                        transaction = transaction.setNodeMarkup(position, undefined, {
                            ...node.attrs,
                            pid: createParagraphId()
                        })
                        changed = true
                        return true
                    })

                    return changed ? transaction : null
                }
            })
        ]
    }
})
