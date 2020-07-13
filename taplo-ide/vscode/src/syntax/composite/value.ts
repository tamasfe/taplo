import { tableInline } from "./table";
import { array } from ".";
import { string, datetime, boolean, number } from "../literal";

export const value = {
  patterns: (<Array<any>>[]).concat(
    tableInline,
    array,
    string,
    datetime,
    boolean,
    number
  ),
};
