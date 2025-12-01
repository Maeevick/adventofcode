export function part1(content: string[]) {
  const DIRECTIONS = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
  ];

  const h = content.length;
  const w = content[0].length;

  const checkOutOfBounds = (
    horizontalIndex: number,
    verticalIndex: number,
    horizontalDirection: number,
    verticalDirection: number
  ): boolean => {
    return (
      horizontalIndex + 3 * horizontalDirection >= content.length ||
      horizontalIndex + 3 * horizontalDirection < 0 ||
      verticalIndex + 3 * verticalDirection >= content[0].length ||
      verticalIndex + 3 * verticalDirection < 0
    );
  };

  const checkWord = (
    horizontalIndex: number,
    verticalIndex: number,
    horizontalDirection: number,
    verticalDirection: number
  ): boolean => {
    const isOutOfBounds = checkOutOfBounds(
      horizontalIndex,
      verticalIndex,
      horizontalDirection,
      verticalDirection
    );
    if (isOutOfBounds) return false;

    return (
      content[horizontalIndex][verticalIndex] === "X" &&
      content[horizontalIndex + horizontalDirection][
        verticalIndex + verticalDirection
      ] === "M" &&
      content[horizontalIndex + 2 * horizontalDirection][
        verticalIndex + 2 * verticalDirection
      ] === "A" &&
      content[horizontalIndex + 3 * horizontalDirection][
        verticalIndex + 3 * verticalDirection
      ] === "S"
    );
  };

  let count = 0;

  for (let horizontalIndex = 0; horizontalIndex < h; horizontalIndex++) {
    for (let verticalIndex = 0; verticalIndex < w; verticalIndex++) {
      if (content[horizontalIndex][verticalIndex] === "X") {
        for (const [horizontalDirection, verticalDirection] of DIRECTIONS) {
          if (
            checkWord(
              horizontalIndex,
              verticalIndex,
              horizontalDirection,
              verticalDirection
            )
          ) {
            count++;
          }
        }
      }
    }
  }

  return count;
}

export function part2(content: string[]) {
  let count = 0;

  for (
    let horizontalIndex = 1, h = content.length - 1;
    horizontalIndex < h;
    horizontalIndex++
  ) {
    for (
      let verticalIndex = 1, w = content[0].length - 1;
      verticalIndex < w;
      verticalIndex++
    ) {
      if (content[horizontalIndex][verticalIndex] === "A") {
        const toplleft = content[horizontalIndex - 1][verticalIndex - 1];
        const topright = content[horizontalIndex - 1][verticalIndex + 1];
        const bottomleft = content[horizontalIndex + 1][verticalIndex - 1];
        const bottomrright = content[horizontalIndex + 1][verticalIndex + 1];

        const tlMblMtrSbrS =
          toplleft === "M" &&
          bottomleft === "M" &&
          topright === "S" &&
          bottomrright === "S";
        const tlSblStrMbrM =
          toplleft === "S" &&
          bottomleft === "S" &&
          topright === "M" &&
          bottomrright === "M";
        const tlMblStrMbrS =
          toplleft === "M" &&
          bottomleft === "S" &&
          topright === "M" &&
          bottomrright === "S";
        const tlSblMtrSbrM =
          toplleft === "S" &&
          bottomleft === "M" &&
          topright === "S" &&
          bottomrright === "M";

        if (tlMblMtrSbrS || tlSblStrMbrM || tlMblStrMbrS || tlSblMtrSbrM) {
          count++;
        }
      }
    }
  }
  return count;
}
