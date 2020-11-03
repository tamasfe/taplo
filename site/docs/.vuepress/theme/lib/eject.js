"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
Object.defineProperty(exports, "__esModule", { value: true });
const path_1 = require("path");
const fs_extra_1 = require("fs-extra");
const chalk = require("chalk");
const EXCLUDED_FILES = [
    "__tests__",
    ".npmignore",
    "test",
    "LICENSE",
    "package.json",
    "node_modules",
    "README.md",
    "readme.md",
];
exports.default = (dir) => __awaiter(void 0, void 0, void 0, function* () {
    try {
        const sourceDir = path_1.resolve(__dirname, "../");
        const targetDir = path_1.resolve(path_1.resolve(dir), ".vuepress/theme");
        yield fs_extra_1.copy(sourceDir, targetDir, {
            filter: (src) => {
                return !EXCLUDED_FILES.includes(path_1.relative(sourceDir, src));
            },
        });
        console.log(`Copied vuepress-theme-hope into ${chalk.cyan(targetDir)}.\n`);
    }
    catch (err) {
        console.error(chalk.red(err.stack || ""));
        process.exitCode = 1;
    }
});
