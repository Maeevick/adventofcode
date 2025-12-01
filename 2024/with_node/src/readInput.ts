import os from "node:os";
import fs from "node:fs";
import path from "node:path";

export function readInput(filename: string) {
  return fs
    .readFileSync(path.join("..", "inputs", `${filename}.txt`), {
      encoding: "utf8",
    })
    .split(os.EOL);
}

export function readInputWithBlankLineBreak(filename: string) {
  return fs
    .readFileSync(path.join("..", "inputs", `${filename}.txt`), {
      encoding: "utf8",
    })
    .split(`${os.EOL}${os.EOL}`)
    .map((sub) => sub.split(os.EOL));
}
