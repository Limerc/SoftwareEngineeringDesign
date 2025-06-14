import { monaco } from '@/utils/customMonaco'
export function newEditor(dom: HTMLDivElement): monaco.editor.IStandaloneCodeEditor {
  const editor = monaco.editor.create(dom, {
    language: 'rust',
    theme: 'vs',
    value: 'fn main() {\n\tprintln!("Hello World!");\n}\n',
    cursorBlinking: 'smooth',
    cursorSmoothCaretAnimation: 'on',
    contextmenu: false,
    minimap: {
      enabled: false
    }
  })
  editor.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.KeyS, () => {
    editor?.getAction('editor.action.formatDocument')?.run()
  })
  return editor
}
