import rust from "@wasm-tool/rollup-plugin-rust";
// import typescript from "@rollup/plugin-typescript";
import typescript from 'rollup-plugin-typescript2';
export default {
  input: {
    cli: "src/cli.ts",
  },
  output: {
    sourcemap: false,
    format: "cjs",
    dir: "dist",
  },
  plugins: [
    rust({
      debug: true,
      nodejs: true,
      inlineWasm: true,
      cargoArgs: ["--no-default-features", "--features=internal-node"],
    }),
    typescript(),
  ],
};
