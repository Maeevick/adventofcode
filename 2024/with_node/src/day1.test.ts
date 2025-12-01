import assert from "node:assert";
import test from "node:test";

import { readInput } from "./readInput.ts";
import { part1 as day1part1, part2 as day1part2 } from "./day1.ts";

test("day1...", async (t) => {
  await t.test("part1 says...", () => {
    assert.strictEqual(day1part1(readInput("day1sample")), 11);
  });

  await t.test("part2 says...", () => {
    assert.strictEqual(day1part2(readInput("day1sample")), 31);
  });
});
