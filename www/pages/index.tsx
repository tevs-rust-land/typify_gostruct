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
  useEffect(() => {
    GoStructInterpreter(source, languageName).then((data) => {

      console.log(data)
    }).catch((err) => {
      console.log(err)
      setResult(String(err))
    })
  }, [source, languageName])

  const onSourceChange = (value: string) => {
    console.log(value)
    setSource(value)
  }
  return (
    <div>
      <h1>Hello 👋, lets do some transformation,</h1>
      <LanguagePicker  language={languageName}  setLanguage={setLanguageName} />

      <Editor language={Language.Go} onValueChange={onSourceChange} value={source} />
    </div>
  )
}

export default IndexPage
