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
  console.log("left", left);
  console.log("right", right);

  let sum = 0;
  const length = Math.max(left.length, right.length);
  for (let i = 0; i < length; i++) {
    sum += Math.abs(left[i] - right[i]);
  }

  return sum;
}

export function part2() {
  return "This is day one, part two";
}
