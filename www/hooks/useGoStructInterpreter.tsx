import { useRef, useEffect, useState } from "react";

import { Language } from "../interfaces/language";

const defaultStruct = `
type Vertex struct {
	X int
	Y int
}
`;
export default function useGoStruct() {
  const [transformer, setTransformer] = useState(null);

  const [source, setSource] = useState<string>(defaultStruct);
  const [result, setResult] = useState<string>("");
  const [targetLanguageName, setTargetLanguage] = useState<Language>(
    Language.Flow
  );
  const formatError = (err: string) => {
    return `{ "error": "${err}" }`;
  };
  useEffect(() => {
    if (!transformer) return;
    try {
      const value = transformer.transform(source, targetLanguageName);
      setResult(value);
    } catch (error) {
      setResult(formatError(String(error)));
    }
  }, [source, targetLanguageName, transformer]);

  const onChangeSource = (value: string) => {
    setSource(value);
  };
  const onChangeResults = (value: string) => {
    setResult(() => value);
  };
  useEffect(() => {
    import("typify_gostruct_wasm")
      .then((transformer) => {
        setTransformer(() => transformer);
      })
      .catch(() => {
        setResult(
          formatError("Failed to load the interpreter. Try reloading the page")
        );
      });
  }, []);

  return {
    source,
    result,
    onChangeResults,
    onChangeSource,
    targetLanguageName,
    setTargetLanguage,
  };
}
