export default function SplitEditorWrapper({ firstEditor, secondEditor }) {
  return (
    <div className="editors__container">
      <div className="editor">{firstEditor}</div>
      <div className="editor">{secondEditor}</div>

      <style jsx>{`
        .editors__container {
          display: flex;
        }
        .editor {
          margin-right: 10px;
        }
      `}</style>
    </div>
  );
}
