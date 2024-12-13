import assert from "node:assert";
import test from "node:test";

import { part1 as day1part1, part2 as day1part2 } from "./day1.ts";

test("day1...", async (t) => {
  await t.test("part1 says...", () => {
    assert.strictEqual(day1part1(), "This is day one, part one");
  });

  await t.test("part2 says...", () => {
    assert.strictEqual(day1part2(), "This is day one, part two");
  });
});
