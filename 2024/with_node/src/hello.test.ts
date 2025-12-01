import assert from "node:assert";
import test from "node:test";

import { hello } from "./hello.ts";

test("setup is ok", async (t) => {
  await t.test("says hello", () => {
    assert.strictEqual(
      hello(),
      "Hello, advent of code with typescript and node!"
    );
  });
});
