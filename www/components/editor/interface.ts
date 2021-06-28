import { Language } from "../../interfaces/language";
export type EditorProps = {
  value: string;
  onValueChange: (value: string) => void;
  language: Language;
};
