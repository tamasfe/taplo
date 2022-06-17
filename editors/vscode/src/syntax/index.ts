import * as path from "path";
import { writeFileSync } from "fs";
import { comment, commentDirective } from "./comment";
import { table, entryBegin } from "./composite";
import { value } from "./composite/value";

const syntax = {
  version: "1.0.0",
  scopeName: "source.toml",
  uuid: "8b4e5008-c50d-11ea-a91b-54ee75aeeb97",
  information_for_contributors: [
    "Originally was maintained by aster (galaster@foxmail.com). This notice is only kept here for the record, please don't send e-mails about bugs and other issues.",
  ],
  patterns: [
    {
      include: "#commentDirective",
    },
    {
      include: "#comment",
    },
    {
      include: "#table",
    },
    {
      include: "#entryBegin",
    },
    {
      include: "#value",
    },
  ],
  repository: {
    comment,
    commentDirective,
    table,
    entryBegin,
    value,
  },
};

writeFileSync(
  path.resolve(__dirname, path.join("..", "..", "toml.tmLanguage.json")),
  JSON.stringify(syntax, null, 2)
);
