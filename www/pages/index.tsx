import React from "react";
import dynamic from "next/dynamic";
import LanguagePicker from "../components/languagePicker";
import Container from "../components/container";
import SplitEditorWrapper from "../components/splitEditorWrapper";
import { Language } from "../interfaces/language";
import useGoStructInterpreter from "../hooks/useGoStructInterpreter";
// import Editor from "../components/editor/editor"
const Editor = dynamic(() => import("../components/editor/editor"), {
  ssr: false,
});

const IndexPage = () => {
  const {
    setTargetLanguage,
    targetLanguageName,
    onChangeResults,
    onChangeSource,
    source,
    result,
  } = useGoStructInterpreter();
  return (
    <Container>
      <h1>Hello 👋, lets do some transformation.</h1>
      <LanguagePicker
        language={targetLanguageName}
        setLanguage={setTargetLanguage}
      />
      <SplitEditorWrapper
        firstEditor={
          <Editor
            language={Language.Go}
            onValueChange={onChangeSource}
            value={source}
          />
        }
        secondEditor={
          <Editor
            language={targetLanguageName}
            onValueChange={onChangeResults}
            value={result}
          />
        }
      />
    </Container>
  );
};

export default IndexPage;
// TODO: setup ESLint
