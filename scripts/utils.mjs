#!/usr/bin/env node

import { rmSync } from "node:fs";
import { spawn } from "node:child_process";

function exec(argv0, argv) {
  return new Promise((resolve, reject) => {
    const proc = spawn(argv0, argv, {
      stdio: ["ignore", "inherit", "inherit"],
    });

    proc.on("error", reject);
    proc.on("close", (code, signal) => {
      if (code === 0) {
        resolve();
      } else {
        const command = [argv0, ...argv].join(" ");
        const result = signal
          ? `was terminated by ${signal}`
          : `exited with code ${code}`;
        reject(new Error(`${command} ${result}`));
      }
    });
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
