import assert from "node:assert";
import test from "node:test";

import { readInput, readInputWithBlankLineBreak } from "./readInput.ts";
import { part1, part2 } from "./day5.ts";

test("day5...", async (t) => {
  await t.test("part1 says...", () => {
    assert.strictEqual(part1(readInputWithBlankLineBreak("day5sample")), 143);
  });

  await t.test("part2 says...", () => {
    assert.strictEqual(part2(readInputWithBlankLineBreak("day5sample")), -1);
  });
});
