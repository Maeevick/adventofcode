import assert from "node:assert";
import test from "node:test";

import { readInput } from "./readInput.ts";
import { part1, part2 } from "./day4.ts";

test("day4...", async (t) => {
  await t.test("part1 says...", () => {
    assert.strictEqual(part1(readInput("day4sample")), 18);
  });

  await t.test("part2 says...", () => {
    assert.strictEqual(part2(readInput("day4sample")), 9);
  });
});
