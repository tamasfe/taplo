#!/usr/bin/env node

import { exec, unlink } from "../../../scripts/utils.mjs";

unlink("./dist");
exec("yarn", ["build:syntax"]);
exec("yarn", ["build:node"]);
exec("yarn", ["build:browser-extension"]);
exec("yarn", ["build:browser-server"]);
