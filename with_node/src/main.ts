import { readInput } from "./readInput.ts";
import { hello } from "./hello.ts";
import { part1 as day1part1, part2 as day1part2 } from "./day1.ts";
import { part1 as day2part1, part2 as day2part2 } from "./day2.ts";

(function main() {
  console.log(`----------------`);
  console.log(`${hello()}`);
  console.log(`----------------`);
  console.log(`D1P1: ${day1part1(readInput("day1"))}`);
  console.log(`D1P2: ${day1part2(readInput("day1"))}`);
  console.log(`----------------`);
  console.log(`D2P1: ${day2part1(readInput("day2"))}`);
  console.log(`D2P2: ${day2part2(readInput("day2"))}`);
  console.log(`----------------`);
})();
