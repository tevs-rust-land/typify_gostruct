import { Language } from "../../interfaces/language";

interface LanguagePickerProps {
  language: Language;
  setLanguage: (language: Language) => void;
}

export default function LanguagePicker(props: LanguagePickerProps) {
  return (
    <div className="select--container">
      <select
        className="select--box"
        value={props.language}
        onChange={(evt) => {
          props.setLanguage(evt.target.value as Language);
        }}
      >
        <option value={Language.Flow}>{Language.Flow}</option>
        <option value={Language.Typescript}>{Language.Typescript}</option>
      </select>

      <style jsx>{`
        .select--container {
          margin-bottom: 30px;
        }
        .select--box {
          appearance: none;
          margin: 0.6rem 0 1rem 0;
          transition: all 0.2s ease;
          cursor: pointer;
          background-color: white;
          color: grey;
          border-radius: 3px;
          background-repeat: no-repeat;
          background-position: calc(100% - 4px) 50%;
          background-size: 16px;
          padding-left: 10px;
          border: 1px solid grey;
          background-image: url("./arrow-down.svg");
          font-weight: normal;
          display: block;
          height: 36px;
          min-width: 195px;
        }
      `}</style>
    </div>
  );
}
