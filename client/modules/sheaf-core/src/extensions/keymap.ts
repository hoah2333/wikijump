import { completionKeymap } from "@codemirror/autocomplete"
import { closeBracketsKeymap } from "@codemirror/closebrackets"
import { copyLineDown, defaultKeymap, defaultTabBinding } from "@codemirror/commands"
import { commentKeymap } from "@codemirror/comment"
import { foldKeymap } from "@codemirror/fold"
import { historyKeymap, redo } from "@codemirror/history"
import { nextDiagnostic, openLintPanel } from "@codemirror/lint"
import { searchKeymap } from "@codemirror/search"
import { KeyBinding, keymap } from "@codemirror/view"

/** Additional key bindings for the editor. */
const KEY_MAP: KeyBinding[] = [
  { key: "Mod-l", run: openLintPanel, preventDefault: true },
  { key: "F8", run: nextDiagnostic, preventDefault: true },
  { key: "Mod-Shift-z", run: redo, preventDefault: true },
  { key: "Mod-d", run: copyLineDown, preventDefault: true }
]

/** Returns an extension for Sheaf's full keybinding set. */
export function getSheafKeymap() {
  return keymap.of([
    ...defaultKeymap,
    ...closeBracketsKeymap,
    ...searchKeymap,
    ...historyKeymap,
    ...foldKeymap,
    ...commentKeymap,
    ...completionKeymap,
    ...KEY_MAP,
    defaultTabBinding
  ])
}