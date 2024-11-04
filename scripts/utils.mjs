#!/usr/bin/env node

import { rmSync } from "node:fs";
import { spawn } from "node:child_process";

function exec(argv0, argv) {
  const proc = spawn(argv0, argv, {
    stdio: ["ignore", "inherit", "inherit"],
    shell: true,
  });

  proc.on("close", (code) => {
    if (code != 0) {
      console.error(`exit code: ${code}`);
    }
  });
}

function unlink(path) {
  try {
    rmSync(path, { recursive: true, force: true });
  } catch (e) {
    switch (e.code) {
      case "ENOENT":
        break;
      default:
        console.error(e);
        break;
    }
  }
}

export { exec, unlink };
