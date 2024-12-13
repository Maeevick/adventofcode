type TwoColumnList = { left: number[]; right: number[] };

function separateTheTwoLists(content: string[]): TwoColumnList {
  return content.reduce(
    (acc: TwoColumnList, current: string) => {
      const [left, right] = current
        .replace(/[ ]{2,}/, " ")
        .split(" ")
        .map(Number);

      acc.left.push(left);
      acc.left.sort((a: number, b: number) => a - b);
      acc.right.push(right);
      acc.right.sort((a: number, b: number) => a - b);

      return acc;
    },
    { left: [], right: [] }
  );
}

export function part1(content: string[]) {
  const { left, right } = separateTheTwoLists(content);

  let sum = 0;
  const length = Math.max(left.length, right.length);
  for (let i = 0; i < length; i++) {
    sum += Math.abs(left[i] - right[i]);
  }

  return sum;
}

export function part2(content: string[]) {
  const { left, right } = separateTheTwoLists(content);

  let sum = 0;
  for (let i = 0; i < left.length; i++) {
    const a = left[i];
    const b = right.filter((e) => e === a).length;

    sum += a * b;
  }

  return sum;
}
