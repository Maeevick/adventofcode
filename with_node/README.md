# Think Stupid

## Run the code

```shell
npm start
```

## Run the test suite

```shell
npm test
```

## HOW TO (_it's more for me than real guidelines for anyone else_)

- Add one file `dayX` and one test file `dayX.test.ts` in the `src` folder
- /!\ **Warning**: Make the tests async to allow the node's test-runner to run without error.

```typescript
import assert from "node:assert";
import test from "node:test";

import { part1 as dayXpart1, part2 as dayXpart2 } from "./dayX.ts";

test("dayX...", async (t) => {
  await t.test("part1 says...", () => {
    assert.strictEqual(dayXpart1(), "This is day XXX, part one");
  });

  await t.test("part2 says...", () => {
    assert.strictEqual(dayXpart2(), "This is day XXX, part two");
  });
});
```
