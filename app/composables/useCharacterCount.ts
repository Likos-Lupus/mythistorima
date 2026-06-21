export function useCharacterCount() {
    function countCharacters(text: string): number {
        return text.replace(/\s+/g, '').length
    }

    return {countCharacters}
}
