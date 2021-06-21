import { Interpreter } from "./interface";
const GoStructInterpreter: Interpreter = async (struct: string, to: string) => {
  const { transform } = await import("typify_gostruct_wasm");
  try {
    const result = transform(struct, to);
    return result;
  } catch (err) {
    throw err;
  }
};

export default GoStructInterpreter;
