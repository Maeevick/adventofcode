import { readInput } from "./readInput.ts";
// import { hello } from "./hello.ts";
import { part1 as day1part1, part2 as day1part2 } from "./day1.ts";

(function main() {
  // console.log(`\n${hello()}\n\n`);
  console.log(`\n${day1part1(readInput("day1part1"))}\n\n`);
  // console.log(`\n${day1part2()}\n\n`);
})();
