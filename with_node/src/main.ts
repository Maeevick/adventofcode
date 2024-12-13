import { readInput } from "./readInput.ts";
import { hello } from "./hello.ts";
import { part1 as day1part1, part2 as day1part2 } from "./day1.ts";

(function main() {
  console.log(`----------------`);
  console.log(`${hello()}`);
  console.log(`----------------`);
  console.log(`D1P1: ${day1part1(readInput("day1part1"))}`);
  console.log(`D1P2: ${day1part2(readInput("day1part1"))}`);
  console.log(`----------------`);
})();
