import { useRef, useEffect, useState } from "react";

import { Language } from "../interfaces/language";
export default function useGoStruct() {
  const [transformer, setTransformer] = useState(null);

  const [source, setSource] = useState<string>("");
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
      const value = transformer(source, targetLanguageName);
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
      .then(({ transform }) => {
        setTransformer(() => transform);
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
