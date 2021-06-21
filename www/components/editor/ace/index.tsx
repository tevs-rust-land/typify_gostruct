import React, { useMemo } from "react";
import AceEditor from "react-ace";

import ace from "ace-builds"


import { EditorProps } from "../interface";
import { Language } from "../../languagePicker/interface";

const CDN = "https://cdn.jsdelivr.net/npm/ace-builds@1.4.12/src-noconflict"

ace.config.set("basePath", CDN)
ace.config.set('modePath', CDN);
ace.config.set('themePath', CDN);
ace.config.set('workerPath', CDN);

const Editor = (props: EditorProps) => {
  const { value, onValueChange, language } = props;
  const mode = useMemo(() => {
    const modes = {
      [Language.Flow]: "typescript",
      [Language.Typescript]: "typescript",
      [Language.Go]: "golang"
    }
    const selectedMode = modes[language];
    return selectedMode ? selectedMode : "typescript"
  }, [])
  return (
    <div>
        <AceEditor
           value={value}
           mode={mode}
           theme="github"
           onChange={onValueChange}
           editorProps={{ $blockScrolling: true, width: 400 }}
         />
    </div>
  );
};

export default Editor;
