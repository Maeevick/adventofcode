import assert from "node:assert";
import test from "node:test";

import { readInput } from "./readInput.ts";
import { part1, part2 } from "./day2.ts";

test("day2...", async (t) => {
  await t.test("part1 says...", () => {
    assert.strictEqual(part1(readInput("day2sample")), 2);
  });

  await t.test("part2 says...", () => {
    assert.strictEqual(part2(readInput("day2sample")), 4);
  });
});
