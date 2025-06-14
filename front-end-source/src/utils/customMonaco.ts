import 'monaco-editor/esm/vs/editor/editor.all.js';
import 'monaco-editor/esm/vs/basic-languages/rust/rust.contribution'
import * as monaco from 'monaco-editor/esm/vs/editor/editor.api';
import prettier from 'prettier'
import rustPlugin from "prettier-plugin-rust"

monaco.languages.registerDocumentFormattingEditProvider('rust', {
  provideDocumentFormattingEdits: model => {
    const text = model.getValue()
    const formatted = prettier.format(text, {
      parser: 'jinx-rust',
      plugins: [rustPlugin],
    })
    return [{ range: model.getFullModelRange(), text: formatted }]
  }
})

monaco.languages.registerCompletionItemProvider('rust', {
  provideCompletionItems: function (model, position) {
    const word = model.getWordUntilPosition(position);
    const range = {
      startLineNumber: position.lineNumber,
      endLineNumber: position.lineNumber,
      startColumn: word.startColumn,
      endColumn: word.endColumn
    };
    return {
      suggestions: [
        {
          label: 'println!()',
          kind: monaco.languages.CompletionItemKind.Function,
          insertText: 'println!("$0");',
          insertTextRules: monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
          range: range
        },
        {
          label: 'read_line()',
          kind: monaco.languages.CompletionItemKind.Function,
          insertText: 'read_line(&mut $0).unwrap();',
          insertTextRules: monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
          range: range
        }
      ]
    };
  }
});
export { monaco }
