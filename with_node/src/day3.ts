export function part1(content: string[]) {
  const matches = content.join("").matchAll(/mul\((\d{1,3}),(\d{1,3})\)/g);

  return [...matches].reduce(
    (sum, [_, a, b]) => sum + Number(a) * Number(b),
    0
  );
}

export function part2(content: string[]) {
  const matches = content
    .join("")
    .matchAll(/(don't|do)\(\)|mul\((\d{1,3}),(\d{1,3})\)/g);

  return [...matches].reduce(
    (acc, [_, instruction, a, b]) => {
      if (instruction) {
        return { sum: acc.sum, processing: instruction === "do" };
      }
      if(acc.processing && a && b) {
        return { sum: acc.sum + Number(a) * Number(b), processing: true }
      }
      return acc
    },
    { processing: true, sum: 0 }
  ).sum;
}
