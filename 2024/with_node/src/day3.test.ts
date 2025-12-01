import assert from "node:assert";
import test from "node:test";

import { readInput } from "./readInput.ts";
import { part1, part2 } from "./day3.ts";

test("day3...", async (t) => {
  await t.test("part1 says...", () => {
    assert.strictEqual(part1(readInput("day3part1sample")), 161);
  });

  await t.test("part2 says...", () => {
    assert.strictEqual(part2(readInput("day3part2sample")), 48);
  });
});
