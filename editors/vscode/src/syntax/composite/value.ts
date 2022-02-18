import { tableInline } from "./table";
import { array } from ".";
import { string, datetime, boolean, number } from "../literal";

export const value = {
  patterns: (<Array<any>>[]).concat(
    string,
    datetime,
    boolean,
    number,
    array,
    tableInline
  ),
};
