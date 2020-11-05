import rust from "@wasm-tool/rollup-plugin-rust";
import typescript from "rollup-plugin-typescript2";
import { terser } from "rollup-plugin-terser";

export default {
  input: {
    index: "src/index.ts",
  },
  output: {
    sourcemap: false,
    name: "taplo",
    format: "umd",
    dir: "dist",
  },
  plugins: [
    rust({
      debug: false,
      nodejs: true,
      inlineWasm: process.env["SEPARATE_WASM"] !== "true",
      cargoArgs: ["--features=_internal_nodejs"],
      
    }),
    typescript(),
    terser()
  ],
};
