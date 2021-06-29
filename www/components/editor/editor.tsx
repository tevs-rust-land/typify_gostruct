import { useMemo } from "react";
import SimpleEditor from "react-simple-code-editor";
import "prismjs/themes/prism.css";

import Prism from "prismjs";

import "prismjs/components/prism-typescript";
import "prismjs/components/prism-go";

import { EditorProps } from "./interface";
import { Language } from "../../interfaces/language";

export default function EditorWrapper(props: EditorProps) {
  const { value, onValueChange, language } = props;
  console.log(Prism.languages);
  const languageToHighlight = useMemo(() => {
    const languagePresets = {
      [Language.Flow]: Prism.languages.typescript,
      [Language.Typescript]: Prism.languages.typescript,
      [Language.Go]: Prism.languages.go,
    };
    const selectedLanguage = languagePresets[language];
    return selectedLanguage ? selectedLanguage : Prism.languages.typescript;
  }, []);
  return (
    <SimpleEditor
      value={value}
      onValueChange={onValueChange}
      padding={10}
      highlight={(code) => Prism.highlight(code, languageToHighlight)}
      style={{
        fontFamily: "JetBrains Mono",
        fontSize: 12,
        fontWeight: "bolder",
        minHeight: 400,
      }}
    />
  );
}
