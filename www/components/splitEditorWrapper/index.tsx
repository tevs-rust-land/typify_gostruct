export default function SplitEditorWrapper({ firstEditor, secondEditor }) {
  return (
    <div className="editors__container">
      <div className="editor">{firstEditor}</div>
      <div className="editor">{secondEditor}</div>

      <style jsx>{`
        .editors__container {
          display: flex;
          flex-direction: row;
          justify-content: center;
        }
        @media screen and (max-width: 500px) {
          .editor {
            max-width: 90%;
            margin-bottom: 10px;
          }
          .editors__container {
            align-items: center;
            flex-direction: column;
          }
        }

        .editor {
          width: 500px;
          border: solid 1px;
        }
        .editor:first-child {
          margin-right: 10px;
        }
      `}</style>
    </div>
  );
}
