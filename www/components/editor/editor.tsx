import { useMemo } from "react";
import SimpleEditor from "react-simple-code-editor";
import { highlight, languages } from "prismjs/components/prism-core";
import "prismjs/components/prism-clike";
import "prismjs/components/prism-javascript";
import "prismjs/components/prism-go";

import { EditorProps } from "./interface";
import { Language } from "../../interfaces/language";

export default function EditorWrapper(props: EditorProps) {
  const { value, onValueChange, language } = props;

  const languageToHighlight = useMemo(() => {
    const languagePresets = {
      [Language.Flow]: languages.typescript,
      [Language.Typescript]: languages.typescript,
      [Language.Go]: languages.go,
    };
    const selectedLanguage = languagePresets[language];
    return selectedLanguage ? selectedLanguage : languages.typescript;
  }, []);
  return (
    <SimpleEditor
      value={value}
      onValueChange={onValueChange}
      padding={10}
      highlight={(code) => highlight(code, languageToHighlight)}
      style={{
        fontFamily: "JetBrains Mono",
        fontSize: 12,
      }}
    />
  );
}
