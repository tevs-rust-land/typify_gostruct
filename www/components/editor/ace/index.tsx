import React, { useMemo } from "react";
import AceEditor from "react-ace";

import "ace-builds/src-noconflict/mode-golang";
import "ace-builds/src-noconflict/mode-javascript";

import "ace-builds/src-noconflict/theme-github";

import { EditorProps } from "../interface";
import { Language } from "../../languagePicker/interface";


const Editor = (props: EditorProps) => {
  const { value, onValueChange, language } = props;
  
  return (
    <div>
        <AceEditor
           value={value}
           mode={"java"}
           theme="dawn"
           onChange={onValueChange}
           name="UNIQUE_ID_OF_DIV"
           editorProps={{ $blockScrolling: true }}
         />
    </div>
  );
};

export default Editor;
