import { Language } from "./interface";



interface LanguagePickerProps  {
    language : Language,
    setLanguage: (language: Language) => void
}


export default function LanguagePicker(props: LanguagePickerProps) {
    
   return <select value={props.language} onChange={(evt) => {
       props.setLanguage(evt.target.value as Language)
   }}>
       <option value={Language.Flow}>
            {Language.Flow}
       </option>
       <option value={Language.Typescript}>
            {Language.Typescript}
       </option>
      
   </select>
}
