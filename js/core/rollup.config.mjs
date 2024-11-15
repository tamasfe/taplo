import resolve from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";
import path from "path";
import process from "process";
import { minify } from "rollup-plugin-esbuild";
import typescript from "@rollup/plugin-typescript";

export default {
  input: {
    index: "src/index.ts",
  },
  output: {
    sourcemap: false,
    name: "taploCore",
    format: "umd",
    dir: "dist",
  },
  plugins: [
    typescript(),
    commonjs(),
    resolve({
      jsnext: true,
      preferBuiltins: true,
      rootDir: path.join(process.cwd(), ".."),
    }),
    minify(),
  ],
};
