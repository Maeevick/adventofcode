import { readInput } from "./readInput.ts";
import { hello } from "./hello.ts";
import { part1 as day1part1, part2 as day1part2 } from "./day1.ts";
import { part1 as day2part1, part2 as day2part2 } from "./day2.ts";
import { part1 as day3part1, part2 as day3part2 } from "./day3.ts";
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
  console.log(`D3P1: ${day3part1(readInput("day3"))}`);
  console.log(`D3P2: ${day3part2(readInput("day3"))}`);
  console.log(`----------------`);
})();
