import rust from "@wasm-tool/rollup-plugin-rust";
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
    name: "taplo",
    format: "umd",
    dir: "dist",
  },
  plugins: [
    typescript(),
    rust({
      debug: process.env["RELEASE"] !== "true",
      nodejs: true,
      inlineWasm: true,
      cargoArgs: ["--frozen"],
      verbose: process.env["VERBOSE"] === "true",
    }),
    commonjs(),
    resolve({
      jsnext: true,
      preferBuiltins: true,
      rootDir: path.join(process.cwd(), ".."),
    }),
    minify(),
  ],
};
