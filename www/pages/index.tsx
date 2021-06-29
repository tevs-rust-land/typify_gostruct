import React from "react";
import dynamic from "next/dynamic";
import LanguagePicker from "../components/languagePicker";
import Container from "../components/container";
import SplitEditorWrapper from "../components/splitEditorWrapper";
import { Language } from "../interfaces/language";
import useGoStructInterpreter from "../hooks/useGoStructInterpreter";
import CopyResults from "../components/copyResults";
import ActionsWrapper from "../components/actionsWrapper";
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
      <h1>Hello 👋, lets do some GO transformation. 🚀</h1>

      <ActionsWrapper
        firstAction={
          <LanguagePicker
            language={targetLanguageName}
            setLanguage={setTargetLanguage}
          />
        }
        secondAction={<CopyResults result={result} />}
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
