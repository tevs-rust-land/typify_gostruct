import { Language } from "../languagePicker/interface";
export type EditorProps = {
  value: string;
  onValueChange: (value: string) => void;
  language: Language;
};
