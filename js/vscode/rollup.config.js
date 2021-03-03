import typescript from "rollup-plugin-typescript2";
import commonjs from "@rollup/plugin-commonjs";
import resolve from "@rollup/plugin-node-resolve";

export default {
  input: {
    server: "src/server.ts",
    extension: "src/extension.ts",
  },
  output: {
    sourcemap: false,
    format: "cjs",
    dir: "dist",
  },
  plugins: [
    typescript(),
    commonjs({ include: ["src/*.ts", "node_modules/**", "../lsp/**"] }),
    resolve({ jsnext: true, preferBuiltins: true }),
  ],
};
