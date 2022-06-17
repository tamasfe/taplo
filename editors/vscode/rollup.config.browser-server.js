import commonjs from "@rollup/plugin-commonjs";
import resolve from "@rollup/plugin-node-resolve";
import path from "node:path";
import esbuild from "rollup-plugin-esbuild";
import replace from "@rollup/plugin-replace";

const onwarn = (warning, rollupWarn) => {
  const ignoredWarnings = [
    {
      ignoredCode: "CIRCULAR_DEPENDENCY",
      ignoredPath: "node_modules/semver",
    },
  ];

  // only show warning when code and path don't match
  // anything in above list of ignored warnings
  if (
    !ignoredWarnings.some(
      ({ ignoredCode, ignoredPath }) =>
        warning.code === ignoredCode &&
        warning.importer.includes(path.normalize(ignoredPath))
    )
  ) {
    rollupWarn(warning);
  }
};

/** @type {import('rollup').RollupOptions} */
const options = {
  onwarn,
  input: {
    "server-worker": "src/server-worker.ts",
  },
  output: {
    sourcemap: false,
    format: "commonjs",
    dir: "dist",
    chunkFileNames: "[name].js",
  },
  external: ["vscode"],
  preserveEntrySignatures: true,
  treeshake: "smallest",
  plugins: [
    replace({
      preventAssignment: true,
      "import.meta.env.BROWSER": "true",
    }),
    esbuild({ minify: true, logLevel: "error" }),
    commonjs({
      ignore: ["url"],
    }),
    resolve({
      preferBuiltins: true,
      browser: true,
    }),
  ],
};

export default options;
