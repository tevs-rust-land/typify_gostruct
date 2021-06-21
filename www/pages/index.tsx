import React, {useEffect, useState} from "react"
import dynamic from "next/dynamic"
import LanguagePicker from "../components/languagePicker"
import { Language } from "../components/languagePicker/interface"
import GoStructInterpreter from "../interpreter/gostruct_wasm"

const Editor = dynamic(() => import("../components/editor/ace"), { ssr: false })

const IndexPage = () => {
  const [source, setSource] = useState<string>("")
  const [result, setResult] = useState<string>("")
  const [languageName, setLanguageName] = useState<Language>(Language.Flow)
  const formatError = (err: string) => {
    return `{ "error": "${err}" }`
  }
  useEffect(() => {
    GoStructInterpreter(source, languageName).then(setResult).catch((err) => {
      setResult(() => formatError(String(err)))
    })
  }, [source, languageName])

  const onSourceChange = (value: string) => {
    setSource(value)
  }
  const onChangeResults = (value: string) => {
    setResult(() => value)
  }
  return (
    <div className="container">
      <h1>Hello 👋, lets do some transformation. Ill style this later on:)</h1>
      <LanguagePicker  language={languageName}  setLanguage={setLanguageName} />
    <div className="editors__container">
      <div className="editor">

      <Editor language={Language.Go} onValueChange={onSourceChange} value={source} />
      </div>
      <div className="editor">
      <Editor language={languageName} onValueChange={onChangeResults} value={result}  />
      </div>
    </div>
    <style jsx>{`
        .container {
          width: 100%;
          margin: auto;
          max-width: 900px;
        }
        .editors__container {
          display: flex
        }
        .editor {
          margin-right: 10px
        }
      `}</style>
    </div>
  )
}

export default IndexPage
// TODO: setup ESLint
